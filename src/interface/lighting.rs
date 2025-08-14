use bevy::prelude::*;

pub fn spawn_lights(commands: &mut Commands) {
    // Directional light without shadows to eliminate artifacts
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 400.0,
            shadow_depth_bias: 10.0,
            shadow_normal_bias: 10.0,
            ..default()
        },
        Transform::from_xyz(5.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 4000.0,
            ..default()
        },
        Transform::from_xyz(-2.0, -1.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.35),
        brightness: 0.2,
        affects_lightmapped_meshes: false,
    });
}
