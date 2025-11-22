use bevy::prelude::*;

pub fn spawn_lights(commands: &mut Commands) {
    // Main directional light - shadows disabled to avoid artifacts
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 400.0,
            ..default()
        },
        Transform::from_xyz(5.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Fill light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 2000.0,
            ..default()
        },
        Transform::from_xyz(-2.0, -1.0, -4.0),
        GlobalTransform::default(),
    ));

    // Ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.35),
        brightness: 0.4,
        affects_lightmapped_meshes: false,
    });
}
