/// Application layer for the application
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::{
    Polygon, PolygonRegistry, Segment, SegmentRegistry, Solid, Vertex, VertexRegistry,
};

/// Triangulated face data for rendering
#[derive(Debug)]
pub struct TriangulatedFace {
    /// The vertices of the triangulated face
    pub vertices: Vec<Vec3>,
    /// The normal vector of the face
    pub normal: Vec3,
    /// The winding order for proper face orientation
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
fn triangulate_polygon_for_rendering(
    polygon: &Polygon,
    segments: &HashMap<Uuid, Segment>,
    vertices: &HashMap<Uuid, Vertex>,
) -> Vec<TriangulatedFace> {
    println!("\n=== TRIANGULATING POLYGON {} ===", polygon.id);

    // Step 1: Extract all vertices from segments and validate they form a closed loop
    let mut vertex_counts: HashMap<Uuid, usize> = HashMap::new();
    let mut vertex_to_segments: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for segment_id in &polygon.segments {
        let segment = &segments[segment_id];
        *vertex_counts.entry(segment.start_vertex).or_insert(0) += 1;
        *vertex_counts.entry(segment.end_vertex).or_insert(0) += 1;

        // Track which segments connect to each vertex
        vertex_to_segments
            .entry(segment.start_vertex)
            .or_insert_with(Vec::new)
            .push(*segment_id);
        vertex_to_segments
            .entry(segment.end_vertex)
            .or_insert_with(Vec::new)
            .push(*segment_id);
    }

    println!("Step 1 - Loop Closure Validation:");
    println!("  Segment count: {}", polygon.segments.len());
    println!("  Vertex counts: {:?}", vertex_counts);

    // Check if each vertex appears exactly twice (start and end of segments)
    let valid_loop = vertex_counts.values().all(|&count| count == 2);
    if !valid_loop {
        println!("  ✗ Invalid polygon: vertices don't form a closed loop!");
        return vec![];
    }
    println!("  ✓ All vertices appear exactly twice");

    // Step 2: Reconstruct the vertex order by following segment connections
    let mut ordered_vertices = Vec::new();
    let mut visited_segments = HashSet::new();
    let mut current_vertex_id = None;

    println!("\nStep 2 - Reconstructing Vertex Order:");
    println!("  Processing {} segments", polygon.segments.len());

    // Start with any vertex that has exactly two segments
    for (vertex_id, segments) in &vertex_to_segments {
        if segments.len() == 2 {
            current_vertex_id = Some(*vertex_id);
            break;
        }
    }

    if current_vertex_id.is_none() {
        println!("  ✗ Could not find starting vertex!");
        return vec![];
    }

    // Follow the chain of segments to reconstruct the vertex order
    while visited_segments.len() < polygon.segments.len() {
        let vertex_id = current_vertex_id.unwrap();
        let vertex_pos = &vertices[&vertex_id].position;
        let vertex_vec3 = Vec3::new(vertex_pos.x, vertex_pos.y, vertex_pos.z);

        ordered_vertices.push(vertex_vec3);
        println!(
            "    Added vertex {} at position {:?}",
            vertex_id, vertex_vec3
        );

        // Find the next unvisited segment connected to this vertex
        let connected_segments = &vertex_to_segments[&vertex_id];
        let next_segment_id = connected_segments
            .iter()
            .find(|&&id| !visited_segments.contains(&id));

        if let Some(&segment_id) = next_segment_id {
            let segment = &segments[&segment_id];
            visited_segments.insert(segment_id);

            // Move to the other vertex of this segment
            current_vertex_id = if segment.start_vertex == vertex_id {
                Some(segment.end_vertex)
            } else {
                Some(segment.start_vertex)
            };

            println!(
                "    Next: segment {} -> vertex {}",
                segment_id,
                current_vertex_id.unwrap()
            );
        } else {
            // If we can't find a next segment, we've completed the loop
            break;
        }
    }

    if ordered_vertices.len() != polygon.segments.len() {
        println!(
            "  ✗ Failed to reconstruct vertex order! Got {} vertices, expected {}",
            ordered_vertices.len(),
            polygon.segments.len()
        );
        return vec![];
    }

    println!("  ✓ Reconstructed vertex order: {:?}", ordered_vertices);

    // Step 3: Calculate centers and determine normal direction
    // Calculate solid center (average of all solid vertices)
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
    let solid_to_polygon = polygon_center - solid_center;

    println!("\nStep 3 - Center Calculation:");
    println!("  Solid center: {:?}", solid_center);
    println!("  Polygon center: {:?}", polygon_center);
    println!("  Vector from solid to polygon: {:?}", solid_to_polygon);

    // Step 4: Use solid-to-face vector to determine correct winding order
    // The solid-to-face vector tells us which side is "outward"
    // We need to ensure vertices are ordered so the face normal points outward

    // Calculate the face normal using the first three vertices
    let v0 = ordered_vertices[0];
    let v1 = ordered_vertices[1];
    let v2 = ordered_vertices[2];
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let face_normal = edge1.cross(edge2).normalize();

    println!("\nStep 4 - Determining Winding Order:");
    println!("  Face normal from vertices: {:?}", face_normal);
    println!("  Vector from solid to face: {:?}", solid_to_polygon);

    // Check if the face normal points in the same direction as solid-to-face vector
    let normal_dot_solid_to_face = face_normal.dot(solid_to_polygon);
    println!(
        "  Dot product: normal • (solid→face) = {}",
        normal_dot_solid_to_face
    );

    // If dot product is positive, normal points outward (good)
    // If dot product is negative, normal points inward (need to flip winding)
    let winding_order = if normal_dot_solid_to_face > 0.0 {
        println!("  ✓ Face normal points outward, using CounterClockwise winding");
        WindingOrder::CounterClockwise
    } else {
        println!("  ✗ Face normal points inward, using Clockwise winding to flip");
        WindingOrder::Clockwise
    };

    // Use the corrected normal (always pointing outward)
    let normal = if normal_dot_solid_to_face > 0.0 {
        face_normal
    } else {
        -face_normal
    };

    println!("  Final normal: {:?}", normal);
    println!("  Final winding order: {:?}", winding_order);

    // Step 5: Triangulate respecting the winding order
    let mut triangulated_faces = Vec::new();
    let triangle_count = ordered_vertices.len() - 2;

    println!("\nStep 5 - Triangulation:");
    println!(
        "  Creating {} triangles from {} vertices with {:?} winding",
        triangle_count,
        ordered_vertices.len(),
        winding_order
    );

    for i in 0..triangle_count {
        let (v0, v1, v2) = if winding_order == WindingOrder::CounterClockwise {
            // CounterClockwise: v0 -> v1 -> v2
            (
                ordered_vertices[0],
                ordered_vertices[i + 1],
                ordered_vertices[i + 2],
            )
        } else {
            // Clockwise: v0 -> v2 -> v1 (reversed)
            (
                ordered_vertices[0],
                ordered_vertices[i + 2],
                ordered_vertices[i + 1],
            )
        };

        println!(
            "  Triangle {}: {:?} -> {:?} -> {:?} ({:?})",
            i, v0, v1, v2, winding_order
        );

        triangulated_faces.push(TriangulatedFace {
            vertices: vec![v0, v1, v2],
            normal,
            winding_order,
        });
    }

    println!(
        "  ✓ Created {} triangulated faces with {:?} winding",
        triangulated_faces.len(),
        winding_order
    );
    println!("=== END POLYGON {} ===\n", polygon.id);

    triangulated_faces
}

/// Creates a Bevy mesh from a domain Solid using the provided registries
pub fn create_mesh_from_solid(
    solid: &Solid,
    polygon_registry: &PolygonRegistry,
    segment_registry: &SegmentRegistry,
    vertex_registry: &VertexRegistry,
) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let mut current_index = 0u32;

    // Process each polygon in the solid
    println!("Processing solid with {} polygons", solid.polygons.len());
    for polygon_id in &solid.polygons {
        if let Some(polygon) = polygon_registry.get(polygon_id) {
            println!("Processing polygon {}", polygon.id);
            let triangulated_faces = triangulate_polygon_for_rendering(
                polygon,
                &segment_registry.segments,
                &vertex_registry.vertices,
            );

            println!("  Got {} triangulated faces", triangulated_faces.len());
            for face in triangulated_faces {
                // Add vertices for this triangle
                for (i, vertex) in face.vertices.iter().enumerate() {
                    positions.push([vertex.x, vertex.y, vertex.z]);
                    normals.push([face.normal.x, face.normal.y, face.normal.z]);

                    // Simple UV mapping
                    let uv = match i {
                        0 => [0.0, 0.0],
                        1 => [1.0, 0.0],
                        2 => [0.5, 1.0],
                        _ => [0.0, 0.0],
                    };
                    uvs.push(uv);
                }

                // Add indices for this triangle
                indices.push(current_index);
                indices.push(current_index + 1);
                indices.push(current_index + 2);

                current_index += 3;
            }
        }
    }

    // Build the mesh
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    mesh
}

/// Create a new solid
pub fn new_solid() -> Solid {
    // Arbitrary solid for now
    let solid = Solid {
        id: Uuid::new_v4(),
        polygons: vec![],
    };
    solid
}

/// Cube creation utilities for the application layer
pub mod cube;
