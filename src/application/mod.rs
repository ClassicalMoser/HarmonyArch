/// Application layer for the application
use uuid::Uuid;

use crate::domain::Solid;

/// Selection module for the application
/// This module contains the logic for selecting geometry in the application
pub mod selection;

/// Triangulation module for converting polygons into renderable triangles
mod triangulation;

/// Mesh creation module for converting domain solids into Bevy meshes
mod mesh;

/// Cube creation utilities for the application layer
pub mod cuboid;

pub use cuboid::*;
pub use mesh::create_mesh_from_solid;

/// Create a new solid
pub fn new_solid() -> Solid {
    // Arbitrary solid for now
    let solid = Solid {
        id: Uuid::new_v4(),
        polygons: vec![],
    };
    solid
}
