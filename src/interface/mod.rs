/// Interface layer for the application
/// 
/// Bevy integration, graph library, 3D renderer
/// Synced to a modelling 
/// 
/// Define things like actors, objects, the world, the floor,

use bevy::{prelude::*};


/// A scene builder is a builder for a scene
pub struct SceneBuilder {
    /// The children of the scene
    pub children: Vec<SceneBuilder>,
    /// The name of the scene
    pub name: String,
    /// The transform of the scene
    pub transform: Transform,
    /// Whether the scene is visible
    pub visible: bool,
}