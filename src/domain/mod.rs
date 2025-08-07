/// Domain layer for the application
/// Pure domain logic, no external dependencies, no ECS, no Bevy
use uuid::Uuid;
use std::collections::HashMap;

/// meters per unit
pub const METERS_PER_UNIT: f32 = 1.0;

/// Create the origin point of the world
pub fn create_origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}


/// A position point in meters 3D space.
/// Points are less precise than distances.
/// Points can be moved in space.
#[derive(Clone, Debug)]
pub struct Point {
    /// The east coordinate of the point in meters.
    /// Positive values are to the east.
    pub x: f32,
    /// The north coordinate of the point in meters.
    /// Positive values are to the north.
    pub y: f32,
    /// The height coordinate of the point in meters.
    /// Positive values are up.
    pub z: f32,
}

impl Point {
    /// Move a point in space
    pub fn move_by_vector(&mut self, distance: &Vector) -> () {
        self.x += distance.x as f32;
        self.y += distance.y as f32;
        self.z += distance.z as f32;
    }

    /// Move a point to a defined position
    pub fn move_to_position(&mut self, position: &Point)  -> () {
        self.x = position.x as f32;
        self.y = position.y as f32;
        self.z = position.z as f32;
    }
}



/// A vertex in 3D space
pub struct Vertex {
    /// The unique identifier of the vertex
    pub id: Uuid,
    /// The position of the vertex
    pub position: Point,
}

/// Create a new vertex
pub fn new_vertex(position: &Point) -> Vertex {
    Vertex {
        id: Uuid::new_v4(),
        position: position.clone(),
    }
}

/// A registry of points
pub struct VertexRegistry {
    /// The vertices in the registry
    pub vertices: HashMap<Uuid, Vertex>,
}

/// A distance in meters 3D space.
/// Distances are more precise than points
pub struct Vector {
    /// The east component of the distance in meters.
    /// Positive values are to the east.
    pub x: f64,
    /// The north component of the distance in meters.
    /// Positive values are to the north.
    pub y: f64,
    /// The height component of the distance in meters.
    /// Positive values are up.
    pub z: f64,
}

/// Create a new distance
pub fn measure_vector(start_point: &Point, end_point: &Point) -> Vector {
    Vector {
        x: end_point.x as f64 - start_point.x as f64,
        y: end_point.y as f64 - start_point.y as f64,
        z: end_point.z as f64 - start_point.z as f64,
    }
}

/// A segment in 3D space
pub struct Segment {
    /// The unique identifier of the segment
    pub id: Uuid,
    /// Reference to the start point of the segment
    pub start_vertex: Uuid,
    /// Reference to the end point of the segment
    pub end_vertex: Uuid,
}

/// Create a new segment
pub fn new_segment(start_vertex: &Vertex, end_vertex: &Vertex) -> Segment {
    Segment {
        id: Uuid::new_v4(),
        start_vertex: start_vertex.id,
        end_vertex: end_vertex.id,
    }
}


/// A polygon in 3D space
pub struct Polygon {
    /// The unique identifier of the polygon
    pub id: Uuid,
    /// Reference to the segments of the polygon
    pub segments: Vec<Uuid>,
}

/// Create a new polygon
pub fn new_polygon(segments: Vec<Uuid>) -> Polygon {
    Polygon {
        id: Uuid::new_v4(),
        segments,
    }
}

/// A solid in 3D space
pub struct Solid {
    /// The unique identifier of the solid
    pub id: Uuid,
    /// Reference to the polygons of the solid
    pub polygons: Vec<Uuid>,
}

/// Create a new solid
pub fn new_solid(polygons: Vec<Uuid>) -> Solid {
    Solid {
        id: Uuid::new_v4(),
        polygons,
    }
}