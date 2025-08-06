use harmony_arch::application::new_model;
use harmony_arch::domain::{Element, Point};
use bevy::prelude::*;


fn main() {
    println!("HarmonyArch - Semantic Architectural Modeling Engine");
    println!("==================================================");
    
    // Create a new model using the application layer
    let model = new_model();
    
    println!("Created model: {} (ID: {})", model.name, model.id);
    println!("Model has {} elements", model.elements.len());
    
    // Demonstrate creating some elements
    let element1 = Element {
        id: "elem1".to_string(),
        name: "Wall".to_string(),
        position: Point { x: 0.0, y: 0.0, z: 0.0 },
    };
    
    let element2 = Element {
        id: "elem2".to_string(),
        name: "Window".to_string(),
        position: Point { x: 5.0, y: 0.0, z: 0.0 },
    };
    
    println!("\nExample elements:");
    println!("- {} at position ({}, {}, {})", 
             element1.name, element1.position.x, element1.position.y, element1.position.z);
    println!("- {} at position ({}, {}, {})", 
             element2.name, element2.position.x, element2.position.y, element2.position.z);
    
    println!("\nHarmonyArch is ready for architectural modeling!");

    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
