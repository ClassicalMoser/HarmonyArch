use crate::interface::ui::UiState;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

/// Zoom speed for orthographic view (viewport height units per second)
const ORTHO_ZOOM_SPEED: f32 = 5.0;

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
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_translation(config.initial_position)
            .looking_at(config.look_at_target, config.up_direction),
        GlobalTransform::default(),
    ));
}

/// Camera controls system for movement, rotation, and orthographic zoom
pub fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    config: Res<CameraConfig>,
    mut ui_state: ResMut<UiState>,
) {
    if let Ok(mut camera_transform) = query.single_mut() {
        if ui_state.isometric_view {
            // In orthographic view, W/S control zoom instead of forward/back movement
            let zoom_delta = ORTHO_ZOOM_SPEED * time.delta_secs();
            if keyboard_input.pressed(KeyCode::KeyW) {
                ui_state.ortho_zoom -= zoom_delta;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                ui_state.ortho_zoom += zoom_delta;
            }
            // Still allow lateral movement (A/D) and vertical (Q/E)
            handle_lateral_movement(&mut camera_transform, &keyboard_input, &time, &config);
        } else {
            // In perspective view, normal movement controls
            handle_movement(&mut camera_transform, &keyboard_input, &time, &config);
        }
        handle_rotation(&mut camera_transform, &keyboard_input, &time, &config);
    }
}

/// Handle camera movement controls (perspective view)
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

/// Handle lateral and vertical movement (for orthographic view)
fn handle_lateral_movement(
    camera_transform: &mut Transform,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    config: &CameraConfig,
) {
    let delta_time = time.delta_secs();
    let speed = config.movement_speed * delta_time;

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

/// Update camera projection based on UI state
pub fn update_camera_projection(
    mut camera_query: Query<(&mut Projection, &GlobalTransform), With<Camera>>,
    mut ui_state: ResMut<UiState>,
) {
    let Ok((mut projection, global_transform)) = camera_query.single_mut() else {
        return;
    };

    if ui_state.isometric_view {
        match projection.as_mut() {
            Projection::Perspective(persp) => {
                // Calculate viewport height based on camera distance and FOV
                let camera_distance = global_transform.translation().length();
                let fov_rad = persp.fov;
                // Visible height at distance d: height = 2 * d * tan(fov/2)
                let viewport_height = 2.0 * camera_distance * (fov_rad / 2.0).tan();
                ui_state.ortho_zoom = viewport_height;

                *projection = Projection::Orthographic(OrthographicProjection {
                    scale: 1.0,
                    near: 0.1,
                    far: 100.0,
                    viewport_origin: Vec2::new(0.5, 0.5),
                    area: Rect::default(),
                    scaling_mode: ScalingMode::FixedVertical {
                        viewport_height: ui_state.ortho_zoom,
                    },
                });
            }
            Projection::Orthographic(ortho) => {
                ortho.viewport_origin = Vec2::new(0.5, 0.5);
                ortho.scaling_mode = ScalingMode::FixedVertical {
                    viewport_height: ui_state.ortho_zoom,
                };
            }
            Projection::Custom(_) => {}
        }
    } else if !matches!(projection.as_ref(), Projection::Perspective(_)) {
        *projection = Projection::Perspective(PerspectiveProjection::default());
    }
}
