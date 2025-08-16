/// Define the Polygon type and its registry
use std::collections::HashMap;
use uuid::Uuid;

/// A polygon in 3D space
pub struct Polygon {
    /// The unique identifier of the polygon
    pub id: Uuid,
    /// Reference to the segments of the polygon
    pub segments: Vec<Uuid>,
}

/// Create a new polygon
pub fn new_polygon(segment_ids: Vec<&Uuid>) -> Polygon {
    // Create polygon with owned UUIDs
    let new_polygon = Polygon {
        id: Uuid::new_v4(),
        segments: segment_ids.iter().map(|id| **id).collect(),
    };

    new_polygon
}

/// A registry of polygons
pub struct PolygonRegistry {
    /// Unique identifier for the registry
    pub id: Uuid,
    /// The polygons in the registry
    pub polygons: HashMap<Uuid, Polygon>,
}

impl PolygonRegistry {
    /// Create a new polygon registry
    pub fn create_new() -> Self {
        Self {
            id: Uuid::new_v4(),
            polygons: HashMap::new(),
        }
    }
}

impl PolygonRegistry {
    /// Declare, store, and return the ID of a polygon
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, segment_ids: Vec<&Uuid>) -> Uuid {
        // 1. Declare the polygon
        let polygon = new_polygon(segment_ids);

        // 2. Store it in the registry (self is already mutably borrowed)
        let id = polygon.id.clone();
        self.polygons.insert(id, polygon);

        // 3. Return the ID of the stored polygon
        id
    }

    /// Remove a polygon from the registry
    pub fn remove(&mut self, id: &Uuid) -> () {
        self.polygons.remove(id);
    }

    /// Get a reference to a polygon by ID
    pub fn get(&self, id: &Uuid) -> Option<&Polygon> {
        self.polygons.get(id)
    }

    /// Get a mutable reference to a polygon by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Polygon> {
        self.polygons.get_mut(id)
    }
}
