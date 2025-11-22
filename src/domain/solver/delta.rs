/// Delta tracking and propagation
/// 
/// Tracks changes to geometry that result from constraint application.
/// Deltas cascade through the constraint system until convergence.

use crate::domain::Point;
use uuid::Uuid;

/// A single change to geometry
/// 
/// Represents a change that needs to be applied, typically a vertex
/// position change that results from constraint application.
#[derive(Debug, Clone)]
pub struct Delta {
    /// The vertex that changed
    pub vertex_id: Uuid,
    /// The old position
    pub old_position: Point,
    /// The new position
    pub new_position: Point,
}

/// Collection of deltas from constraint application
/// 
/// Tracks all geometry changes that result from applying constraints.
#[derive(Debug, Clone, Default)]
pub struct DeltaSet {
    /// The deltas that need to be applied
    pub deltas: Vec<Delta>,
}

impl DeltaSet {
    /// Create a new empty delta set
    pub fn new() -> Self {
        Self { deltas: Vec::new() }
    }

    /// Add a delta to the set
    pub fn add(&mut self, delta: Delta) {
        self.deltas.push(delta);
    }

    /// Check if there are any deltas
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }

    /// Get the number of deltas
    pub fn len(&self) -> usize {
        self.deltas.len()
    }
}

/// Tracks which geometry entities are affected by changes
/// 
/// Used to determine what needs to be re-evaluated when deltas are applied.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    // TODO: Implement dependency tracking
    // Maps vertex_id -> affected segment_ids -> affected polygon_ids
}

