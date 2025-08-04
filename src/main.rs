//! HarmonyArch - Main application entry point

use harmony_arch::composition::CompositionRoot;
use harmony_arch::domain::Element;
use eframe::egui;

struct HarmonyArchApp {
    elements: Vec<Element>,
}

impl Default for HarmonyArchApp {
    fn default() -> Self {
        let elements = CompositionRoot::create_sample_scene();
        
        // Export to STL file
        match CompositionRoot::export_to_stl(&elements, "output.stl") {
            Ok(_) => println!("STL file written successfully to output.stl"),
            Err(e) => println!("Error writing STL file: {e}"),
        }

        println!("Created {} elements", elements.len());
        
        Self { elements }
    }
}

impl eframe::App for HarmonyArchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HarmonyArch - Architectural Viewer");
            
            ui.separator();
            
            // Display scene information
            ui.label(format!("Scene contains {} elements", self.elements.len()));
            
            ui.separator();
            
            // Display each element
            for element in &self.elements {
                ui.group(|ui| {
                    ui.label(format!("Element: {}", element.id));
                    ui.label(format!("Type: {:?}", element.element_type));
                    ui.label(format!("Position: {}", element.position));
                    ui.label(format!("Dimensions: ({:.1}, {:.1}, {:.1})", 
                        element.dimensions.x, element.dimensions.y, element.dimensions.z));
                    ui.label(format!("Rotation: {:.1}°", element.rotation_degrees));
                });
            }
            
            ui.separator();
            
            // Add some controls
            if ui.button("Export STL").clicked() {
                match CompositionRoot::export_to_stl(&self.elements, "output.stl") {
                    Ok(_) => {
                        ui.label("STL exported successfully!");
                    },
                    Err(e) => {
                        ui.label(format!("Export failed: {e}"));
                    },
                }
            }
            
            if ui.button("Export SVG").clicked() {
                match CompositionRoot::export_to_svg(&self.elements, "output.svg") {
                    Ok(_) => {
                        ui.label("SVG exported successfully!");
                    },
                    Err(e) => {
                        ui.label(format!("Export failed: {e}"));
                    },
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "HarmonyArch",
        options,
        Box::new(|_cc| Box::new(HarmonyArchApp::default())),
    )
}
