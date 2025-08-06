/// Domain layer for the application

/// A model is a collection of elements
pub struct Model {
    /// The unique identifier of the model
    pub id: String,
    /// The name of the model
    pub name: String,
    /// The elements of the model
    pub elements: Vec<Element>,
}

/// An element is a part of a model
pub struct Element {
    /// The unique identifier of the element
    pub id: String,
    /// The name of the element
    pub name: String,
    /// The position of the element
    pub position: Point,
}

/// A point in 3D space
pub struct Point {
    /// The x coordinate of the point
    pub x: f32,
    /// The y coordinate of the point
    pub y: f32,
    /// The z coordinate of the point
    pub z: f32,
}