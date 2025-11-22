use bevy::prelude::*;

use crate::domain::GeometryRegistry;

/// Resource to store geometry registry for access in update systems
#[derive(Resource)]
pub struct GeometryRegistryResource {
    pub registry: GeometryRegistry,
}

/// Component to track which solid this entity represents
#[derive(Component)]
pub struct SolidId(pub uuid::Uuid);

/// System that renders segment outlines as 2px white lines locked to the 3D geometry
/// Draws lines in world space at the actual segment positions, transformed by entity transforms
pub fn render_segment_outlines_2d(
    mut gizmos: Gizmos,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    geometry_registry: Res<GeometryRegistryResource>,
    ui_state: Res<crate::interface::ui::UiState>,
    mesh_entities: Query<(&GlobalTransform, &SolidId), With<crate::interface::ui::ToggleableMesh>>,
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
    let polygon_registry = &geometry_registry.registry.polygons;
    let solid_registry = &geometry_registry.registry.solids;

    // For each mesh entity, render its solid's segments at the entity's transform
    for (entity_transform, solid_id_component) in mesh_entities.iter() {
        let solid_id = &solid_id_component.0;

        // Get the solid
        let Some(solid) = solid_registry.get(solid_id) else {
            continue;
        };

        // Get all segments for this solid by iterating through its polygons
        let mut solid_segments = std::collections::HashSet::new();
        for polygon_id in &solid.polygons {
            if let Some(polygon) = polygon_registry.get(polygon_id) {
                for segment_id in &polygon.segments {
                    solid_segments.insert(*segment_id);
                }
            }
        }

        // Draw each segment belonging to this solid, transformed by the entity's transform
        for segment_id in solid_segments {
            let Some(segment) = segment_registry.get(&segment_id) else {
                continue;
            };

            // Get vertex positions in local space
            let vertex1 = match vertex_registry.get(&segment.vertices[0]) {
                Some(v) => v,
                None => continue,
            };
            let pos1_local = Vec3::new(vertex1.position.x, vertex1.position.y, vertex1.position.z);

            let vertex2 = match vertex_registry.get(&segment.vertices[1]) {
                Some(v) => v,
                None => continue,
            };
            let pos2_local = Vec3::new(vertex2.position.x, vertex2.position.y, vertex2.position.z);

            // Transform to world space using the entity's transform
            let pos1_world = entity_transform.transform_point(pos1_local);
            let pos2_world = entity_transform.transform_point(pos2_local);

            // Check if both points are visible on screen (optional optimization)
            let pos1_visible = camera
                .world_to_viewport(camera_transform, pos1_world)
                .is_ok();
            let pos2_visible = camera
                .world_to_viewport(camera_transform, pos2_world)
                .is_ok();

            // Draw the line if at least one point is visible
            if pos1_visible || pos2_visible {
                gizmos.line(pos1_world, pos2_world, Color::WHITE);
            }
        }
    }
}
