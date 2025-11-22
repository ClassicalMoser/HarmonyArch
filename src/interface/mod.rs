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

use camera::{camera_controls, spawn_camera, CameraConfig};
use lighting::spawn_lights;
use mesh_creation::MeshConfig;
use segment_outlines::{render_segment_outlines_2d, GeometryRegistryResource};

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .insert_resource(MeshConfig::default())
            .add_systems(Startup, setup_world)
            .add_systems(Update, (camera_controls, render_segment_outlines_2d));
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

    // Create domain objects for the cube
    let solid_id = create_rectangular_solid(2.0, 2.5, 3.5, &mut geometry_registry);

    // Extract information before moving geometry_registry
    let (solid_id_for_log, polygon_count, segment_count, vertex_count, mesh_handle) = {
        let solid = geometry_registry
            .solids
            .get(&solid_id)
            .expect("Failed to get solid from registry");

        // Generate mesh from domain objects
        let mesh = create_mesh_from_solid(&solid, &geometry_registry);
        let mesh_handle = meshes.add(mesh);

        // Extract values for logging
        (
            solid.id,
            solid.polygons.len(),
            geometry_registry.segments.segments.len(),
            geometry_registry.vertices.vertices.len(),
            mesh_handle,
        )
    };

    // Store geometry registry for 2D overlay rendering
    commands.insert_resource(GeometryRegistryResource {
        registry: geometry_registry,
    });

    // Create material
    let material = StandardMaterial {
        base_color: mesh_config.material_color,
        perceptual_roughness: mesh_config.material_roughness,
        metallic: mesh_config.material_metallic,
        ..Default::default()
    };
    let material_handle = materials.add(material);

    // Spawn the cube entity
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn camera and lighting
    spawn_lights(&mut commands);
    spawn_camera(&mut commands, &camera_config);

    println!("Created cube from domain objects:");
    println!("  Solid ID: {}", solid_id_for_log);
    println!("  Polygons: {}", polygon_count);
    println!("  Segments: {}", segment_count);
    println!("  Vertices: {}", vertex_count);
}
