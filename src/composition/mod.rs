//! Composition layer - Dependency injection and application setup

use crate::domain::{Element, wall, door, window, point, origin, NORTH, EAST};
use crate::infrastructure::{StlRenderer, SvgRenderer};

/// Application composition root
pub struct CompositionRoot;

impl CompositionRoot {
    /// Create a sample scene with basic elements
    pub fn create_sample_scene() -> Vec<Element> {
        vec![
            // Create a wall facing north
            wall(
                "north_wall".to_string(),
                origin(),
                10.0,  // 10m wide
                3.0,   // 3m tall
                70.0,
            ),

            // Create a door facing east
            door(
                "main_door".to_string(),
                point(3.0, 0.0, 0.0),
                2.0,  // 2m wide
                2.5,  // 2.5m tall
                EAST,
            ),

            // Create a window facing north
            window(
                "main_window".to_string(),
                point(7.0, 0.0, 1.0),
                3.0,  // 3m wide
                1.5,  // 1.5m tall
                NORTH,
            ),
        ]
    }

    /// Export scene to STL file
    pub fn export_to_stl(elements: &[Element], filename: &str) -> std::io::Result<()> {
        StlRenderer::write_stl(elements, filename)
    }

    /// Export scene to SVG file
    pub fn export_to_svg(elements: &[Element], filename: &str) -> std::io::Result<()> {
        SvgRenderer::write_svg(elements, filename)
    }
}
