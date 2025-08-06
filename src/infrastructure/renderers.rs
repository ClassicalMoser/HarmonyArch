//! Renderers for different output formats

use crate::domain::Element;
use crate::infrastructure::{GeometryUtils, FileWriter};
use std::io::Result;
use std::fs::File;
use nalgebra::Vector3;

/// STL file renderer for 3D geometry
pub struct StlRenderer;

impl StlRenderer {
    /// Write elements to an STL file with actual 3D triangles
    pub fn write_stl(elements: &[Element], filename: &str) -> Result<()> {
        let mut file = FileWriter::create_file(filename)?;
        
        Self::write_header(&mut file)?;
        
        for element in elements {
            Self::write_element_triangles(&mut file, element)?;
        }
        
        Self::write_footer(&mut file)?;
        
        Ok(())
    }

    /// Write STL file header
    fn write_header(file: &mut File) -> Result<()> {
        FileWriter::write_bytes(file, b"solid HarmonyArch\n")
    }

    /// Write STL file footer
    fn write_footer(file: &mut File) -> Result<()> {
        FileWriter::write_bytes(file, b"endsolid HarmonyArch\n")
    }

    /// Write triangles for a single element
    fn write_element_triangles(file: &mut File, element: &Element) -> Result<()> {
        let triangles = GeometryUtils::element_to_triangles(element);
        
        for triangle in triangles {
            Self::write_triangle(file, &triangle)?;
        }
        
        Ok(())
    }

    /// Write a single triangle to STL format
    fn write_triangle(file: &mut File, triangle: &[Vector3<f64>; 3]) -> Result<()> {
        let normal = GeometryUtils::calculate_triangle_normal(triangle);

        FileWriter::write_line(file, &format!("  facet normal {} {} {}", normal.x, normal.y, normal.z))?;
        FileWriter::write_line(file, "    outer loop")?;
        
        for vertex in triangle {
            FileWriter::write_line(file, &format!("      vertex {} {} {}", vertex.x, vertex.y, vertex.z))?;
        }
        
        FileWriter::write_line(file, "    endloop")?;
        FileWriter::write_line(file, "  endfacet")?;

        Ok(())
    }
}

/// SVG file renderer for 2D visualization
pub struct SvgRenderer;

impl SvgRenderer {
    /// Write elements to an SVG file for 2D drawings
    pub fn write_svg(elements: &[Element], filename: &str) -> Result<()> {
        let mut file = FileWriter::create_file(filename)?;
        
        Self::write_header(&mut file)?;
        Self::write_title(&mut file)?;
        Self::write_elements(&mut file, elements)?;
        Self::write_footer(&mut file)?;
        
        Ok(())
    }

    /// Write SVG file header
    fn write_header(file: &mut File) -> Result<()> {
        FileWriter::write_line(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        FileWriter::write_line(file, r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">"#)
    }

    /// Write SVG title
    fn write_title(file: &mut File) -> Result<()> {
        FileWriter::write_line(file, r#"  <text x="10" y="30" font-family="Arial" font-size="16">"#)?;
        FileWriter::write_line(file, r"    Semantic Architectural Model")?;
        FileWriter::write_line(file, r"  </text>")
    }

    /// Write element information
    fn write_elements(file: &mut File, elements: &[Element]) -> Result<()> {
        for (i, element) in elements.iter().enumerate() {
            FileWriter::write_line(file, &format!(r#"  <text x="10" y="{}" font-family="Arial" font-size="12">"#, 50 + i * 20))?;
            FileWriter::write_line(file, &format!(r"    {}: {:?} at {}", element.id, element.element_type, element.position))?;
            FileWriter::write_line(file, r"  </text>")?;
        }
        Ok(())
    }

    /// Write SVG file footer
    fn write_footer(file: &mut File) -> Result<()> {
        FileWriter::write_line(file, r"</svg>")
    }
} 