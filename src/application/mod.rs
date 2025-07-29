//! Application layer - Use cases and business logic coordination

use crate::domain::{Element, ElementType};

/// Service for converting domain elements to geometric primitives
pub struct GeometryService;

impl GeometryService {
    /// Convert an element to triangles for rendering
    pub fn element_to_triangles(elem: &Element) -> Vec<stl_io::Triangle> {
        match elem.element_type {
            ElementType::Wall => Self::wall_to_triangles(elem),
            ElementType::Door => Self::door_to_triangles(elem),
            ElementType::Window => Self::window_to_triangles(elem),
            _ => Self::wall_to_triangles(elem), // Default to wall
        }
    }

    fn wall_to_triangles(elem: &Element) -> Vec<stl_io::Triangle> {
        let mut triangles = Vec::new();
        
        // Create a simple box for the wall (convert f64 to f32)
        let p1 = stl_io::Vector::new([elem.position.x as f32, elem.position.y as f32, elem.position.z as f32]);
        let p2 = stl_io::Vector::new([(elem.position.x + elem.dimensions.x) as f32, elem.position.y as f32, elem.position.z as f32]);
        let p3 = stl_io::Vector::new([(elem.position.x + elem.dimensions.x) as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.y) as f32]);
        let p4 = stl_io::Vector::new([elem.position.x as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.y) as f32]);
        let p5 = stl_io::Vector::new([elem.position.x as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.z) as f32]);
        let p6 = stl_io::Vector::new([(elem.position.x + elem.dimensions.x) as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.z) as f32]);
        let p7 = stl_io::Vector::new([(elem.position.x + elem.dimensions.x) as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.y + elem.dimensions.z) as f32]);
        let p8 = stl_io::Vector::new([elem.position.x as f32, elem.position.y as f32, (elem.position.z + elem.dimensions.y + elem.dimensions.z) as f32]);
        
        // Front face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 0.0, 1.0]),
            vertices: [p1, p2, p3],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 0.0, 1.0]),
            vertices: [p1, p3, p4],
        });
        
        // Back face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 0.0, -1.0]),
            vertices: [p5, p7, p6],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 0.0, -1.0]),
            vertices: [p5, p8, p7],
        });
        
        // Left face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([-1.0, 0.0, 0.0]),
            vertices: [p1, p4, p8],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([-1.0, 0.0, 0.0]),
            vertices: [p1, p8, p5],
        });
        
        // Right face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([1.0, 0.0, 0.0]),
            vertices: [p2, p6, p7],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([1.0, 0.0, 0.0]),
            vertices: [p2, p7, p3],
        });
        
        // Top face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 1.0, 0.0]),
            vertices: [p4, p3, p7],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, 1.0, 0.0]),
            vertices: [p4, p7, p8],
        });
        
        // Bottom face
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, -1.0, 0.0]),
            vertices: [p1, p6, p2],
        });
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector::new([0.0, -1.0, 0.0]),
            vertices: [p1, p5, p6],
        });
        
        triangles
    }

    fn door_to_triangles(elem: &Element) -> Vec<stl_io::Triangle> {
        // Similar to wall but thinner
        Self::wall_to_triangles(elem)
    }

    fn window_to_triangles(elem: &Element) -> Vec<stl_io::Triangle> {
        // Similar to wall but thinner
        Self::wall_to_triangles(elem)
    }
} 