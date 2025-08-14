/// Domain layer for the application
/// Pure domain logic, no external dependencies, no ECS, no Bevy
pub mod point;
/// Polygon module for defining 3D polygons
pub mod polygon;
/// Segment module for defining line segments between vertices
pub mod segment;
/// Solid module for defining 3D solids composed of polygons
pub mod solid;
/// Vertex module for defining 3D vertices
pub mod vertex;

pub use point::*;
pub use polygon::*;
pub use segment::*;
pub use solid::*;
pub use vertex::*;

/// Constant to define unit size for coordinate system
pub const METERS_PER_UNIT: f32 = 1.0;

/// Create the origin point of the world
pub fn create_origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

/// A distance in meters 3D space.
/// Distances are more precise than points
pub struct Vector {
    /// The east component of the distance in meters.
    /// Positive values are to the east.
    pub x: f32,
    /// The north component of the distance in meters.
    /// Positive values are to the north.
    pub y: f32,
    /// The height component of the distance in meters.
    /// Positive values are up.
    pub z: f32,
}

/// Create a new distance
pub fn measure_vector(start_point: &Point, end_point: &Point) -> Vector {
    Vector {
        x: end_point.x as f32 - start_point.x as f32,
        y: end_point.y as f32 - start_point.y as f32,
        z: end_point.z as f32 - start_point.z as f32,
    }
}
