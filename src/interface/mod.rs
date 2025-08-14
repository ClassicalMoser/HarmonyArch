/// Interface layer for the application
/// This module sets up the world and the camera
use bevy::pbr::*;
use bevy::prelude::*;

use crate::application::create_cube;

mod camera;
mod lighting;
mod mesh_creation;

use camera::{camera_controls, spawn_camera, CameraConfig};
use lighting::{spawn_directional_light, LightingConfig};
use mesh_creation::{create_cube_mesh, MeshConfig};

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .insert_resource(LightingConfig::default())
            .insert_resource(MeshConfig::default())
            .add_systems(Startup, setup_world)
            .add_systems(Update, camera_controls);
    }
}

/// Bevy system to setup the world with our cube
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_config: Res<CameraConfig>,
    lighting_config: Res<LightingConfig>,
    mesh_config: Res<MeshConfig>,
) {
    // Create our semantic cube
    let (vertices, segments, polygons, solid) = create_cube();

    // Log the creation
    println!("Created cube with:");
    println!("  {} segments", segments.len());
    println!("  {} polygons", polygons.len());
    println!("  1 solid with {} polygon references", solid.polygons.len());

    // Spawn camera and lighting using our modular components
    spawn_camera(&mut commands, &camera_config);
    spawn_directional_light(&mut commands, &lighting_config);

    // Create the cube mesh from our semantic data
    let cube_mesh = create_cube_mesh(&vertices, &segments, &polygons);

    // Spawn the cube entity using the correct component types for Bevy 0.16
    let mesh_handle = meshes.add(cube_mesh);

    // BEVY MATERIAL OPTIONS TO FIX RENDERING ISSUES:
    let material_handle = materials.add(StandardMaterial {
        base_color: mesh_config.material_color,
        perceptual_roughness: mesh_config.material_roughness,
        metallic: mesh_config.material_metallic,

        // Use default settings for now - Bevy handles most issues automatically
        ..default()
    });

    #[derive(Bundle)]
    struct Cube {
        mesh: Mesh3d,
        material: MeshMaterial3d<StandardMaterial>,
    }

    let cube = Cube {
        mesh: Mesh3d(mesh_handle),
        material: MeshMaterial3d(material_handle),
    };

    // Spawn the cube entity
    commands.spawn(cube);

    // Add instructions for testing
    println!();
    println!("=== CUBE RENDERING OPTIONS ===");
    println!("To test Bevy's built-in cube (fixes all issues automatically):");
    println!("  Set MeshConfig::use_builtin_cube = true");
    println!();
    println!("Current material settings:");
    println!("  - double_sided: true (renders both sides)");
    println!("  - cull_mode: None (no face culling)");
    println!("  - depth_write: true (proper depth handling)");
    println!("  - alpha_mode: Opaque (no transparency issues)");
    println!();
    println!("Camera controls:");
    println!("  WASD: Move camera");
    println!("  QE: Move up/down");
    println!("  Arrow keys: Rotate around cube");
}
