use bevy::prelude::*;

pub fn spawn_lights(commands: &mut Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1.0,
            ..default()
        },
        Transform::from_xyz(-2.0, 2.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.35),
        brightness: 0.2,
        affects_lightmapped_meshes: false,
    });
}
