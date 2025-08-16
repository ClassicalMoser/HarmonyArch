use crate::domain::Point;

/// Define a vector in 3D space
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
