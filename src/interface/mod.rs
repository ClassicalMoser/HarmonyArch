/// Interface layer for the application
/// This module sets up the world and the camera

use bevy::prelude::*;
use crate::application::create_cube;

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

/// Bevy system to setup the world with our cube
fn setup_world() {
    // Create our semantic cube
    let (registry, segments, polygons, solid) = create_cube();
    
    // For now, just log the creation (we'll add rendering later)
    println!("Created cube with:");
    println!("  {} points in registry", registry.points.len());
    println!("  {} segments", segments.len());
    println!("  {} polygons", polygons.len());
    println!("  1 solid with {} polygon references", solid.polygons.len());
}