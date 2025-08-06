/// Application layer for the application

use crate::domain::Model;

/// Create a new model
pub fn new_model() -> Model {
    // Arbitrary model for now
    let model = Model {
        id: "1".to_string(),
        name: "Model 1".to_string(),
        elements: vec![],
    };
    model
}
