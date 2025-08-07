use crate::application::create_cube;
/// Composition layer for the application
use crate::domain::{Polygon, Segment, Solid, Vertex};

/// Create a sample scene with a cube
pub fn create_sample_scene() -> ([Vertex; 8], [Segment; 12], [Polygon; 6], Solid) {
    // Create a complete cube with all geometric relationships
    create_cube()
}
