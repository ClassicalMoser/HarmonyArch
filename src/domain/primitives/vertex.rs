/// Define the Vertex type and its registry
use crate::domain::Point;
use std::collections::HashMap;
use uuid::Uuid;

/// A vertex in 3D space
pub struct Vertex {
    /// The unique identifier of the vertex
    pub id: Uuid,
    /// The position of the vertex
    pub position: Point,
}

/// Create a new vertex
pub fn new_vertex(position: Point) -> Vertex {
    let new_vertex = Vertex {
        id: Uuid::new_v4(),
        position: position,
    };
    new_vertex
}

/// A registry of vertices
pub struct VertexRegistry {
    /// The vertices in the registry
    pub vertices: HashMap<Uuid, Vertex>,
}

impl Default for VertexRegistry {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }
}

impl VertexRegistry {
    /// Declare, store, and return the ID of a vertex
    /// This method handles all three operations in one call
    pub fn create_and_store(&mut self, position: Point) -> Uuid {
        // 1. Declare the vertex
        let vertex = new_vertex(position);

        // 2. Store it in the registry (self is already mutably borrowed)
        let id = vertex.id;
        self.vertices.insert(id, vertex);

        // 3. Return the ID of the stored vertex
        id
    }

    /// Remove a vertex from the registry
    pub fn remove(&mut self, id: &Uuid) -> () {
        self.vertices.remove(id);
    }

    /// Get a reference to a vertex by ID
    pub fn get(&self, id: &Uuid) -> Option<&Vertex> {
        self.vertices.get(id)
    }

    /// Get a mutable reference to a vertex by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Vertex> {
        self.vertices.get_mut(id)
    }
}
