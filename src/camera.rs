use crate::InputCodes;
use bevy::input::mouse::MouseMotion;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::Inspectable;

// These values are all in degrees, for ease of use
#[derive(Component, Inspectable)]
pub struct CameraData {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub enabled: bool,
    pub velocity: Vec3,
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32,
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 12.0,
            enabled: true,
            velocity: Vec3::ZERO,
            acceleration: 1.5,
            max_speed: 3.0,
            deceleration: 1.25,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(mouse_handling_system)
            .add_system(grab_mouse)
            .add_system(keyboard_movement_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraData::default());
}

fn mouse_handling_system(
    time: Res<Time>,
    mut mouse_event: EventReader<MouseMotion>,
    mut cam_query: Query<(&mut Transform, &mut CameraData)>,
) {
    let mut delta = Vec2::ZERO;

    for event in mouse_event.iter() {
        delta += event.delta;
    }
    if delta.is_nan() || !delta.is_finite() || delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut data) in cam_query.iter_mut() {
        if !data.enabled {
            return;
        }
        data.yaw -= delta.x * time.delta_seconds() * data.sensitivity;
        data.pitch += delta.y * time.delta_seconds() * data.sensitivity;

        data.pitch = data.pitch.clamp(-90.0, 90.0);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, data.yaw.to_radians())
            * Quat::from_axis_angle(-Vec3::X, data.pitch.to_radians());
    }
}

fn keyboard_movement_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut cam_query: Query<(&mut Transform, &mut CameraData)>,
    codes: Res<InputCodes>,
) {
    for (mut transform, mut data) in cam_query.iter_mut() {
        if !data.enabled {
            return;
        }

        let mut horizontal_axis = 0.0;
        if keys.pressed(codes.forward) {
            horizontal_axis += 1.0;
        }
        if keys.pressed(codes.backward) {
            horizontal_axis -= 1.0;
        }

        let mut side_axis = 0.0;
        if keys.pressed(codes.right) {
            side_axis += 1.0;
        }
        if keys.pressed(codes.left) {
            side_axis -= 1.0;
        }

        let mut vertical_axis = 0.0;
        if keys.pressed(codes.up) {
            vertical_axis += 1.0;
        }
        if keys.pressed(codes.down) {
            vertical_axis -= 1.0;
        }

        let velocity: Vec3 = vec3(side_axis, vertical_axis, -horizontal_axis)
            * time.delta_seconds()
            * data.acceleration;

        data.velocity += velocity;

        data.velocity = data
            .velocity
            .clamp(Vec3::splat(-data.max_speed), Vec3::splat(data.max_speed));

        let deceleration = if data.velocity.length() != 0.0 {
            data.velocity.normalize() * -1.0 * data.deceleration
        } else {
            Vec3::ZERO
        };

        let delta_deccel = deceleration * time.delta_seconds();
        data.velocity = if (data.velocity + delta_deccel).signum() != data.velocity.signum() {
            Vec3::ZERO
        } else {
            data.velocity + delta_deccel
        };

        let vel_with_rotation = transform.rotation.mul_vec3(data.velocity);

        transform.translation += vel_with_rotation;
    }
}

fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut cam_query: Query<&mut CameraData>,
    mut gui_context: ResMut<EguiContext>,
    input_codes: Res<InputCodes>,
) {
    let window = windows.get_primary_mut().unwrap();
    let egui = gui_context.ctx_for_window_mut(window.id());
    if mouse.just_pressed(input_codes.focus) && !egui.is_pointer_over_area() {
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
        window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));

        for mut data in cam_query.iter_mut() {
            data.enabled = true;
        }
    }
    if key.just_pressed(input_codes.exit) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
        window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));
        for mut data in cam_query.iter_mut() {
            data.enabled = false;
        }
    }
}
