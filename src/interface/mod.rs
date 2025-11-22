/// Interface layer for the application
/// This module sets up the world and the camera
use bevy::pbr::*;
use bevy::prelude::*;

use crate::application::{create_mesh_from_solid, create_rectangular_solid};
use crate::domain::GeometryRegistry;

mod camera;
mod lighting;
mod mesh_creation;
mod segment_outlines;
mod ui;

use camera::{camera_controls, spawn_camera, update_camera_projection, CameraConfig};
use lighting::spawn_lights;
use mesh_creation::MeshConfig;
use segment_outlines::{render_segment_outlines_2d, GeometryRegistryResource};
use ui::{handle_ui_interactions, setup_ui, toggle_mesh_visibility, ToggleableMesh, update_button_appearance, UiState};

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .insert_resource(MeshConfig::default())
            .insert_resource(UiState::default())
            .add_systems(Startup, (setup_world, setup_ui))
            .add_systems(
                Update,
                (
                    camera_controls,
                    render_segment_outlines_2d,
                    handle_ui_interactions,
                    update_button_appearance,
                    toggle_mesh_visibility,
                    update_camera_projection,
                ),
            );
    }
}

/// Bevy system to setup the world with our cube
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_config: Res<CameraConfig>,
    mesh_config: Res<MeshConfig>,
) {
    // Create domain registries
    let mut geometry_registry = GeometryRegistry::create_new();

    // Create domain objects for the first cube
    let solid_id1 = create_rectangular_solid(2.0, 2.5, 3.5, &mut geometry_registry);
    
    // Create domain objects for the second cube
    let solid_id2 = create_rectangular_solid(1.5, 2.0, 2.5, &mut geometry_registry);

    // Extract information and create meshes
    let (solid1, mesh_handle1) = {
        let solid = geometry_registry
            .solids
            .get(&solid_id1)
            .expect("Failed to get solid from registry");
        let mesh = create_mesh_from_solid(&solid, &geometry_registry);
        let mesh_handle = meshes.add(mesh);
        (solid.id, mesh_handle)
    };

    let (solid2, mesh_handle2) = {
        let solid = geometry_registry
            .solids
            .get(&solid_id2)
            .expect("Failed to get solid from registry");
        let mesh = create_mesh_from_solid(&solid, &geometry_registry);
        let mesh_handle = meshes.add(mesh);
        (solid.id, mesh_handle)
    };

    // Store geometry registry for 2D overlay rendering
    commands.insert_resource(GeometryRegistryResource {
        registry: geometry_registry,
    });

    // Create materials with different colors
    let material1 = StandardMaterial {
        base_color: mesh_config.material_color,
        perceptual_roughness: mesh_config.material_roughness,
        metallic: mesh_config.material_metallic,
        ..Default::default()
    };
    let material_handle1 = materials.add(material1);

    let material2 = StandardMaterial {
        base_color: Color::srgba(0.2, 0.4, 0.8, 1.0), // Blue color
        perceptual_roughness: mesh_config.material_roughness,
        metallic: mesh_config.material_metallic,
        ..Default::default()
    };
    let material_handle2 = materials.add(material2);

    // Spawn the first cube entity, offset to the left
    commands.spawn((
        Mesh3d(mesh_handle1),
        MeshMaterial3d(material_handle1),
        Transform::from_xyz(-2.0, 0.0, 1.0),
        ToggleableMesh,
    ));

    // Spawn the second cube entity, offset to the right
    commands.spawn((
        Mesh3d(mesh_handle2),
        MeshMaterial3d(material_handle2),
        Transform::from_xyz(2.0, 0.0, -1.0),
        ToggleableMesh,
    ));

    // Spawn camera and lighting
    spawn_lights(&mut commands);
    spawn_camera(&mut commands, &camera_config);

    println!("Created cubes from domain objects:");
    println!("  Solid 1 ID: {}", solid1);
    println!("  Solid 2 ID: {}", solid2);
}
