//! Domain layer - Core business logic and geometric primitives

use std::fmt;

/// 3D point in space
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 3D vector
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

/// An architectural element
#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub element_type: ElementType,
    pub position: Point,
    pub dimensions: Vector,
    pub normal: Vector,
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

pub fn element(id: String, element_type: ElementType, position: Point, dimensions: Vector) -> Element {
    Element {
        id,
        element_type,
        position,
        dimensions,
        normal: up_vector(), // Sensible default
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

// Higher-order function for composition
pub fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// Factory functions for specific element types
pub fn wall(id: String, position: Point, width: f64, height: f64) -> Element {
    element(id, ElementType::Wall, position, vector(width, height, 0.3))
}

pub fn door(id: String, position: Point, width: f64, height: f64) -> Element {
    element(id, ElementType::Door, position, vector(width, height, 0.1))
}

pub fn window(id: String, position: Point, width: f64, height: f64) -> Element {
    element(id, ElementType::Window, position, vector(width, height, 0.1))
}

// Function that transforms elements (like map in FP)
pub fn transform_element<F>(elem: Element, transform: F) -> Element 
where 
    F: Fn(Element) -> Element 
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