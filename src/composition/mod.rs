/// Composition layer for the application

use crate::domain::{Solid, Segment, Polygon, Vertex};
use crate::application::create_cube;

/// Create a sample scene with a cube
pub fn create_sample_scene() -> ([Vertex; 8], [Segment; 12], [Polygon; 6], Solid) {
    // Create a complete cube with all geometric relationships
    create_cube()
}   