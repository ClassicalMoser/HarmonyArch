//! Domain layer - Core business logic and geometric primitives

use nalgebra::Vector3;
use std::fmt;

/// 3D point in space (aligned with nalgebra Vector3)
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

/// 3D vector (aligned with nalgebra Vector3)
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Vector> for Vector3<f64> {
    fn from(vector: Vector) -> Self {
        Vector3::new(vector.x, vector.y, vector.z)
    }
}

impl From<Vector3<f64>> for Vector {
    fn from(vector: Vector3<f64>) -> Self {
        Vector {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }
}

/// Types of architectural elements
#[derive(Debug, Clone)]
pub enum ElementType {
    Wall,
    Door,
    Window,
    Column,
    Beam,
    Floor,
    Roof,
}

/// An architectural element with semantic properties
#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub element_type: ElementType,
    pub position: Point,
    pub dimensions: Vector,
    pub normal: Vector,
    pub rotation_degrees: f64, // Rotation around Z-axis (0° = north, 90° = east, etc.)
    pub material: Option<String>,
    pub properties: std::collections::HashMap<String, String>,
}

// Pure factory functions (FP style)
pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
}

pub fn origin() -> Point {
    point(0.0, 0.0, 0.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z }
}

pub fn up_vector() -> Vector {
    vector(0.0, 0.0, 1.0)
}

// Orientation constants (degrees)
pub const NORTH: f64 = 0.0;    // 0°
pub const EAST: f64 = 90.0;    // 90°
pub const SOUTH: f64 = 180.0;  // 180°
pub const WEST: f64 = 270.0;   // 270°

pub fn element(
    id: String,
    element_type: ElementType,
    position: Point,
    dimensions: Vector,
) -> Element {
    Element {
        id,
        element_type,
        position,
        dimensions,
        normal: up_vector(), // Sensible default
        rotation_degrees: NORTH, // Default facing north
        material: None,
        properties: std::collections::HashMap::new(),
    }
}

// Composable transformation functions
pub fn with_position(position: Point) -> impl Fn(Element) -> Element {
    move |mut elem: Element| {
        elem.position = position;
        elem
    }
}

pub fn with_dimensions(dimensions: Vector) -> impl Fn(Element) -> Element {
    move |mut elem: Element| {
        elem.dimensions = dimensions;
        elem
    }
}

pub fn with_normal(normal: Vector) -> impl Fn(Element) -> Element {
    move |mut elem: Element| {
        elem.normal = normal;
        elem
    }
}

pub fn with_rotation(rotation_degrees: f64) -> impl Fn(Element) -> Element {
    move |mut elem: Element| {
        elem.rotation_degrees = rotation_degrees;
        elem
    }
}

pub fn with_material(material: String) -> impl Fn(Element) -> Element {
    let material_clone = material.clone();
    move |mut elem: Element| {
        elem.material = Some(material_clone.clone());
        elem
    }
}

// Higher-order function for composition
pub fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// Factory functions for specific element types with rotation
pub fn wall(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    element(id, ElementType::Wall, position, vector(width, 0.3, height))
        .pipe(with_rotation(rotation_degrees))
}

pub fn door(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    element(id, ElementType::Door, position, vector(width, 0.1, height))
        .pipe(with_rotation(rotation_degrees))
}

pub fn window(id: String, position: Point, width: f64, height: f64, rotation_degrees: f64) -> Element {
    element(id, ElementType::Window, position, vector(width, 0.1, height))
        .pipe(with_rotation(rotation_degrees))
}

// Helper trait for functional composition
trait Pipe {
    fn pipe<F>(self, f: F) -> Element where F: Fn(Element) -> Element;
}

impl Pipe for Element {
    fn pipe<F>(self, f: F) -> Element where F: Fn(Element) -> Element {
        f(self)
    }
}

// Function that transforms elements (like map in FP)
pub fn transform_element<F>(elem: Element, transform: F) -> Element
where
    F: Fn(Element) -> Element,
{
    transform(elem)
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
