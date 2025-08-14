use bevy::prelude::*;

/// Camera configuration and setup
#[derive(Resource, Clone)]
pub struct CameraConfig {
    pub initial_position: Vec3,
    pub look_at_target: Vec3,
    pub up_direction: Vec3,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            initial_position: Vec3::new(3.0, 3.0, 3.0),
            look_at_target: Vec3::ZERO,
            up_direction: Vec3::Y,
            movement_speed: 2.0,
            rotation_speed: 3.0,
        }
    }
}

/// Spawns the main camera with the given configuration
pub fn spawn_camera(commands: &mut Commands, config: &CameraConfig) {
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(
            config.initial_position.x,
            config.initial_position.y,
            config.initial_position.z,
        )
        .looking_at(config.look_at_target, config.up_direction),
        GlobalTransform::default(),
    ));
}

/// Camera controls system for movement and rotation
pub fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    config: Res<CameraConfig>,
) {
    if let Ok(mut camera_transform) = query.single_mut() {
        handle_movement(&mut camera_transform, &keyboard_input, &time, &config);
        handle_rotation(&mut camera_transform, &keyboard_input, &time, &config);
    }
}

/// Handle camera movement controls
fn handle_movement(
    camera_transform: &mut Transform,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    config: &CameraConfig,
) {
    let delta_time = time.delta_secs();
    let speed = config.movement_speed * delta_time;

    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = camera_transform.forward();
        camera_transform.translation += forward * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        let back = camera_transform.back();
        camera_transform.translation += back * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        let left = camera_transform.left();
        camera_transform.translation += left * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        let right = camera_transform.right();
        camera_transform.translation += right * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        let up = camera_transform.up();
        camera_transform.translation += up * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        let down = camera_transform.down();
        camera_transform.translation += down * speed;
    }
}

/// Handle camera rotation controls
fn handle_rotation(
    camera_transform: &mut Transform,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    config: &CameraConfig,
) {
    let delta_time = time.delta_secs();
    let speed = config.rotation_speed * delta_time;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, speed));
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, -speed));
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        let right = camera_transform.right();
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(*right, speed));
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        let right = camera_transform.right();
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(*right, -speed));
    }
}
