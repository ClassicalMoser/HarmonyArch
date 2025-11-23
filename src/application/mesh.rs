/// Mesh creation module for converting domain solids into Bevy meshes
use bevy::prelude::*;

use crate::domain::{GeometryRegistry, Solid};

use super::triangulation::triangulate_polygon_for_rendering;

/// Creates a Bevy mesh from a domain Solid using the provided registries
/// This function translates our domain model into a renderable mesh with proper triangulation
pub fn create_mesh_from_solid(solid: &Solid, geometry_registry: &GeometryRegistry) -> Mesh {
    let polygon_registry = &geometry_registry.polygons;
    let segment_registry = &geometry_registry.segments;
    let vertex_registry = &geometry_registry.vertices;

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
    for polygon_id in &solid.polygons {
        if let Some(polygon) = polygon_registry.get(polygon_id) {
            // Triangulate this polygon into renderable triangles
            let triangulated_faces = triangulate_polygon_for_rendering(
                polygon,
                &segment_registry.segments,
                &vertex_registry.vertices,
            );

            // Process each triangulated face to add to the mesh
            for face in triangulated_faces.iter() {
                // Add vertices for this triangle to the mesh buffers
                for (vertex_idx, vertex) in face.vertices.iter().enumerate() {
                    // Position: Convert our Vec3 to Bevy's expected format
                    positions.push([vertex.x, vertex.y, vertex.z]);

                    // Normal: Each vertex gets the face normal for consistent lighting
                    normals.push([face.normal.x, face.normal.y, face.normal.z]);

                    // UV coordinates: Simple mapping that could be improved
                    // This creates a basic texture coordinate pattern for each triangle
                    let uv = match vertex_idx {
                        0 => [0.0, 0.0], // Bottom-left of triangle
                        1 => [1.0, 0.0], // Bottom-right of triangle
                        2 => [0.5, 1.0], // Top of triangle
                        _ => [0.0, 0.0], // Fallback (shouldn't happen)
                    };
                    uvs.push(uv);
                }

                // Add triangle indices - these tell the GPU how to connect vertices
                // Each triangle uses 3 consecutive vertices in our buffer
                indices.push(current_index); // First vertex
                indices.push(current_index + 1); // Second vertex
                indices.push(current_index + 2); // Third vertex

                // Move to next set of 3 vertices
                current_index += 3;
            }
        }
    }

    // Build the final Bevy mesh by inserting all the data
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    mesh
}
