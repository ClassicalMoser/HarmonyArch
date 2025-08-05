//! Geometry utilities for 3D transformations and calculations

use crate::domain::Element;
use nalgebra::{Vector3, Matrix3};

/// Utility functions for 3D geometry operations
pub struct GeometryUtils;

impl GeometryUtils {
    /// Convert element to 12 triangles forming a 3D box
    pub fn element_to_triangles(element: &Element) -> Vec<[Vector3<f64>; 3]> {
        let center = Self::calculate_center(element);
        let half_dim = Self::calculate_half_dimensions(element);
        let rotation_matrix = Self::create_rotation_matrix(element.rotation_degrees);
        let vertices = Self::centered_box_vertices(center, half_dim, &rotation_matrix);
        
        Self::generate_box_triangles(&vertices)
    }

    /// Calculate the center point of an element
    fn calculate_center(element: &Element) -> Vector3<f64> {
        Vector3::new(
            element.position.x + element.dimensions.x / 2.0,
            element.position.y + element.dimensions.y / 2.0, 
            element.position.z + element.dimensions.z / 2.0
        )
    }

    /// Calculate half dimensions for centered box
    fn calculate_half_dimensions(element: &Element) -> Vector3<f64> {
        Vector3::new(
            element.dimensions.x / 2.0,
            element.dimensions.y / 2.0,
            element.dimensions.z / 2.0
        )
    }

    /// Create rotation matrix around Z-axis
    fn create_rotation_matrix(rotation_degrees: f64) -> Matrix3<f64> {
        let rotation_radians = rotation_degrees * std::f64::consts::PI / 180.0;
        let cos_rot = rotation_radians.cos();
        let sin_rot = rotation_radians.sin();
        
        Matrix3::new(
            cos_rot, -sin_rot, 0.0,
            sin_rot,  cos_rot, 0.0,
            0.0,      0.0,    1.0,
        )
    }

    /// Generate the 8 vertices of a centered box (symmetric around origin, with rotation)
    fn centered_box_vertices(
        center: Vector3<f64>, 
        half_dim: Vector3<f64>,
        rotation_matrix: &Matrix3<f64>
    ) -> [Vector3<f64>; 8] {
        // Define the 8 vertices relative to center (before rotation)
        let relative_vertices = [
            Vector3::new(-half_dim.x, -half_dim.y, -half_dim.z), // v0: bottom-front-left
            Vector3::new( half_dim.x, -half_dim.y, -half_dim.z), // v1: bottom-front-right
            Vector3::new( half_dim.x,  half_dim.y, -half_dim.z), // v2: bottom-back-right
            Vector3::new(-half_dim.x,  half_dim.y, -half_dim.z), // v3: bottom-back-left
            Vector3::new(-half_dim.x, -half_dim.y,  half_dim.z), // v4: top-front-left
            Vector3::new( half_dim.x, -half_dim.y,  half_dim.z), // v5: top-front-right
            Vector3::new( half_dim.x,  half_dim.y,  half_dim.z), // v6: top-back-right
            Vector3::new(-half_dim.x,  half_dim.y,  half_dim.z), // v7: top-back-left
        ];
        
        // Apply rotation and translation to each vertex
        relative_vertices.map(|vertex| {
            center + rotation_matrix * vertex
        })
    }

    /// Generate triangles for each face of the box
    fn generate_box_triangles(vertices: &[Vector3<f64>; 8]) -> Vec<[Vector3<f64>; 3]> {
        let mut triangles = Vec::new();
        
        // Define face vertex indices (each face is 2 triangles)
        let faces = [
            // Front face: v0, v1, v4, v5
            ([0, 1, 4], [1, 5, 4]),
            // Back face: v2, v3, v6, v7  
            ([2, 3, 6], [3, 7, 6]),
            // Left face: v3, v0, v7, v4
            ([3, 0, 7], [0, 4, 7]),
            // Right face: v1, v2, v5, v6
            ([1, 2, 5], [2, 6, 5]),
            // Bottom face: v3, v2, v0, v1
            ([3, 2, 0], [2, 1, 0]),
            // Top face: v4, v5, v7, v6
            ([4, 5, 7], [5, 6, 7]),
        ];
        
        // Generate triangles for each face
        for (tri1, tri2) in faces {
            triangles.push([
                vertices[tri1[0]],
                vertices[tri1[1]], 
                vertices[tri1[2]]
            ]);
            triangles.push([
                vertices[tri2[0]],
                vertices[tri2[1]], 
                vertices[tri2[2]]
            ]);
        }
        
        triangles
    }

    /// Calculate normal vector for a triangle
    pub fn calculate_triangle_normal(triangle: &[Vector3<f64>; 3]) -> Vector3<f64> {
        let edge1 = triangle[1] - triangle[0];
        let edge2 = triangle[2] - triangle[0];
        edge1.cross(&edge2).normalize()
    }
} 