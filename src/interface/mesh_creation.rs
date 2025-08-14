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
            material_color: Color::srgb(0.9, 0.9, 0.9),
            material_roughness: 0.5, // Bevy's default - good balance
            material_metallic: 0.0,
        }
    }
}
