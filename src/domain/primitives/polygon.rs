/// Define the Polygon type and its registry
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::validate_distinct_ids;

/// A polygon in 3D space
pub struct Polygon {
    /// The unique identifier of the polygon
    pub id: Uuid,
    /// Reference to the segments of the polygon
    pub segments: Vec<Uuid>,
}

/// Create a new polygon
pub fn new_polygon(segment_ids: Vec<&Uuid>) -> Option<Polygon> {
    // Validate the segment IDs
    if !validate_distinct_ids(&segment_ids) {
        return None;
    }

    // TODO: Validate that the segments are coplanar

    // Copy the segment IDs to owned UUIDs
    let segments: Vec<Uuid> = segment_ids.iter().map(|id| **id).collect();

    // Create polygon with owned UUIDs
    let new_polygon = Polygon {
        id: Uuid::new_v4(),
        segments,
    };

    Some(new_polygon)
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
    pub fn create_and_store(&mut self, segment_ids: Vec<&Uuid>) -> Option<Uuid> {
        // 1. Declare the polygon
        let polygon = new_polygon(segment_ids);

        if polygon.is_none() {
            return None;
        }

        let polygon = polygon.expect("None failed to return in polygon storage");

        // 2. Store it in the registry (self is already mutably borrowed)
        let id = polygon.id.clone();
        self.polygons.insert(id, polygon);

        // 3. Return the ID of the stored polygon
        Some(id)
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
