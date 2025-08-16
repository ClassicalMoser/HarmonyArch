/// Composition layer for the application
use crate::application::create_cube_solid;
use crate::domain::*;
use uuid::Uuid;

/// Create a sample scene with a cube
pub fn create_sample_scene(geometry_registry: &mut GeometryRegistry) -> Uuid {
    let solid_id = create_cube_solid(1.0, geometry_registry);
    solid_id
}
