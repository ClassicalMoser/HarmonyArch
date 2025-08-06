/// Application layer for the application

use crate::domain::{Point, Segment, Polygon, Solid, PointRegistry};
use uuid::Uuid;
use std::collections::HashMap;

/// Create a new solid
pub fn new_solid() -> Solid {
    // Arbitrary solid for now
    let solid = Solid {
        id: Uuid::new_v4(),
        polygons: vec![],
    };
    solid
}

/// Setup world with a cube
/// Returns (registry, segments, polygons, solid)
pub fn create_cube() -> (PointRegistry, Vec<Segment>, Vec<Polygon>, Solid) {
    // Create 8 vertices for a unit cube centered at origin
    let vertices = [
        // Bottom face (z = -0.5)
        Point { id: Uuid::new_v4(), x: -0.5, y: -0.5, z: -0.5 }, // 0: back-left-bottom
        Point { id: Uuid::new_v4(), x:  0.5, y: -0.5, z: -0.5 }, // 1: back-right-bottom
        Point { id: Uuid::new_v4(), x:  0.5, y:  0.5, z: -0.5 }, // 2: front-right-bottom
        Point { id: Uuid::new_v4(), x: -0.5, y:  0.5, z: -0.5 }, // 3: front-left-bottom
        // Top face (z = 0.5)
        Point { id: Uuid::new_v4(), x: -0.5, y: -0.5, z:  0.5 }, // 4: back-left-top
        Point { id: Uuid::new_v4(), x:  0.5, y: -0.5, z:  0.5 }, // 5: back-right-top
        Point { id: Uuid::new_v4(), x:  0.5, y:  0.5, z:  0.5 }, // 6: front-right-top
        Point { id: Uuid::new_v4(), x: -0.5, y:  0.5, z:  0.5 }, // 7: front-left-top
    ];

    // Build point registry
    let mut point_map = HashMap::new();
    for vertex in &vertices {
        point_map.insert(vertex.id, vertex.clone());
    }
    let registry = PointRegistry { points: point_map };

    // Create 12 edges for the cube
    let segments = vec![
        // Bottom face edges
        Segment { id: Uuid::new_v4(), start_point: vertices[0].id, end_point: vertices[1].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[1].id, end_point: vertices[2].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[2].id, end_point: vertices[3].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[3].id, end_point: vertices[0].id },
        // Top face edges
        Segment { id: Uuid::new_v4(), start_point: vertices[4].id, end_point: vertices[5].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[5].id, end_point: vertices[6].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[6].id, end_point: vertices[7].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[7].id, end_point: vertices[4].id },
        // Vertical edges
        Segment { id: Uuid::new_v4(), start_point: vertices[0].id, end_point: vertices[4].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[1].id, end_point: vertices[5].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[2].id, end_point: vertices[6].id },
        Segment { id: Uuid::new_v4(), start_point: vertices[3].id, end_point: vertices[7].id },
    ];

    // Create 6 faces for the cube
    let polygons = vec![
        // Bottom face (z = -0.5)
        Polygon { id: Uuid::new_v4(), segments: vec![segments[0].id, segments[1].id, segments[2].id, segments[3].id] },
        // Top face (z = 0.5)  
        Polygon { id: Uuid::new_v4(), segments: vec![segments[4].id, segments[5].id, segments[6].id, segments[7].id] },
        // Back face (y = -0.5)
        Polygon { id: Uuid::new_v4(), segments: vec![segments[0].id, segments[9].id, segments[4].id, segments[8].id] },
        // Front face (y = 0.5)
        Polygon { id: Uuid::new_v4(), segments: vec![segments[2].id, segments[11].id, segments[6].id, segments[10].id] },
        // Left face (x = -0.5)
        Polygon { id: Uuid::new_v4(), segments: vec![segments[3].id, segments[8].id, segments[7].id, segments[11].id] },
        // Right face (x = 0.5)
        Polygon { id: Uuid::new_v4(), segments: vec![segments[1].id, segments[10].id, segments[5].id, segments[9].id] },
    ];

    // Create the solid
    let solid = Solid {
        id: Uuid::new_v4(),
        polygons: polygons.iter().map(|p| p.id).collect(),
    };

    (registry, segments, polygons, solid)
}
