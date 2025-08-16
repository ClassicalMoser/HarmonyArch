use crate::domain::{GeometryRegistry, Point};
use uuid::Uuid;

/// Create a cuboid solid with all its components using domain registries
/// Returns references to the created solid and all its components
pub fn create_rectangular_solid(
    width: f32,
    height: f32,
    depth: f32,
    geometry_registry: &mut GeometryRegistry,
) -> Uuid {
    // Phase 1: Create all vertices using registry methods
    // Coordinate system: X = left(-) to right(+), Y = bottom(-) to top(+), Z = back(-) to front(+)
    let vertex_registry = &mut geometry_registry.vertices;
    let segment_registry = &mut geometry_registry.segments;
    let polygon_registry = &mut geometry_registry.polygons;
    let solid_registry = &mut geometry_registry.solids;

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

    // Phase 2: Create all segments using registry methods
    let bottom_left = segment_registry.create_and_store(&bottom_back_left, &bottom_front_left);
    let bottom_back = segment_registry.create_and_store(&bottom_back_right, &bottom_back_left);
    let bottom_right = segment_registry.create_and_store(&bottom_front_right, &bottom_back_right);
    let bottom_front = segment_registry.create_and_store(&bottom_front_left, &bottom_front_right);

    let top_left = segment_registry.create_and_store(&top_back_left, &top_front_left);
    let top_back = segment_registry.create_and_store(&top_back_right, &top_back_left);
    let top_right = segment_registry.create_and_store(&top_front_right, &top_back_right);
    let top_front = segment_registry.create_and_store(&top_front_left, &top_front_right);

    let back_left = segment_registry.create_and_store(&bottom_back_left, &top_back_left);
    let back_right = segment_registry.create_and_store(&bottom_back_right, &top_back_right);
    let front_right = segment_registry.create_and_store(&bottom_front_right, &top_front_right);
    let front_left = segment_registry.create_and_store(&bottom_front_left, &top_front_left);

    // Phase 3: Create all polygons using registry methods
    let bottom_face = polygon_registry.create_and_store(vec![
        &bottom_left,
        &bottom_back,
        &bottom_right,
        &bottom_front,
    ]);
    let top_face =
        polygon_registry.create_and_store(vec![&top_right, &top_back, &top_left, &top_front]);
    let back_face =
        polygon_registry.create_and_store(vec![&back_left, &bottom_back, &back_right, &top_back]);
    let front_face = polygon_registry.create_and_store(vec![
        &front_left,
        &bottom_front,
        &top_front,
        &front_right,
    ]);
    let left_face =
        polygon_registry.create_and_store(vec![&bottom_left, &front_left, &top_left, &back_left]);
    let right_face = polygon_registry.create_and_store(vec![
        &bottom_right,
        &back_right,
        &top_right,
        &front_right,
    ]);

    // Phase 4: Create the solid using registry method
    let solid_id = solid_registry.create_and_store(vec![
        &bottom_face,
        &top_face,
        &back_face,
        &front_face,
        &left_face,
        &right_face,
    ]);

    // Phase 5: Return the ID of the solid
    solid_id
}

/// Create a cube solid with all its components using domain registries
/// Returns references to the created solid
pub fn create_cube_solid(side_length: f32, geometry_registry: &mut GeometryRegistry) -> Uuid {
    create_rectangular_solid(side_length, side_length, side_length, geometry_registry)
}
