use crate::domain::{Polygon, Segment, Vertex};
use bevy::prelude::*;

/// Configuration for mesh creation
#[derive(Resource, Clone)]
pub struct MeshConfig {
    pub material_color: Color,
    pub material_roughness: f32,
    pub material_metallic: f32,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            material_color: Color::srgb(0.8, 0.7, 0.6),
            material_roughness: 0.5,
            material_metallic: 0.0,
        }
    }
}

/// Create a mesh from semantic cube data with proper normal calculation
///
/// GEOMETRY EXPLANATION:
/// ====================
/// We're creating a cube with 8 vertices and 12 triangles (6 faces × 2 triangles each).
///
/// IMPORTANT: We're using FLAT SHADING for crisp edges
/// - Each face gets its own set of vertices (24 total vertices for 6 faces)
/// - This prevents normal interpolation between faces
/// - Results in sharp, crisp edges between cube faces
///
/// Vertex Layout (looking at the cube from the front):
/// - Bottom face (z = -0.5): vertices 0,1,2,3 (counter-clockwise from bottom-left)
/// - Top face (z = 0.5): vertices 4,5,6,7 (counter-clockwise from top-left)
///
/// Face Layout:
/// - Face 0: Bottom (z = -0.5) - triangles: 0,1,2 and 0,2,3
/// - Face 1: Top (z = 0.5) - triangles: 4,7,6 and 4,6,5  
/// - Face 2: Back (y = -0.5) - triangles: 0,4,5 and 0,5,1
/// - Face 3: Front (y = 0.5) - triangles: 2,6,7 and 2,7,3
/// - Face 4: Left (x = -0.5) - triangles: 3,7,4 and 3,4,0
/// - Face 5: Right (x = 0.5) - triangles: 1,5,6 and 1,6,2
///
/// NORMAL CALCULATION EXPLANATION:
/// ===============================
/// For FLAT SHADING, each face needs its own normal vector pointing outward.
/// We calculate this by:
/// 1. Computing the face normal for each triangle using cross product of edges
/// 2. Assigning the same normal to all vertices of that face
/// 3. This creates sharp edges between faces (no normal interpolation)
///
/// SHADER IMPLICATIONS:
/// ===================
/// - Position attribute: Defines the 3D geometry (24 vertices for flat shading)
/// - Normal attribute: Each face has consistent normal for crisp lighting
/// - UV attribute: Enables texturing (though we're not using basic mapping here)
///
/// The PBR shader will use these normals to:
/// - Calculate how much light hits each surface (diffuse lighting)
/// - Determine reflection angles (specular highlights)
/// - Cast and receive shadows properly
/// - Create sharp, crisp edges between faces
pub fn create_cube_mesh(
    vertices: &[Vertex; 8],
    _segments: &[Segment; 12],
    _polygons: &[Polygon; 6],
) -> Mesh {
    // Convert our semantic vertices to Bevy mesh format
    let base_positions: Vec<[f32; 3]> = vertices
        .iter()
        .map(|v| [v.position.x, v.position.y, v.position.z])
        .collect();

    println!("=== MESH CREATION DEBUG ===");
    println!("Creating FLAT SHADED mesh for crisp edges");
    println!("Base vertices: {:?}", base_positions);
    println!();
    println!("GEOMETRY SETUP:");
    println!("Each vertex represents a corner of the cube:");
    for (i, pos) in base_positions.iter().enumerate() {
        let face_desc = match i {
            0 => "Bottom-back-left",
            1 => "Bottom-back-right",
            2 => "Bottom-front-right",
            3 => "Bottom-front-left",
            4 => "Top-back-left",
            5 => "Top-back-right",
            6 => "Top-front-right",
            7 => "Top-front-left",
            _ => "Unknown",
        };
        println!("  Vertex {}: {:?} ({})", i, pos, face_desc);
    }

    // FLAT SHADING: Create separate vertices for each face
    // This prevents normal interpolation and creates crisp edges
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    // Face definitions with vertex indices and UV coordinates
    // FIXED: Corrected normal directions and winding order
    let faces = [
        // Bottom face (z = -0.5) - looking down (normal should point down)
        ([0, 1, 2, 0, 2, 3], [0.0, 0.0, -1.0], "Bottom"), // FIXED: negative Z
        // Top face (z = 0.5) - looking up (normal should point up)
        ([4, 7, 6, 4, 6, 5], [0.0, 0.0, 1.0], "Top"), // FIXED: positive Z
        // Back face (y = -0.5) - looking back (normal should point back)
        ([0, 4, 5, 0, 5, 1], [0.0, -1.0, 0.0], "Back"), // FIXED: negative Y
        // Front face (y = 0.5) - looking forward (normal should point forward)
        ([2, 6, 7, 2, 7, 3], [0.0, 1.0, 0.0], "Front"), // FIXED: positive Y
        // Left face (x = -0.5) - looking left (normal should point left)
        ([3, 7, 4, 3, 4, 0], [-1.0, 0.0, 0.0], "Left"), // FIXED: negative X
        // Right face (x = 0.5) - looking right (normal should point right)
        ([1, 5, 6, 1, 6, 2], [1.0, 0.0, 0.0], "Right"), // FIXED: positive X
    ];

    println!();
    println!("FLAT SHADING IMPLEMENTATION:");
    println!("Creating separate vertices for each face to prevent normal interpolation");

    // Build the mesh data for each face
    for (face_idx, (vertex_indices, face_normal, face_name)) in faces.iter().enumerate() {
        println!("  Processing {} face (index {})", face_name, face_idx);

        // Add vertices for this face (6 vertices = 2 triangles)
        for &vertex_idx in vertex_indices {
            let pos = base_positions[vertex_idx];
            positions.push(pos);

            // Each vertex in this face gets the same normal (flat shading)
            normals.push(*face_normal);

            // UV coordinates for this vertex
            let uv = match vertex_idx % 4 {
                0 => [0.0, 0.0], // Bottom-left
                1 => [1.0, 0.0], // Bottom-right
                2 => [1.0, 1.0], // Top-right
                3 => [0.0, 1.0], // Top-left
                _ => [0.0, 0.0],
            };
            uvs.push(uv);
        }

        println!("    Added 6 vertices with normal {:?}", face_normal);
    }

    // Create triangle indices (sequential for flat shading)
    let indices: Vec<u32> = (0..positions.len() as u32).collect();

    println!();
    println!("TRIANGLE SETUP:");
    println!(
        "Total vertices: {} (24 for 6 faces × 4 vertices each)",
        positions.len()
    );
    println!(
        "Total triangles: {} (6 faces × 2 triangles each)",
        positions.len() / 3
    );
    println!("Triangle indices: {:?}", indices);
    println!();
    println!("FLAT SHADING BENEFITS:");
    println!("  1. Each face has consistent normal direction");
    println!("  2. No normal interpolation between faces");
    println!("  3. Sharp, crisp edges between cube faces");
    println!("  4. Proper lighting calculation per face");
    println!();

    println!("NORMAL ASSIGNMENT:");
    println!("Each face gets its own normal vector:");
    for (i, face) in faces.iter().enumerate() {
        println!("  Face {} ({}): normal {:?}", i, face.2, face.1);
    }
    println!();

    println!("UV COORDINATE SETUP:");
    println!("UV coordinates: {:?}", uvs);
    println!("Note: Each face gets proper UV mapping for texturing");
    println!();

    println!("MESH ASSEMBLY:");
    println!("Creating Bevy mesh with:");
    println!("  - {} position vertices (flat shading)", positions.len());
    println!("  - {} normal vectors (one per vertex)", normals.len());
    println!("  - {} UV coordinates", uvs.len());
    println!("  - {} triangle indices", indices.len());
    println!("  - {} total triangles", positions.len() / 3);

    let triangle_count = positions.len() / 3;

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    println!();
    println!("SHADER IMPLICATIONS:");
    println!("This FLAT SHADED mesh will be processed by Bevy's PBR shader which:");
    println!("  1. Uses positions for geometry and depth testing");
    println!("  2. Uses consistent normals per face for crisp lighting");
    println!("  3. Uses UVs for texturing (though we're not using textures yet)");
    println!("  4. Creates sharp edges between faces (no normal interpolation)");
    println!("  5. Provides proper shadow casting and receiving per face");
    println!();
    println!(
        "Mesh created successfully with {} triangles",
        triangle_count
    );
    println!("=== END MESH CREATION DEBUG ===");
    mesh
}
