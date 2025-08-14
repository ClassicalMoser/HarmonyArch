/// Define the Solid type and its registry
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::polygon::Polygon;

/// A solid in 3D space
pub struct Solid {
    /// The unique identifier of the solid
    pub id: Uuid,
    /// Reference to the polygons of the solid
    pub polygons: Vec<Uuid>,
}

/// Create a new solid
pub fn new_solid(polygons: Vec<&Polygon>) -> Solid {
    let polygon_ids: Vec<Uuid> = polygons.iter().map(|p| p.id.clone()).collect();
    let new_solid = Solid {
        id: Uuid::new_v4(),
        polygons: polygon_ids,
    };
    new_solid
}

/// A registry of solids
pub struct SolidRegistry {
    /// The solids in the registry
    pub solids: HashMap<Uuid, Solid>,
}

impl Default for SolidRegistry {
    fn default() -> Self {
        Self {
            solids: HashMap::new(),
        }
    }
}

impl SolidRegistry {
    /// Declare, store, and return the ID of a solid
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, polygons: Vec<&Polygon>) -> Uuid {
        // 1. Declare the solid
        let solid = new_solid(polygons);

        // 2. Store it in the registry (self is already mutably borrowed)
        let id = solid.id;
        self.solids.insert(id, solid);

        // 3. Return the ID of the stored solid
        id
    }

    /// Remove a solid from the registry
    pub fn remove(&mut self, id: &Uuid) -> () {
        self.solids.remove(id);
    }

    /// Get a reference to a solid by ID
    pub fn get(&self, id: &Uuid) -> Option<&Solid> {
        self.solids.get(id)
    }

    /// Get a mutable reference to a solid by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Solid> {
        self.solids.get_mut(id)
    }
}
