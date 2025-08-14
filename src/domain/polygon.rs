/// Define the Polygon type and its registry
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::segment::Segment;

/// A polygon in 3D space
pub struct Polygon {
    /// The unique identifier of the polygon
    pub id: Uuid,
    /// Reference to the segments of the polygon
    pub segments: Vec<Uuid>,
}

/// Create a new polygon
pub fn new_polygon(segments: Vec<&Segment>) -> Polygon {
    let segment_ids: Vec<Uuid> = segments.iter().map(|s| s.id.clone()).collect();
    let new_polygon = Polygon {
        id: Uuid::new_v4(),
        segments: segment_ids,
    };
    new_polygon
}

/// A registry of polygons
pub struct PolygonRegistry {
    /// The polygons in the registry
    pub polygons: HashMap<Uuid, Polygon>,
}

impl Default for PolygonRegistry {
    fn default() -> Self {
        Self {
            polygons: HashMap::new(),
        }
    }
}

impl PolygonRegistry {
    /// Declare, store, and return the ID of a polygon
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, segments: Vec<&Segment>) -> Uuid {
        // 1. Declare the polygon
        let polygon = new_polygon(segments);

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
