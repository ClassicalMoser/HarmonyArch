/// Geometry validation functions
/// These functions are used to prevent degenerate geometries

/// Collinear validation functions
pub mod colinear;
/// Coplanar validation functions
pub mod coplanar;
/// Distinct validation functions
pub mod distinct;

pub use colinear::*;
pub use coplanar::*;
pub use distinct::*;
