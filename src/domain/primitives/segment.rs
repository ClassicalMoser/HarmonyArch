/// Define the Segment type and its registry
use std::collections::HashMap;
use uuid::Uuid;

/// A segment in 3D space
///
/// Segments are unordered pairs of vertices - the order doesn't matter
/// for geometric relationships, only for rendering/display purposes.
pub struct Segment {
    /// The unique identifier of the segment
    pub id: Uuid,
    /// The two vertices that define the segment (unordered pair)
    /// Stored in normalized order (smaller UUID first) for consistent equality
    pub vertices: [Uuid; 2],
}

/// Create a new segment
///
/// Normalizes vertex order (smaller UUID first) for consistent storage
fn new_segment(vertex1: &Uuid, vertex2: &Uuid) -> Segment {
    // Normalize: always store smaller UUID first
    let vertices = if vertex1 < vertex2 {
        [*vertex1, *vertex2]
    } else {
        [*vertex2, *vertex1]
    };

    Segment {
        id: Uuid::new_v4(),
        vertices,
    }
}

impl Segment {
    /// Check if this segment contains the given vertex
    pub fn contains_vertex(&self, vertex_id: &Uuid) -> bool {
        self.vertices[0] == *vertex_id || self.vertices[1] == *vertex_id
    }

    /// Get both vertices as a tuple (for compatibility/rendering)
    /// Returns in the order they were originally provided (if needed)
    /// For geometric operations, use `vertices` field directly
    pub fn vertices_tuple(&self) -> (Uuid, Uuid) {
        (self.vertices[0], self.vertices[1])
    }

    /// Get the other vertex (given one vertex)
    /// Returns None if the vertex is not part of this segment
    pub fn other_vertex(&self, vertex_id: &Uuid) -> Option<Uuid> {
        if self.vertices[0] == *vertex_id {
            Some(self.vertices[1])
        } else if self.vertices[1] == *vertex_id {
            Some(self.vertices[0])
        } else {
            None
        }
    }
}

/// A registry of segments
pub struct SegmentRegistry {
    /// Unique identifier for the registry
    pub id: Uuid,
    /// The segments in the registry
    pub segments: HashMap<Uuid, Segment>,
}

impl SegmentRegistry {
    /// Create a new segment registry
    pub fn create_new() -> Self {
        Self {
            id: Uuid::new_v4(),
            segments: HashMap::new(),
        }
    }
}

impl SegmentRegistry {
    /// Declare, store, and return the ID of a segment
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, start_vertex: &Uuid, end_vertex: &Uuid) -> Uuid {
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
