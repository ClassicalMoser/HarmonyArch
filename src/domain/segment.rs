/// Define the Segment type and its registry
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::vertex::Vertex;

/// A segment in 3D space
pub struct Segment {
    /// The unique identifier of the segment
    pub id: Uuid,
    /// Reference to the start point of the segment
    pub start_vertex: Uuid,
    /// Reference to the end point of the segment
    pub end_vertex: Uuid,
}

/// Create a new segment
pub fn new_segment(start_vertex: &Vertex, end_vertex: &Vertex) -> Segment {
    let new_segment = Segment {
        id: Uuid::new_v4(),
        start_vertex: start_vertex.id,
        end_vertex: end_vertex.id,
    };
    new_segment
}

/// A registry of segments
pub struct SegmentRegistry {
    /// The segments in the registry
    pub segments: HashMap<Uuid, Segment>,
}

impl Default for SegmentRegistry {
    fn default() -> Self {
        Self {
            segments: HashMap::new(),
        }
    }
}

impl SegmentRegistry {
    /// Declare, store, and return the ID of a segment
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, start_vertex: &Vertex, end_vertex: &Vertex) -> Uuid {
        // 1. Declare the segment
        let segment = new_segment(start_vertex, end_vertex);

        // 2. Store it in the registry (self is already mutably borrowed)
        let id = segment.id.clone();
        self.segments.insert(id, segment);

        // 3. Return the ID of the stored segment
        id
    }

    /// Remove a segment from the registry
    pub fn remove(&mut self, id: &Uuid) -> () {
        self.segments.remove(id);
    }

    /// Get a reference to a segment by ID
    pub fn get(&self, id: &Uuid) -> Option<&Segment> {
        self.segments.get(id)
    }

    /// Get a mutable reference to a segment by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Segment> {
        self.segments.get_mut(id)
    }
}
