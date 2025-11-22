/// Individual constraint implementations
/// 
/// Each constraint is implemented as a pure function that:
/// - Takes geometry and context
/// - Returns deltas (what needs to change) or errors
/// - Does not mutate geometry directly

/// Coincident constraint (Rare)
pub mod coincident;

/// Collinear constraint (Projection)
pub mod collinear;

/// Coplanar constraint (Projection)
pub mod coplanar;

/// Boundary constraint
pub mod boundary_constraint;

/// Equilateral constraint
pub mod equilateral;

/// Equiangular constraint
pub mod equiangular;

/// Plumb constraint (Opt-out)
pub mod plumb;

/// Level constraint (Opt-out)
pub mod level;

/// Orthogonal constraint (Opt-out)
pub mod orthogonal;

pub use coincident::*;
pub use collinear::*;
pub use coplanar::*;
pub use boundary_constraint::*;
pub use equilateral::*;
pub use equiangular::*;
pub use plumb::*;
pub use level::*;
pub use orthogonal::*;

