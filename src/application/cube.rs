use crate::domain::{Point, PolygonRegistry, SegmentRegistry, SolidRegistry, VertexRegistry};
use uuid::Uuid;

/// Create a cube solid with all its components using domain registries
/// Returns references to the created solid and all its components
pub fn create_cube(
    vertex_registry: &mut VertexRegistry,
    segment_registry: &mut SegmentRegistry,
    polygon_registry: &mut PolygonRegistry,
    solid_registry: &mut SolidRegistry,
) -> Uuid {
    // Phase 1: Create all vertices using registry methods
    // Coordinate system: X = left(-) to right(+), Y = bottom(-) to top(+), Z = back(-) to front(+)
    let bottom_back_left_id = vertex_registry.create_and_store(Point {
        x: -0.5, // Left
        y: -0.5, // Bottom
        z: -0.5, // Back
    });
    let bottom_back_right_id = vertex_registry.create_and_store(Point {
        x: 0.5,  // Right
        y: -0.5, // Bottom
        z: -0.5, // Back
    });
    let bottom_front_right_id = vertex_registry.create_and_store(Point {
        x: 0.5,  // Right
        y: -0.5, // Bottom
        z: 0.5,  // Front
    });
    let bottom_front_left_id = vertex_registry.create_and_store(Point {
        x: -0.5, // Left
        y: -0.5, // Bottom
        z: 0.5,  // Front
    });
    let top_back_left_id = vertex_registry.create_and_store(Point {
        x: -0.5, // Left
        y: 0.5,  // Top
        z: -0.5, // Back
    });
    let top_back_right_id = vertex_registry.create_and_store(Point {
        x: 0.5,  // Right
        y: 0.5,  // Top
        z: -0.5, // Back
    });
    let top_front_right_id = vertex_registry.create_and_store(Point {
        x: 0.5, // Right
        y: 0.5, // Top
        z: 0.5, // Front
    });
    let top_front_left_id = vertex_registry.create_and_store(Point {
        x: -0.5, // Left
        y: 0.5,  // Top
        z: 0.5,  // Front
    });

    // Phase 2: Get references to the vertices in the registry
    let bottom_back_left = vertex_registry
        .get(&bottom_back_left_id)
        .expect("Vertex was just inserted");
    let bottom_back_right = vertex_registry
        .get(&bottom_back_right_id)
        .expect("Vertex was just inserted");
    let bottom_front_right = vertex_registry
        .get(&bottom_front_right_id)
        .expect("Vertex was just inserted");
    let bottom_front_left = vertex_registry
        .get(&bottom_front_left_id)
        .expect("Vertex was just inserted");
    let top_back_left = vertex_registry
        .get(&top_back_left_id)
        .expect("Vertex was just inserted");
    let top_back_right = vertex_registry
        .get(&top_back_right_id)
        .expect("Vertex was just inserted");
    let top_front_right = vertex_registry
        .get(&top_front_right_id)
        .expect("Vertex was just inserted");
    let top_front_left = vertex_registry
        .get(&top_front_left_id)
        .expect("Vertex was just inserted");

    // Phase 3: Create all segments using registry methods
    let bottom_left_id = segment_registry.create_and_store(bottom_back_left, bottom_front_left);
    let bottom_back_id = segment_registry.create_and_store(bottom_back_right, bottom_back_left);
    let bottom_right_id = segment_registry.create_and_store(bottom_front_right, bottom_back_right);
    let bottom_front_id = segment_registry.create_and_store(bottom_front_left, bottom_front_right);

    let top_left_id = segment_registry.create_and_store(top_back_left, top_front_left);
    let top_back_id = segment_registry.create_and_store(top_back_right, top_back_left);
    let top_right_id = segment_registry.create_and_store(top_front_right, top_back_right);
    let top_front_id = segment_registry.create_and_store(top_front_left, top_front_right);

    let back_left_id = segment_registry.create_and_store(bottom_back_left, top_back_left);
    let back_right_id = segment_registry.create_and_store(bottom_back_right, top_back_right);
    let front_right_id = segment_registry.create_and_store(bottom_front_right, top_front_right);
    let front_left_id = segment_registry.create_and_store(bottom_front_left, top_front_left);

    // Phase 4: Get references to the segments in the registry
    let bottom_left = segment_registry
        .get(&bottom_left_id)
        .expect("Segment was just inserted");
    let bottom_back = segment_registry
        .get(&bottom_back_id)
        .expect("Segment was just inserted");
    let bottom_right = segment_registry
        .get(&bottom_right_id)
        .expect("Segment was just inserted");
    let bottom_front = segment_registry
        .get(&bottom_front_id)
        .expect("Segment was just inserted");
    let top_left = segment_registry
        .get(&top_left_id)
        .expect("Segment was just inserted");
    let top_back = segment_registry
        .get(&top_back_id)
        .expect("Segment was just inserted");
    let top_right = segment_registry
        .get(&top_right_id)
        .expect("Segment was just inserted");
    let top_front = segment_registry
        .get(&top_front_id)
        .expect("Segment was just inserted");
    let back_left = segment_registry
        .get(&back_left_id)
        .expect("Segment was just inserted");
    let back_right = segment_registry
        .get(&back_right_id)
        .expect("Segment was just inserted");
    let front_right = segment_registry
        .get(&front_right_id)
        .expect("Segment was just inserted");
    let front_left = segment_registry
        .get(&front_left_id)
        .expect("Segment was just inserted");

    // Phase 5: Create all polygons using registry methods
    let bottom_face_id = polygon_registry.create_and_store(vec![
        bottom_left,
        bottom_back,
        bottom_right,
        bottom_front,
    ]);
    let top_face_id =
        polygon_registry.create_and_store(vec![top_right, top_back, top_left, top_front]);
    let back_face_id =
        polygon_registry.create_and_store(vec![back_left, bottom_back, back_right, top_back]);
    let front_face_id =
        polygon_registry.create_and_store(vec![front_left, bottom_front, top_front, front_right]);
    let left_face_id =
        polygon_registry.create_and_store(vec![bottom_left, front_left, top_left, back_left]);
    let right_face_id =
        polygon_registry.create_and_store(vec![bottom_right, back_right, top_right, front_right]);

    // Phase 6: Get references to the polygons in the registry
    let bottom_face = polygon_registry
        .get(&bottom_face_id)
        .expect("Polygon was just inserted");
    let top_face = polygon_registry
        .get(&top_face_id)
        .expect("Polygon was just inserted");
    let back_face = polygon_registry
        .get(&back_face_id)
        .expect("Polygon was just inserted");
    let front_face = polygon_registry
        .get(&front_face_id)
        .expect("Polygon was just inserted");
    let left_face = polygon_registry
        .get(&left_face_id)
        .expect("Polygon was just inserted");
    let right_face = polygon_registry
        .get(&right_face_id)
        .expect("Polygon was just inserted");

    // Phase 7: Create the solid using registry method
    let solid_id: Uuid = solid_registry.create_and_store(vec![
        bottom_face,
        top_face,
        back_face,
        front_face,
        left_face,
        right_face,
    ]);

    // Phase 8: Return the ID of the solid
    solid_id
}
