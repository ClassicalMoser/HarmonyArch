use crate::domain::{Point, PolygonRegistry, SegmentRegistry, SolidRegistry, VertexRegistry};
use uuid::Uuid;

/// Create a cuboid solid with all its components using domain registries
/// Returns references to the created solid and all its components
pub fn create_rectangular_solid(
    width: f32,
    height: f32,
    depth: f32,
    vertex_registry: &mut VertexRegistry,
    segment_registry: &mut SegmentRegistry,
    polygon_registry: &mut PolygonRegistry,
    solid_registry: &mut SolidRegistry,
) -> Option<Uuid> {
    // Phase 1: Create all vertices using registry methods
    // Coordinate system: X = left(-) to right(+), Y = bottom(-) to top(+), Z = back(-) to front(+)
    let bottom_back_left = vertex_registry.create_and_store(Point {
        x: -width / 2.0,  // Left
        y: -height / 2.0, // Bottom
        z: -depth / 2.0,  // Back
    });
    let bottom_back_right = vertex_registry.create_and_store(Point {
        x: width / 2.0,   // Right
        y: -height / 2.0, // Bottom
        z: -depth / 2.0,  // Back
    });
    let bottom_front_right = vertex_registry.create_and_store(Point {
        x: width / 2.0,   // Right
        y: -height / 2.0, // Bottom
        z: depth / 2.0,   // Front
    });
    let bottom_front_left = vertex_registry.create_and_store(Point {
        x: -width / 2.0,  // Left
        y: -height / 2.0, // Bottom
        z: depth / 2.0,   // Front
    });
    let top_back_left = vertex_registry.create_and_store(Point {
        x: -width / 2.0, // Left
        y: height / 2.0, // Top
        z: -depth / 2.0, // Back
    });
    let top_back_right = vertex_registry.create_and_store(Point {
        x: width / 2.0,  // Right
        y: height / 2.0, // Top
        z: -depth / 2.0, // Back
    });
    let top_front_right = vertex_registry.create_and_store(Point {
        x: width / 2.0,  // Right
        y: height / 2.0, // Top
        z: depth / 2.0,  // Front
    });
    let top_front_left = vertex_registry.create_and_store(Point {
        x: -width / 2.0, // Left
        y: height / 2.0, // Top
        z: depth / 2.0,  // Front
    });

    // Phase 3: Create all segments using registry methods
    let bottom_left = segment_registry
        .create_and_store(&bottom_back_left, &bottom_front_left, vertex_registry)
        .expect("Failed to create bottom left segment");
    let bottom_back = segment_registry
        .create_and_store(&bottom_back_right, &bottom_back_left, vertex_registry)
        .expect("Failed to create bottom back segment");
    let bottom_right = segment_registry
        .create_and_store(&bottom_front_right, &bottom_back_right, vertex_registry)
        .expect("Failed to create bottom right segment");
    let bottom_front = segment_registry
        .create_and_store(&bottom_front_left, &bottom_front_right, vertex_registry)
        .expect("Failed to create bottom front segment");

    let top_left = segment_registry
        .create_and_store(&top_back_left, &top_front_left, vertex_registry)
        .expect("Failed to create top left segment");
    let top_back = segment_registry
        .create_and_store(&top_back_right, &top_back_left, vertex_registry)
        .expect("Failed to create top back segment");
    let top_right = segment_registry
        .create_and_store(&top_front_right, &top_back_right, vertex_registry)
        .expect("Failed to create top right segment");
    let top_front = segment_registry
        .create_and_store(&top_front_left, &top_front_right, vertex_registry)
        .expect("Failed to create top front segment");

    let back_left = segment_registry
        .create_and_store(&bottom_back_left, &top_back_left, vertex_registry)
        .expect("Failed to create back left segment");
    let back_right = segment_registry
        .create_and_store(&bottom_back_right, &top_back_right, vertex_registry)
        .expect("Failed to create back right segment");
    let front_right = segment_registry
        .create_and_store(&bottom_front_right, &top_front_right, vertex_registry)
        .expect("Failed to create front right segment");
    let front_left = segment_registry
        .create_and_store(&bottom_front_left, &top_front_left, vertex_registry)
        .expect("Failed to create front left segment");

    // Phase 5: Create all polygons using registry methods
    let bottom_face = polygon_registry
        .create_and_store(
            vec![&bottom_left, &bottom_back, &bottom_right, &bottom_front],
            segment_registry,
        )
        .expect("Failed to create bottom face");
    let top_face = polygon_registry
        .create_and_store(
            vec![&top_right, &top_back, &top_left, &top_front],
            segment_registry,
        )
        .expect("Failed to create top face");
    let back_face = polygon_registry
        .create_and_store(
            vec![&back_left, &bottom_back, &back_right, &top_back],
            segment_registry,
        )
        .expect("Failed to create back face");
    let front_face = polygon_registry
        .create_and_store(
            vec![&front_left, &bottom_front, &top_front, &front_right],
            segment_registry,
        )
        .expect("Failed to create front face");
    let left_face = polygon_registry
        .create_and_store(
            vec![&bottom_left, &front_left, &top_left, &back_left],
            segment_registry,
        )
        .expect("Failed to create left face");
    let right_face = polygon_registry
        .create_and_store(
            vec![&bottom_right, &back_right, &top_right, &front_right],
            segment_registry,
        )
        .expect("Failed to create right face");

    // Phase 7: Create the solid using registry method
    let solid_id = solid_registry.create_and_store(
        vec![
            &bottom_face,
            &top_face,
            &back_face,
            &front_face,
            &left_face,
            &right_face,
        ],
        polygon_registry,
    );

    // Phase 8: Return the ID of the solid if it was created successfully

    if solid_id.is_none() {
        return None;
    }

    let solid_id = solid_id.expect("None failed to return in solid storage");
    Some(solid_id)
}

/// Create a cube solid with all its components using domain registries
/// Returns references to the created solid
pub fn create_cube_solid(
    side_length: f32,
    vertex_registry: &mut VertexRegistry,
    segment_registry: &mut SegmentRegistry,
    polygon_registry: &mut PolygonRegistry,
    solid_registry: &mut SolidRegistry,
) -> Option<Uuid> {
    create_rectangular_solid(
        side_length,
        side_length,
        side_length,
        vertex_registry,
        segment_registry,
        polygon_registry,
        solid_registry,
    )
}
