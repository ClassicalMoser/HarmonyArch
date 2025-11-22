use bevy::prelude::*;

use crate::domain::GeometryRegistry;

/// Resource to store geometry registry for access in update systems
#[derive(Resource)]
pub struct GeometryRegistryResource {
    pub registry: GeometryRegistry,
}

/// System that renders segment outlines as 2px white lines locked to the 3D geometry
/// Draws lines in world space at the actual segment positions
pub fn render_segment_outlines_2d(
    mut gizmos: Gizmos,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    geometry_registry: Res<GeometryRegistryResource>,
) {
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let segment_registry = &geometry_registry.registry.segments;
    let vertex_registry = &geometry_registry.registry.vertices;

    // Draw each segment as a line in 3D world space
    for segment in segment_registry.segments.values() {
        // Get start vertex position
        let start_vertex = match vertex_registry.get(&segment.start_vertex) {
            Some(v) => v,
            None => continue,
        };
        let start_pos_3d = Vec3::new(
            start_vertex.position.x,
            start_vertex.position.y,
            start_vertex.position.z,
        );

        // Get end vertex position
        let end_vertex = match vertex_registry.get(&segment.end_vertex) {
            Some(v) => v,
            None => continue,
        };
        let end_pos_3d = Vec3::new(
            end_vertex.position.x,
            end_vertex.position.y,
            end_vertex.position.z,
        );

        // Check if both points are visible on screen (optional optimization)
        let start_visible = camera
            .world_to_viewport(camera_transform, start_pos_3d)
            .is_ok();
        let end_visible = camera
            .world_to_viewport(camera_transform, end_pos_3d)
            .is_ok();

        // Draw the line if at least one point is visible
        if start_visible || end_visible {
            // Draw the line directly in 3D world space at the segment positions
            // This locks it to the geometry - it moves with the scene
            gizmos.line(start_pos_3d, end_pos_3d, Color::WHITE);
        }
    }
}
