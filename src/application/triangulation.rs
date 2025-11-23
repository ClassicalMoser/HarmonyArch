/// Triangulation module for converting polygons into renderable triangles
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::{Polygon, Segment, Vertex};

/// Triangulated face data for rendering
#[derive(Debug)]
pub struct TriangulatedFace {
    /// The vertices of the triangulated face
    pub vertices: Vec<Vec3>,
    /// The normal vector of the face
    pub normal: Vec3,
    /// The winding order for proper face orientation
    #[allow(dead_code)]
    pub winding_order: WindingOrder,
}

/// Winding order for face orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindingOrder {
    /// Clockwise winding order
    Clockwise,
    /// Counter-clockwise winding order
    CounterClockwise,
}

/// Triangulate a polygon for rendering, ensuring proper winding order
/// This function takes a polygon (defined by segments) and converts it into triangulated faces
/// that can be rendered by the GPU. It handles:
/// 1. Building a vertex connectivity graph from segments
/// 2. Finding the circular path through connected vertices
/// 3. Calculating proper face normals
/// 4. Determining correct winding order for outward-facing normals
/// 5. Creating triangles with consistent orientation
pub(crate) fn triangulate_polygon_for_rendering(
    polygon: &Polygon,
    segments: &HashMap<Uuid, Segment>,
    vertices: &HashMap<Uuid, Vertex>,
) -> Vec<TriangulatedFace> {
    // Step 1: Build vertex connectivity graph from segments
    // Each vertex maps to a list of vertices it connects to
    let mut vertex_connections: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    let mut vertex_counts: HashMap<Uuid, usize> = HashMap::new();

    for segment_id in &polygon.segments {
        let segment = &segments[segment_id];

        // Count vertex occurrences (each should appear exactly twice in a valid polygon)
        *vertex_counts.entry(segment.vertices[0]).or_insert(0) += 1;
        *vertex_counts.entry(segment.vertices[1]).or_insert(0) += 1;

        // Build bidirectional connectivity graph
        vertex_connections
            .entry(segment.vertices[0])
            .or_insert_with(Vec::new)
            .push(segment.vertices[1]);
        vertex_connections
            .entry(segment.vertices[1])
            .or_insert_with(Vec::new)
            .push(segment.vertices[0]);
    }

    // Check if each vertex appears exactly twice (start and end of segments)
    let valid_loop = vertex_counts.values().all(|&count| count == 2);
    if !valid_loop {
        return vec![];
    }

    // Step 2: Find the circular path through the vertex connectivity graph
    // This approach is agnostic to segment ordering
    let mut ordered_vertices = Vec::new();
    let mut visited_vertices = HashSet::new();
    let mut current_vertex_id = None;

    // Start with any vertex that has exactly two connections
    for (vertex_id, connections) in &vertex_connections {
        if connections.len() == 2 {
            current_vertex_id = Some(*vertex_id);
            break;
        }
    }

    if current_vertex_id.is_none() {
        return vec![];
    }

    // Follow the circular path through connected vertices
    // This builds the proper vertex order regardless of segment storage order
    while visited_vertices.len() < vertex_counts.len() {
        let vertex_id = current_vertex_id.unwrap();
        let vertex_pos = &vertices[&vertex_id].position;
        let vertex_vec3 = Vec3::new(vertex_pos.x, vertex_pos.y, vertex_pos.z);

        ordered_vertices.push(vertex_vec3);
        visited_vertices.insert(vertex_id);

        // Find the next unvisited vertex connected to current vertex
        let connections = &vertex_connections[&vertex_id];
        let next_vertex_id = connections
            .iter()
            .find(|&&id| !visited_vertices.contains(&id));

        if let Some(&next_id) = next_vertex_id {
            current_vertex_id = Some(next_id);
        } else {
            // If we can't find a next vertex, we've completed the loop
            break;
        }
    }

    // Validate that we got the right number of vertices
    if ordered_vertices.len() != vertex_counts.len() {
        return vec![];
    }

    // Step 3: Calculate centers and determine normal direction
    // We need to know which way the face normal should point (outward from the solid)
    // Calculate solid center (average of all solid vertices)
    // This gives us a reference point to determine "outward" direction
    let mut solid_center = Vec3::ZERO;
    let mut solid_vertex_count = 0;
    for vertex in vertices.values() {
        solid_center += Vec3::new(vertex.position.x, vertex.position.y, vertex.position.z);
        solid_vertex_count += 1;
    }
    solid_center /= solid_vertex_count as f32;

    // Calculate polygon center (average of polygon vertices)
    let polygon_center =
        ordered_vertices.iter().fold(Vec3::ZERO, |acc, v| acc + *v) / ordered_vertices.len() as f32;

    // Vector from solid center to polygon center
    // This tells us which side of the face is "outward"
    let solid_to_polygon = polygon_center - solid_center;

    // Step 4: Use solid-to-face vector to determine correct winding order
    // The solid-to-face vector tells us which side is "outward"
    // We need to ensure vertices are ordered so the face normal points outward
    // Calculate the face normal using the first three vertices
    // This gives us the "raw" normal direction based on vertex order
    let v0 = ordered_vertices[0];
    let v1 = ordered_vertices[1];
    let v2 = ordered_vertices[2];
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let face_normal = edge1.cross(edge2).normalize();

    // Check if the face normal points in the same direction as solid-to-face vector
    // If dot product is positive, normal points outward (good)
    // If dot product is negative, normal points inward (need to flip winding)
    let normal_dot_solid_to_face = face_normal.dot(solid_to_polygon);

    let winding_order = if normal_dot_solid_to_face > 0.0 {
        WindingOrder::CounterClockwise
    } else {
        WindingOrder::Clockwise
    };

    // Use the corrected normal (always pointing outward)
    let normal = if normal_dot_solid_to_face > 0.0 {
        face_normal
    } else {
        -face_normal
    };

    // Step 5: Triangulate respecting the winding order
    // Convert the polygon into triangles that the GPU can render
    let mut triangulated_faces = Vec::new();
    let triangle_count = ordered_vertices.len() - 2;

    // Fan triangulation: connect first vertex to all other vertices
    // This works for convex polygons and ensures consistent orientation
    for i in 0..triangle_count {
        let (v0, v1, v2) = if winding_order == WindingOrder::CounterClockwise {
            // CounterClockwise: v0 -> v1 -> v2 (standard right-hand rule)
            (
                ordered_vertices[0],
                ordered_vertices[i + 1],
                ordered_vertices[i + 2],
            )
        } else {
            // Clockwise: v0 -> v2 -> v1 (reversed for inward normals)
            (
                ordered_vertices[0],
                ordered_vertices[i + 2],
                ordered_vertices[i + 1],
            )
        };

        triangulated_faces.push(TriangulatedFace {
            vertices: vec![v0, v1, v2],
            normal,
            winding_order,
        });
    }

    triangulated_faces
}
