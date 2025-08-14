/// Define the Point type
use crate::domain::Vector;

#[derive(Clone, Debug)]
/// A position point in meters 3D space.
/// Points can be moved in space.
pub struct Point {
    /// The east coordinate of the point in meters.
    /// Positive values are to the east.
    pub x: f32,
    /// The north coordinate of the point in meters.
    /// Positive values are to the north.
    pub y: f32,
    /// The height coordinate of the point in meters.
    /// Positive values are up.
    pub z: f32,
}

impl Point {
    /// Move a point in space
    pub fn move_by_vector(&mut self, distance: &Vector) -> () {
        self.x += distance.x as f32;
        self.y += distance.y as f32;
        self.z += distance.z as f32;
    }

    /// Move a point to a defined position
    pub fn move_to_position(&mut self, position: &Point) -> () {
        self.x = position.x as f32;
        self.y = position.y as f32;
        self.z = position.z as f32;
    }
}
