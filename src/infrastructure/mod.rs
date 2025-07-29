//! Infrastructure layer - External concerns like file I/O and rendering

use crate::domain::Element;
use crate::application::GeometryService;
use std::io::{Write, Result};

/// STL file renderer
pub struct StlRenderer;

impl StlRenderer {
    /// Write elements to an STL file
    pub fn write_stl(elements: &[Element], filename: &str) -> Result<()> {
        use std::fs::File;
        
        let mut file = File::create(filename)?;
        
        // Write STL header
        file.write_all(b"solid HarmonyArch\n")?;
        
        // Write triangles for each element
        for element in elements {
            let triangles = GeometryService::element_to_triangles(element);
            for triangle in triangles {
                // Write triangle
                writeln!(file, "  facet normal {} {} {}", 
                    triangle.normal[0], triangle.normal[1], triangle.normal[2])?;
                writeln!(file, "    outer loop")?;
                for vertex in &triangle.vertices {
                    writeln!(file, "      vertex {} {} {}", vertex[0], vertex[1], vertex[2])?;
                }
                writeln!(file, "    endloop")?;
                writeln!(file, "  endfacet")?;
            }
        }
        
        // Write STL footer
        file.write_all(b"endsolid HarmonyArch\n")?;
        
        Ok(())
    }
} 