/// Interface layer for the application
/// This module sets up the world and the camera
use bevy::pbr::*;
use bevy::prelude::*;

use crate::application::create_cube;
use crate::domain::{Polygon, Segment, Vertex};

/// A plugin for the interface
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world)
            .add_systems(Update, camera_controls);
    }
}

/// Bevy system to setup the world with our cube
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create our semantic cube
    let (vertices, segments, polygons, solid) = create_cube();

    // Log the creation
    println!("Created cube with:");
    println!("  {} segments", segments.len());
    println!("  {} polygons", polygons.len());
    println!("  1 solid with {} polygon references", solid.polygons.len());

    // Create a 3D camera
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Create lighting
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Create the cube mesh from our semantic data
    let cube_mesh = create_cube_mesh(&vertices, &segments, &polygons);

    // Spawn the cube entity using the correct component types for Bevy 0.16
    let mesh_handle = meshes.add(cube_mesh);
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
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
}

/// Create a mesh from our semantic cube data
fn create_cube_mesh(
    vertices: &[Vertex; 8],
    _segments: &[Segment; 12],
    _polygons: &[Polygon; 6],
) -> Mesh {
    // Convert our semantic vertices to Bevy mesh format
    let positions: Vec<[f32; 3]> = vertices
        .iter()
        .map(|v| [v.position.x, v.position.y, v.position.z])
        .collect();

    // Define the cube faces (6 faces, 4 vertices each, 2 triangles per face)
    let indices = vec![
        // Bottom face (z = -0.5)
        0, 1, 2, 0, 2, 3, // Top face (z = 0.5)
        4, 7, 6, 4, 6, 5, // Back face (y = -0.5)
        0, 4, 5, 0, 5, 1, // Front face (y = 0.5)
        2, 6, 7, 2, 7, 3, // Left face (x = -0.5)
        3, 7, 4, 3, 4, 0, // Right face (x = 0.5)
        1, 5, 6, 1, 6, 2,
    ];

    // Calculate normals for each face
    let mut normals = Vec::new();
    for face in 0..6 {
        let base = face * 6;
        let v0 = positions[indices[base] as usize];
        let v1 = positions[indices[base + 1] as usize];
        let v2 = positions[indices[base + 2] as usize];

        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        let normal = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];

        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        let normalized = [normal[0] / length, normal[1] / length, normal[2] / length];

        // Add the same normal for all vertices in this face
        for _ in 0..6 {
            normals.push(normalized);
        }
    }

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(
        indices.into_iter().map(|i| i as u32).collect(),
    ));

    mesh
}

/// Camera controls system
fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = query.single_mut() {
        let speed = 2.0;
        let rotation_speed = 1.0;

        // Camera movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = camera_transform.forward();
            camera_transform.translation += forward * speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back = camera_transform.back();
            camera_transform.translation += back * speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            let left = camera_transform.left();
            camera_transform.translation += left * speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            let right = camera_transform.right();
            camera_transform.translation += right * speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            let up = camera_transform.up();
            camera_transform.translation += up * speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            let down = camera_transform.down();
            camera_transform.translation += down * speed * time.delta_secs();
        }

        // Camera rotation
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(Vec3::Y, rotation_speed * time.delta_secs()),
            );
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(Vec3::Y, -rotation_speed * time.delta_secs()),
            );
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let right = camera_transform.right();
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(*right, rotation_speed * time.delta_secs()),
            );
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let right = camera_transform.right();
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(*right, -rotation_speed * time.delta_secs()),
            );
        }
    }
}
