/// Interface layer for the application
/// This module sets up the world and the camera
use bevy::pbr::*;
use bevy::prelude::*;

use crate::application::{create_mesh_from_solid, create_rectangular_solid};
use crate::domain::{PolygonRegistry, SegmentRegistry, SolidRegistry, VertexRegistry};

mod camera;
mod lighting;
mod mesh_creation;

use camera::{camera_controls, spawn_camera, CameraConfig};
use lighting::spawn_lights;
use mesh_creation::MeshConfig;

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
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
    mesh_config: Res<MeshConfig>,
) {
    // Create domain registries
    let mut vertex_registry = VertexRegistry::default();
    let mut segment_registry = SegmentRegistry::default();
    let mut polygon_registry = PolygonRegistry::default();
    let mut solid_registry = SolidRegistry::default();

    // Create domain objects for the cube
    let solid_id = create_rectangular_solid(
        2.0,
        2.5,
        3.5,
        &mut vertex_registry,
        &mut segment_registry,
        &mut polygon_registry,
        &mut solid_registry,
    )
    .expect("Failed to create cube for setup world");

    // Get a reference to the solid in the registry
    let solid = solid_registry
        .get(&solid_id)
        .expect("Failed to get solid from registry");

    // Generate mesh from domain objects
    let mesh = create_mesh_from_solid(
        &solid,
        &polygon_registry,
        &segment_registry,
        &vertex_registry,
    );
    let mesh_handle = meshes.add(mesh);

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
    println!("  Solid ID: {}", solid.id);
    println!("  Polygons: {}", solid.polygons.len());
    println!("  Segments: {}", segment_registry.segments.len());
    println!("  Vertices: {}", vertex_registry.vertices.len());
}
