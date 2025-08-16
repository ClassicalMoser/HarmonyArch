/// Primitives for the domain layer
/// Primitives are the basic building blocks of the domain layer
/// They are used to create more complex objects

/// A point is a position in 3D space
pub mod point;

/// A vertex is a movable location index
pub mod vertex;

/// A segment is a connection between two vertices
pub mod segment;

/// A polygon is a closed coplanar shape
/// It is defined by a list of segments
pub mod polygon;

/// A solid is a watertight 3D object composed of polygons
pub mod solid;

pub use point::*;
pub use polygon::*;
pub use segment::*;
pub use solid::*;
pub use vertex::*;
