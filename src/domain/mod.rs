/// Domain layer for the application
/// Pure domain logic, no external dependencies, no ECS, no Bevy
use uuid::Uuid;
use std::collections::HashMap;

/// meters per unit
pub const METERS_PER_UNIT: f32 = 1.0;

/// Create the origin point of the world
pub fn create_origin() -> Point {
    Point {
        id: Uuid::new_v4(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

/// A position point in meters 3D space
/// Points are less precise than distances
#[derive(Clone)]
pub struct Point {
    /// The unique identifier of the point
    pub id: Uuid,
    /// The east coordinate of the point in meters
    /// Positive values are to the east
    pub x: f32,
    /// The north coordinate of the point in meters
    /// Positive values are to the north
    pub y: f32,
    /// The height coordinate of the point in meters
    /// Positive values are up
    pub z: f32,
}

/// A registry of points
pub struct PointRegistry {
    /// The points in the registry
    pub points: HashMap<Uuid, Point>,
}

/// A distance in meters 3D space
/// Distances are more precise than points
pub struct Distance {
    /// The east component of the distance in meters
    /// Positive values are to the east
    pub x: f64,
    /// The north component of the distance in meters
    /// Positive values are to the north
    pub y: f64,
    /// The height component of the distance in meters
    /// Positive values are up
    pub z: f64,
}

/// A segment in 3D space
pub struct Segment {
    /// The unique identifier of the segment
    pub id: Uuid,
    /// Reference to the start point of the segment
    pub start_point: Uuid,
    /// Reference to the end point of the segment
    pub end_point: Uuid,
}

/// A polygon in 3D space
pub struct Polygon {
    /// The unique identifier of the polygon
    pub id: Uuid,
    /// Reference to the segments of the polygon
    pub segments: Vec<Uuid>,
}

/// A solid in 3D space
pub struct Solid {
    /// The unique identifier of the solid
    pub id: Uuid,
    /// Reference to the polygons of the solid
    pub polygons: Vec<Uuid>,
}