use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::cameras::MainCamera;

// Min and Max radius for camera
const MIN_RADIUS: f32 = 5.0;
const MAX_RADIUS: f32 = 500.0;
const MIN_HEIGHT: f32 = 5.0;
const MAX_HEIGHT: f32 = 500.0;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pan_orbit_camera);
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCameraControls {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub target_radius: f32,
}

impl Default for PanOrbitCameraControls {
    fn default() -> Self {
        PanOrbitCameraControls {
            focus: Vec3::ZERO,
            radius: MIN_RADIUS,
            target_radius: MIN_RADIUS,
        }
    }
}

/// Pan the camera with left mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCameraControls, &mut Transform, &Projection), With<MainCamera>>,
    time: Res<Time>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut keyboard_move = Vec3::ZERO;
    let mut mouse_move = Vec3::ZERO;
    let velocity = 10.0;
    let cursor_velocity = 2.0;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        keyboard_move.z -= velocity * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        keyboard_move.z += velocity * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        keyboard_move.x -= velocity * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        keyboard_move.x += velocity * time.delta_seconds();
    }

    let window = windows.get_single().unwrap();
    let window_size = get_primary_window_size(&windows);
    let default_cursor_pos = Vec2::new(window_size.x * 0.5, window_size.y * 0.5);
    let cursor_pos = window.cursor_position().unwrap_or(default_cursor_pos);
    // print!("cursor_pos: {:?}, window: {:?}\n", cursor_pos, window);
    // if cursor_pos.x < window_size.x * 0.05 {
    //     mouse_move.x -= cursor_velocity;
    // }
    // if cursor_pos.x > window_size.x * 0.95 {
    //     mouse_move.x += cursor_velocity;
    // }
    // if cursor_pos.y < window_size.y * 0.05 {
    //     mouse_move.y += cursor_velocity;
    // }
    // if cursor_pos.y > window_size.y * 0.95 {
    //     mouse_move.y -= cursor_velocity;
    // }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        let mut any_movement = rotation_move.length_squared() > 0.0;
        any_movement |= pan.length_squared() > 0.0;
        any_movement |= scroll.abs() > 0.0;
        any_movement |= keyboard_move.length_squared() > 0.0;
        any_movement |= mouse_move.length_squared() > 0.0;

        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&windows);
            let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            pan_orbit.target_radius -= scroll * pan_orbit.target_radius * 0.2;
            // don't allow zoom to reach zero or you get stuck
            pan_orbit.target_radius = f32::max(pan_orbit.target_radius, MIN_RADIUS);
            pan_orbit.target_radius = f32::min(pan_orbit.target_radius, MAX_RADIUS);
        } else if keyboard_move.length_squared() > 0.0 {
            // translate by local axes
            let right = transform.rotation * Vec3::X * keyboard_move.x;
            let forward = transform.rotation * Vec3::Z * keyboard_move.z;
            // make panning proportional to distance away from focus point
            let translation = (right + forward) * pan_orbit.radius;
            pan_orbit.focus += translation * 0.1;
        } else if mouse_move.length_squared() > 0.0 {
            // translate by local axes
            let right = transform.rotation * Vec3::X * mouse_move.x;
            let forward = transform.rotation * Vec3::Z * mouse_move.y;
            // make panning proportional to distance away from focus point
            let translation = (right + forward) * pan_orbit.radius;
            pan_orbit.focus += translation * 0.01;
        }

        pan_orbit.focus.y = 0.;

        if any_movement {
            if pan_orbit.radius != pan_orbit.target_radius
                && pan.length_squared() == 0.0
                && rotation_move.length_squared() == 0.0
                && keyboard_move.length_squared() == 0.0
                && mouse_move.length_squared() == 0.0
            {
                pan_orbit.radius = lerp(pan_orbit.radius, pan_orbit.target_radius, 0.1);
            }
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            let damping_factor = 0.9;
            let target_pos =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
            transform.translation = transform.translation.lerp(target_pos, damping_factor);
            transform.translation.y = f32::max(transform.translation.y, MIN_HEIGHT);
            transform.translation.y = f32::min(transform.translation.y, MAX_HEIGHT);
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn get_primary_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let Ok(window) = windows.get_single() else { return Vec2::ZERO; };
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
