//! Application layer - Use cases and business logic coordination

use crate::domain::{Element, ElementType, Point};
use nalgebra::Vector3;

/// Service for semantic architectural operations
pub struct GeometryService;

impl GeometryService {
    /// Convert an element to a simple mesh representation
    /// TODO: Replace with proper csgrs integration once API is understood
    pub fn element_to_mesh(_elem: &Element) -> String {
        // Placeholder - return element type as string for now
        "mesh_placeholder".to_string()
    }

    /// Calculate distance between two elements using nalgebra
    pub fn distance_between(elem1: &Element, elem2: &Element) -> f64 {
        let pos1 = Vector3::from(elem1.position);
        let pos2 = Vector3::from(elem2.position);
        (pos1 - pos2).norm()
    }

    /// Check if two elements intersect using bounding box
    pub fn elements_intersect(elem1: &Element, elem2: &Element) -> bool {
        // Simple bounding box intersection for now
        let (x1, y1, z1) = (elem1.position.x, elem1.position.y, elem1.position.z);
        let (x2, y2, z2) = (
            x1 + elem1.dimensions.x,
            y1 + elem1.dimensions.y,
            z1 + elem1.dimensions.z,
        );
        let (x3, y3, z3) = (elem2.position.x, elem2.position.y, elem2.position.z);
        let (x4, y4, z4) = (
            x3 + elem2.dimensions.x,
            y3 + elem2.dimensions.y,
            z3 + elem2.dimensions.z,
        );

        x1 < x4 && x2 > x3 && y1 < y4 && y2 > y3 && z1 < z4 && z2 > z3
    }

    /// Get elements within a certain distance of a point
    pub fn elements_near_point(elements: &[Element], point: Point, radius: f64) -> Vec<&Element> {
        let point_vec = Vector3::from(point);
        elements
            .iter()
            .filter(|elem| {
                let elem_pos = Vector3::from(elem.position);
                (elem_pos - point_vec).norm() <= radius
            })
            .collect()
    }

    /// Find walls that connect to a given wall
    pub fn connected_walls<'a>(elements: &'a [Element], wall: &Element) -> Vec<&'a Element> {
        elements
            .iter()
            .filter(|elem| {
                matches!(elem.element_type, ElementType::Wall)
                    && Self::elements_intersect(elem, wall)
            })
            .collect()
    }

    /// Create a room by combining walls (semantic operation)
    pub fn create_room(elements: &[Element]) -> Vec<&Element> {
        elements
            .iter()
            .filter(|elem| matches!(elem.element_type, ElementType::Wall))
            .collect()
    }

    /// Create an opening (door/window) by finding intersecting elements
    pub fn create_opening(wall: &Element, opening: &Element) -> bool {
        Self::elements_intersect(wall, opening)
    }
}
