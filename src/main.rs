//! HarmonyArch - Main application entry point

use harmony_arch::composition::CompositionRoot;

fn main() {
    // Create a sample scene using the composition root
    let elements = CompositionRoot::create_sample_scene();

    // Export to STL file
    match CompositionRoot::export_to_stl(&elements, "output.stl") {
        Ok(_) => println!("STL file written successfully to output.stl"),
        Err(e) => println!("Error writing STL file: {e}"),
    }

    println!("Created {} elements", elements.len());
}
