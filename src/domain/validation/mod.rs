/// Geometry validation functions
/// These functions are used to prevent degenerate geometries

/// Coplanar validation functions
pub mod coplanar;
/// Distinct validation functions
pub mod distinct;

pub use coplanar::*;
pub use distinct::*;
