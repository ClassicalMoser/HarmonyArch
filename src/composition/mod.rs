//! Composition layer - Dependency injection and application setup

use crate::domain::{Element, wall, door, window, point, origin};
use crate::infrastructure::StlRenderer;

/// Application composition root
pub struct CompositionRoot;

impl CompositionRoot {
    /// Create a sample scene with basic elements
    pub fn create_sample_scene() -> Vec<Element> {
        vec![
            // Create a wall using factory function
            wall(
                "north_wall".to_string(),
                origin(),
                10.0,  // 10m wide
                3.0,   // 3m tall
            ),
            
            // Create a door
            door(
                "main_door".to_string(),
                point(3.0, 0.0, 0.0),
                2.0,  // 2m wide
                2.5,  // 2.5m tall
            ),
            
            // Create a window
            window(
                "main_window".to_string(),
                point(7.0, 0.0, 1.0),
                3.0,  // 3m wide
                1.5,  // 1.5m tall
            ),
        ]
    }

    /// Export scene to STL file
    pub fn export_to_stl(elements: &[Element], filename: &str) -> std::io::Result<()> {
        StlRenderer::write_stl(elements, filename)
    }
} 