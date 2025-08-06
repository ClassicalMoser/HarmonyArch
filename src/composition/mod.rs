/// Composition layer for the application

use crate::domain::{Solid, PointRegistry, Segment, Polygon};
use crate::application::create_cube;

/// Create a sample scene with a cube
pub fn create_sample_scene() -> (PointRegistry, Vec<Segment>, Vec<Polygon>, Solid) {
    // Create a complete cube with all geometric relationships
    create_cube()
}   