//! Infrastructure layer - External concerns like file I/O and rendering

use crate::domain::Element;
use std::io::{Write, Result};
use std::fs::File;

/// STL file renderer with simple 3D geometry
pub struct StlRenderer;

impl StlRenderer {
    /// Write elements to an STL file with actual 3D triangles
    pub fn write_stl(elements: &[Element], filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;

        // Write STL header
        file.write_all(b"solid HarmonyArch\n")?;

        // Generate actual triangles for each element
        for element in elements {
            Self::write_element_triangles(&mut file, element)?;
        }

        // Write STL footer
        file.write_all(b"endsolid HarmonyArch\n")?;

        Ok(())
    }

    /// Write triangles for a single element
    fn write_element_triangles(file: &mut File, element: &Element) -> Result<()> {
        // Convert element to 3D box with 12 triangles (6 faces × 2 triangles each)
        let triangles = Self::element_to_triangles(element);
        
        for triangle in triangles {
            Self::write_triangle(file, &triangle)?;
        }
        
        Ok(())
    }

    /// Convert element to 12 triangles forming a 3D box
    fn element_to_triangles(element: &Element) -> Vec<[nalgebra::Vector3<f64>; 3]> {
        // Convert from corner-based to center-based coordinates
        let center = nalgebra::Vector3::new(
            element.position.x + element.dimensions.x / 2.0,
            element.position.y + element.dimensions.y / 2.0, 
            element.position.z + element.dimensions.z / 2.0
        );
        
        let half_dim = nalgebra::Vector3::new(
            element.dimensions.x / 2.0,
            element.dimensions.y / 2.0,
            element.dimensions.z / 2.0
        );

        // Create rotation matrix around Z-axis
        let rotation_radians = element.rotation_degrees * std::f64::consts::PI / 180.0;
        let cos_rot = rotation_radians.cos();
        let sin_rot = rotation_radians.sin();
        
        let rotation_matrix = nalgebra::Matrix3::new(
            cos_rot, -sin_rot, 0.0,
            sin_rot,  cos_rot, 0.0,
            0.0,      0.0,    1.0,
        );

        // Generate the 8 vertices of the box (centered and rotated)
        let vertices = Self::centered_box_vertices(center, half_dim, &rotation_matrix);
        
        // Generate triangles for each face
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

    /// Generate the 8 vertices of a centered box (symmetric around origin, with rotation)
    fn centered_box_vertices(
        center: nalgebra::Vector3<f64>, 
        half_dim: nalgebra::Vector3<f64>,
        rotation_matrix: &nalgebra::Matrix3<f64>
    ) -> [nalgebra::Vector3<f64>; 8] {
        // Define the 8 vertices relative to center (before rotation)
        let relative_vertices = [
            nalgebra::Vector3::new(-half_dim.x, -half_dim.y, -half_dim.z), // v0: bottom-front-left
            nalgebra::Vector3::new( half_dim.x, -half_dim.y, -half_dim.z), // v1: bottom-front-right
            nalgebra::Vector3::new( half_dim.x,  half_dim.y, -half_dim.z), // v2: bottom-back-right
            nalgebra::Vector3::new(-half_dim.x,  half_dim.y, -half_dim.z), // v3: bottom-back-left
            nalgebra::Vector3::new(-half_dim.x, -half_dim.y,  half_dim.z), // v4: top-front-left
            nalgebra::Vector3::new( half_dim.x, -half_dim.y,  half_dim.z), // v5: top-front-right
            nalgebra::Vector3::new( half_dim.x,  half_dim.y,  half_dim.z), // v6: top-back-right
            nalgebra::Vector3::new(-half_dim.x,  half_dim.y,  half_dim.z), // v7: top-back-left
        ];
        
        // Apply rotation and translation to each vertex
        relative_vertices.map(|vertex| {
            center + rotation_matrix * vertex
        })
    }

    /// Write a single triangle to STL format
    fn write_triangle(file: &mut File, triangle: &[nalgebra::Vector3<f64>; 3]) -> Result<()> {
        // Calculate normal (cross product of two edges)
        let edge1 = triangle[1] - triangle[0];
        let edge2 = triangle[2] - triangle[0];
        let normal = edge1.cross(&edge2).normalize();

        // Write triangle in STL format
        writeln!(file, "  facet normal {} {} {}", normal.x, normal.y, normal.z)?;
        writeln!(file, "    outer loop")?;
        for vertex in triangle {
            writeln!(file, "      vertex {} {} {}", vertex.x, vertex.y, vertex.z)?;
        }
        writeln!(file, "    endloop")?;
        writeln!(file, "  endfacet")?;

        Ok(())
    }

    /// Write elements to an SVG file for 2D drawings
    pub fn write_svg(elements: &[Element], filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;

        // Write SVG header
        writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        writeln!(file, r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">"#)?;

        writeln!(file, r#"  <text x="10" y="30" font-family="Arial" font-size="16">"#)?;
        writeln!(file, r#"    Semantic Architectural Model"#)?;
        writeln!(file, r#"  </text>"#)?;

        for (i, element) in elements.iter().enumerate() {
            writeln!(file, r#"  <text x="10" y="{}" font-family="Arial" font-size="12">"#, 50 + i * 20)?;
            writeln!(file, r#"    {}: {:?} at {}"#, element.id, element.element_type, element.position)?;
            writeln!(file, r#"  </text>"#)?;
        }

        // Write SVG footer
        writeln!(file, r#"</svg>"#)?;

        Ok(())
    }
}