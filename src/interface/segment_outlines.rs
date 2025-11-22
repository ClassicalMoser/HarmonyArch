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
    ui_state: Res<crate::interface::ui::UiState>,
) {
    // Only render if outlines are enabled
    if !ui_state.show_outlines {
        return;
    }
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let segment_registry = &geometry_registry.registry.segments;
    let vertex_registry = &geometry_registry.registry.vertices;

    // Draw each segment as a line in 3D world space
    for segment in segment_registry.segments.values() {
        // Get first vertex position
        let vertex1 = match vertex_registry.get(&segment.vertices[0]) {
            Some(v) => v,
            None => continue,
        };
        let pos1_3d = Vec3::new(vertex1.position.x, vertex1.position.y, vertex1.position.z);

        // Get second vertex position
        let vertex2 = match vertex_registry.get(&segment.vertices[1]) {
            Some(v) => v,
            None => continue,
        };
        let pos2_3d = Vec3::new(vertex2.position.x, vertex2.position.y, vertex2.position.z);

        // Check if both points are visible on screen (optional optimization)
        let pos1_visible = camera.world_to_viewport(camera_transform, pos1_3d).is_ok();
        let pos2_visible = camera.world_to_viewport(camera_transform, pos2_3d).is_ok();

        // Draw the line if at least one point is visible
        if pos1_visible || pos2_visible {
            // Draw the line directly in 3D world space at the segment positions
            // This locks it to the geometry - it moves with the scene
            gizmos.line(pos1_3d, pos2_3d, Color::WHITE);
        }
    }
}
