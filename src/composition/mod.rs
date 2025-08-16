/// Composition layer for the application
use crate::application::create_cube_solid;
use crate::domain::*;
use uuid::Uuid;

/// Create a sample scene with a cube
pub fn create_sample_scene(
    vertex_registry: &mut VertexRegistry,
    segment_registry: &mut SegmentRegistry,
    polygon_registry: &mut PolygonRegistry,
    solid_registry: &mut SolidRegistry,
) -> Uuid {
    let solid_id = create_cube_solid(
        1.0,
        vertex_registry,
        segment_registry,
        polygon_registry,
        solid_registry,
    );
    solid_id.expect("Failed to create cube for sample scene")
}
