//! Domain layer - Core business logic and geometric primitives

use nalgebra::Vector3;
use std::fmt;

/// 3D point in space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Point {
    /// Create a new point with the given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a point at the origin (0, 0, 0)
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<Point> for Vector3<f64> {
    fn from(point: Point) -> Self {
        Vector3::new(point.x, point.y, point.z)
    }
}

impl From<Vector3<f64>> for Point {
    fn from(vector: Vector3<f64>) -> Self {
        Point {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }
}

/// Types of architectural elements
#[derive(Debug, Clone)]
pub enum ElementType {
    /// Vertical wall element
    Wall,
    /// Doorway opening
    Door,
    /// Window opening
    Window,
    /// Vertical structural column
    Column,
    /// Horizontal structural beam
    Beam,
    /// Floor surface
    Floor,
    /// Roof surface
    Roof,
}

/// An architectural element with semantic properties
#[derive(Debug, Clone)]
pub struct Element {
    /// Unique identifier for the element
    pub id: String,
    /// Type of architectural element
    pub element_type: ElementType,
    /// Position in 3D space
    pub position: Point,
    /// Dimensions (width, depth, height)
    pub dimensions: Vector3<f64>,
    /// Surface normal vector
    pub normal: Vector3<f64>,
    /// Rotation around Z-axis (0° = north, 90° = east, etc.)
    pub rotation_degrees: f64,
    /// Optional material identifier
    pub material: Option<String>,
    /// Additional properties as key-value pairs
    pub properties: std::collections::HashMap<String, String>,
}

// Orientation constants (degrees)
/// North orientation (0°)
pub const NORTH: f64 = 0.0;
/// East orientation (90°)
pub const EAST: f64 = 90.0;
/// South orientation (180°)
pub const SOUTH: f64 = 180.0;
/// West orientation (270°)
pub const WEST: f64 = 270.0;

// Factory functions for specific element types
/// Create a wall element with the specified dimensions and orientation
pub fn wall(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    Element {
        id,
        element_type: ElementType::Wall,
        position,
        dimensions: Vector3::new(width, 0.3, height),
        normal: Vector3::new(0.0, 0.0, 1.0),
        rotation_degrees,
        material: None,
        properties: std::collections::HashMap::new(),
    }
}

/// Create a door element with the specified dimensions and orientation
pub fn door(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    Element {
        id,
        element_type: ElementType::Door,
        position,
        dimensions: Vector3::new(width, 0.1, height),
        normal: Vector3::new(0.0, 0.0, 1.0),
        rotation_degrees,
        material: None,
        properties: std::collections::HashMap::new(),
    }
}

/// Create a window element with the specified dimensions and orientation
pub fn window(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    Element {
        id,
        element_type: ElementType::Window,
        position,
        dimensions: Vector3::new(width, 0.1, height),
        normal: Vector3::new(0.0, 0.0, 1.0),
        rotation_degrees,
        material: None,
        properties: std::collections::HashMap::new(),
    }
}

// Convenience functions
/// Create a point with the given coordinates
pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

/// Create a point at the origin (0, 0, 0)
pub fn origin() -> Point {
    Point::origin()
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
