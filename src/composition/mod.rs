/// Composition layer for the application

use crate::domain::{Model, Element, Point};
use crate::application::new_model;

/// Create a sample scene
pub fn create_sample_scene() -> Model {
    // Create a new model
    let mut model = new_model();
    // Add an element to the model
    model.elements.push(Element {
        id: "1".to_string(),
        name: "Element 1".to_string(),
        position: Point { x: 0.0, y: 0.0, z: 0.0 },
    });
    model
}   