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
/// This function takes a polygon (defined by segments) and converts it into triangulated faces
/// that can be rendered by the GPU. It handles:
/// 1. Building a vertex connectivity graph from segments
/// 2. Finding the circular path through connected vertices
/// 3. Calculating proper face normals
/// 4. Determining correct winding order for outward-facing normals
/// 5. Creating triangles with consistent orientation
fn triangulate_polygon_for_rendering(
    polygon: &Polygon,
    segments: &HashMap<Uuid, Segment>,
    vertices: &HashMap<Uuid, Vertex>,
) -> Vec<TriangulatedFace> {
    println!("\n=== TRIANGULATING POLYGON {} ===", polygon.id);

    // Step 1: Build vertex connectivity graph from segments
    // Each vertex maps to a list of vertices it connects to
    let mut vertex_connections: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    let mut vertex_counts: HashMap<Uuid, usize> = HashMap::new();

    println!("Step 1: Building vertex connectivity graph...");
    for segment_id in &polygon.segments {
        let segment = &segments[segment_id];
        println!(
            "  Segment {}: connects vertex {} to vertex {}",
            segment_id, segment.start_vertex, segment.end_vertex
        );

        // Count vertex occurrences (each should appear exactly twice in a valid polygon)
        *vertex_counts.entry(segment.start_vertex).or_insert(0) += 1;
        *vertex_counts.entry(segment.end_vertex).or_insert(0) += 1;

        // Build bidirectional connectivity graph
        vertex_connections
            .entry(segment.start_vertex)
            .or_insert_with(Vec::new)
            .push(segment.end_vertex);
        vertex_connections
            .entry(segment.end_vertex)
            .or_insert_with(Vec::new)
            .push(segment.start_vertex);
    }

    println!("Step 1 - Loop Closure Validation:");
    println!("  Segment count: {}", polygon.segments.len());
    println!("  Vertex counts: {:?}", vertex_counts);
    println!("  Vertex connections: {:?}", vertex_connections);

    // Check if each vertex appears exactly twice (start and end of segments)
    let valid_loop = vertex_counts.values().all(|&count| count == 2);
    if !valid_loop {
        println!("  ✗ Invalid polygon: vertices don't form a closed loop!");
        println!("    Expected: all vertices appear exactly twice");
        println!("    Actual: {:?}", vertex_counts);
        return vec![];
    }
    println!("  ✓ All vertices appear exactly twice - valid closed loop");

    // Step 2: Find the circular path through the vertex connectivity graph
    // This approach is agnostic to segment ordering
    let mut ordered_vertices = Vec::new();
    let mut visited_vertices = HashSet::new();
    let mut current_vertex_id = None;

    println!("\nStep 2 - Finding Circular Path:");
    println!("  Processing {} vertices", vertex_counts.len());

    // Start with any vertex that has exactly two connections
    for (vertex_id, connections) in &vertex_connections {
        if connections.len() == 2 {
            current_vertex_id = Some(*vertex_id);
            println!(
                "    Starting from vertex {} (has {} connections)",
                vertex_id,
                connections.len()
            );
            break;
        }
    }

    if current_vertex_id.is_none() {
        println!("  ✗ Could not find starting vertex with exactly 2 connections!");
        println!("    Available vertices: {:?}", vertex_connections);
        return vec![];
    }

    // Follow the circular path through connected vertices
    // This builds the proper vertex order regardless of segment storage order
    println!("    Following circular path through vertex graph...");
    while visited_vertices.len() < vertex_counts.len() {
        let vertex_id = current_vertex_id.unwrap();
        let vertex_pos = &vertices[&vertex_id].position;
        let vertex_vec3 = Vec3::new(vertex_pos.x, vertex_pos.y, vertex_pos.z);

        ordered_vertices.push(vertex_vec3);
        visited_vertices.insert(vertex_id);
        println!(
            "      Added vertex {} at position {:?} (vertex {} of {})",
            vertex_id,
            vertex_vec3,
            ordered_vertices.len(),
            vertex_counts.len()
        );

        // Find the next unvisited vertex connected to current vertex
        let connections = &vertex_connections[&vertex_id];
        let next_vertex_id = connections
            .iter()
            .find(|&&id| !visited_vertices.contains(&id));

        if let Some(&next_id) = next_vertex_id {
            current_vertex_id = Some(next_id);
            println!("      Next: vertex {} -> vertex {}", vertex_id, next_id);
        } else {
            // If we can't find a next vertex, we've completed the loop
            println!("      No more unvisited connections - loop complete");
            break;
        }
    }

    // Validate that we got the right number of vertices
    if ordered_vertices.len() != vertex_counts.len() {
        println!(
            "  ✗ Failed to find complete circular path! Got {} vertices, expected {}",
            ordered_vertices.len(),
            vertex_counts.len()
        );
        println!("    This suggests the vertex connectivity doesn't form a proper cycle");
        return vec![];
    }

    println!("  ✓ Found circular path: {:?}", ordered_vertices);
    println!(
        "    Vertex positions: {:?}",
        ordered_vertices
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<_>>()
    );

    // Step 3: Calculate centers and determine normal direction
    // We need to know which way the face normal should point (outward from the solid)
    println!("\nStep 3 - Center Calculation and Normal Direction:");

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

    println!("  Solid center: {:?}", solid_center);
    println!("  Polygon center: {:?}", polygon_center);
    println!("  Vector from solid to polygon: {:?}", solid_to_polygon);

    // Step 4: Use solid-to-face vector to determine correct winding order
    // The solid-to-face vector tells us which side is "outward"
    // We need to ensure vertices are ordered so the face normal points outward
    println!("\nStep 4 - Determining Winding Order:");

    // Calculate the face normal using the first three vertices
    // This gives us the "raw" normal direction based on vertex order
    let v0 = ordered_vertices[0];
    let v1 = ordered_vertices[1];
    let v2 = ordered_vertices[2];
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let face_normal = edge1.cross(edge2).normalize();

    println!("  First three vertices: {:?}, {:?}, {:?}", v0, v1, v2);
    println!("  Edge vectors: {:?}, {:?}", edge1, edge2);
    println!("  Face normal from vertices: {:?}", face_normal);
    println!("  Vector from solid to face: {:?}", solid_to_polygon);

    // Check if the face normal points in the same direction as solid-to-face vector
    // If dot product is positive, normal points outward (good)
    // If dot product is negative, normal points inward (need to flip winding)
    let normal_dot_solid_to_face = face_normal.dot(solid_to_polygon);
    println!(
        "  Dot product: normal • (solid→face) = {}",
        normal_dot_solid_to_face
    );

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
    // Convert the polygon into triangles that the GPU can render
    println!("\nStep 5 - Triangulation:");
    let mut triangulated_faces = Vec::new();
    let triangle_count = ordered_vertices.len() - 2;

    println!(
        "  Creating {} triangles from {} vertices with {:?} winding",
        triangle_count,
        ordered_vertices.len(),
        winding_order
    );

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
/// This function translates our domain model into a renderable mesh with proper triangulation
pub fn create_mesh_from_solid(
    solid: &Solid,
    polygon_registry: &PolygonRegistry,
    segment_registry: &SegmentRegistry,
    vertex_registry: &VertexRegistry,
) -> Mesh {
    println!("\n=== MESH GENERATION START ===");
    println!(
        "Creating mesh from solid with {} polygons",
        solid.polygons.len()
    );

    // Initialize the Bevy mesh with triangle list topology
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    // Mesh data buffers - these will hold all the geometry information
    let mut positions = Vec::new(); // Vertex positions in 3D space
    let mut normals = Vec::new(); // Surface normals for lighting
    let mut uvs = Vec::new(); // Texture coordinates
    let mut indices = Vec::new(); // Triangle indices for efficient rendering

    // Track the current vertex index for building triangles
    let mut current_index = 0u32;

    // Process each polygon in the solid to build the complete mesh
    println!("\n--- PROCESSING POLYGONS ---");
    for (polygon_idx, polygon_id) in solid.polygons.iter().enumerate() {
        println!(
            "\nProcessing polygon {} ({} of {})",
            polygon_id,
            polygon_idx + 1,
            solid.polygons.len()
        );

        if let Some(polygon) = polygon_registry.get(polygon_id) {
            println!("  Polygon found: {} segments", polygon.segments.len());

            // Triangulate this polygon into renderable triangles
            let triangulated_faces = triangulate_polygon_for_rendering(
                polygon,
                &segment_registry.segments,
                &vertex_registry.vertices,
            );

            println!("  Triangulation result: {} faces", triangulated_faces.len());

            // Process each triangulated face to add to the mesh
            for (face_idx, face) in triangulated_faces.iter().enumerate() {
                println!(
                    "    Processing face {}: {} vertices, normal {:?}",
                    face_idx,
                    face.vertices.len(),
                    face.normal
                );

                // Add vertices for this triangle to the mesh buffers
                for (vertex_idx, vertex) in face.vertices.iter().enumerate() {
                    // Position: Convert our Vec3 to Bevy's expected format
                    positions.push([vertex.x, vertex.y, vertex.z]);
                    println!("      Vertex {}: position {:?}", vertex_idx, vertex);

                    // Normal: Each vertex gets the face normal for consistent lighting
                    normals.push([face.normal.x, face.normal.y, face.normal.z]);
                    println!("      Vertex {}: normal {:?}", vertex_idx, face.normal);

                    // UV coordinates: Simple mapping that could be improved
                    // This creates a basic texture coordinate pattern for each triangle
                    let uv = match vertex_idx {
                        0 => [0.0, 0.0], // Bottom-left of triangle
                        1 => [1.0, 0.0], // Bottom-right of triangle
                        2 => [0.5, 1.0], // Top of triangle
                        _ => [0.0, 0.0], // Fallback (shouldn't happen)
                    };
                    uvs.push(uv);
                    println!("      Vertex {}: UV {:?}", vertex_idx, uv);
                }

                // Add triangle indices - these tell the GPU how to connect vertices
                // Each triangle uses 3 consecutive vertices in our buffer
                indices.push(current_index); // First vertex
                indices.push(current_index + 1); // Second vertex
                indices.push(current_index + 2); // Third vertex

                println!(
                    "      Triangle indices: [{}, {}, {}]",
                    current_index,
                    current_index + 1,
                    current_index + 2
                );

                // Move to next set of 3 vertices
                current_index += 3;
            }
        } else {
            println!("  WARNING: Polygon {} not found in registry!", polygon_id);
        }
    }

    // Final mesh statistics
    println!("\n--- MESH ASSEMBLY ---");
    println!("Total vertices: {}", positions.len());
    println!("Total triangles: {}", indices.len() / 3);
    println!("Total normals: {}", normals.len());
    println!("Total UVs: {}", uvs.len());

    // Validate mesh data integrity
    if positions.len() != normals.len() || positions.len() != uvs.len() {
        println!("  ERROR: Mismatched buffer sizes!");
        println!("    Positions: {}", positions.len());
        println!("    Normals: {}", normals.len());
        println!("    UVs: {}", uvs.len());
    }

    if indices.len() % 3 != 0 {
        println!("  ERROR: Indices not divisible by 3!");
        println!("    Indices count: {}", indices.len());
    }

    // Build the final Bevy mesh by inserting all the data
    println!("\n--- BUILDING BEVY MESH ---");
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    println!("=== MESH GENERATION COMPLETE ===\n");
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
