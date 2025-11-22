/// Constraint solving system for HarmonyArch
/// 
/// This module implements deterministic constraint solving through ordered application
/// of geometric and architectural constraints. Constraints are applied in a specific
/// order (see ORDER.md), with deltas cascading through the system until convergence
/// or conflict.
/// 
/// The system is tier-aware and supports opt-out constraints (plumb, level, orthogonal)
/// that apply by default but can be explicitly disabled.

/// Core constraint types and definitions
pub mod types;

/// Constraint application context (tier-aware settings)
pub mod context;

/// Main constraint solver (orchestrates constraint application)
pub mod apply_solve;

/// Delta tracking and propagation
pub mod delta;

/// Boundary enforcement (parent-child relationship)
pub mod boundary;

/// Constraint errors and conflicts
pub mod error;

/// Individual constraint implementations
pub mod constraints;

pub use types::*;
pub use context::*;
pub use apply_solve::*;
pub use delta::*;
pub use boundary::*;
pub use error::*;
pub use constraints::*;

