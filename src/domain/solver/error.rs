/// Constraint errors and conflicts
/// 
/// Defines error types for constraint solving failures.

use uuid::Uuid;

/// Errors that can occur during constraint solving
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintError {
    /// Geometry violates parent tier boundary
    /// 
    /// Contains the geometry ID that violated the boundary.
    BoundaryViolation {
        /// The geometry entity that violated the boundary
        geometry_id: Uuid,
        /// Description of the violation
        message: String,
    },
    /// Constraints conflict with each other
    /// 
    /// Multiple constraints cannot be satisfied simultaneously.
    ConstraintConflict {
        /// The conflicting constraints
        conflicting_constraints: Vec<String>,
        /// Description of the conflict
        message: String,
    },
    /// Delta loop exceeded maximum iterations
    /// 
    /// Constraint application created a cycle of changes that didn't converge.
    DeltaLoopLimit {
        /// Number of iterations attempted
        iterations: usize,
        /// Description of the issue
        message: String,
    },
    /// Geometry not found in registry
    GeometryNotFound {
        /// The missing geometry ID
        geometry_id: Uuid,
    },
    /// Invalid constraint configuration
    InvalidConfiguration {
        /// Description of the invalid configuration
        message: String,
    },
}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintError::BoundaryViolation { geometry_id, message } => {
                write!(f, "Boundary violation for geometry {}: {}", geometry_id, message)
            }
            ConstraintError::ConstraintConflict {
                conflicting_constraints,
                message,
            } => {
                write!(
                    f,
                    "Constraint conflict: {}. Conflicting constraints: {:?}",
                    message, conflicting_constraints
                )
            }
            ConstraintError::DeltaLoopLimit { iterations, message } => {
                write!(
                    f,
                    "Delta loop limit exceeded after {} iterations: {}",
                    iterations, message
                )
            }
            ConstraintError::GeometryNotFound { geometry_id } => {
                write!(f, "Geometry not found: {}", geometry_id)
            }
            ConstraintError::InvalidConfiguration { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
        }
    }
}

impl std::error::Error for ConstraintError {}

