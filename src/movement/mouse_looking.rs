use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::movement::resources::View;

const MOUSE_SENSITIVITY: f32 = 0.5;

pub fn mouse_looking(
    mut camera: Query<(&mut Camera3d, &mut Transform)>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut view: ResMut<View>,
    time: Res<Time>,
) {
    if view.mouse_locked {
        let (_, mut transform) = camera.single_mut().unwrap();
        for ev in mouse_motion.read() {
            view.yaw -= ev.delta.x * MOUSE_SENSITIVITY * time.delta_secs();
            view.pitch -= ev.delta.y * MOUSE_SENSITIVITY * time.delta_secs();
            view.pitch = clamp(view.pitch, -90., 90.);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, view.pitch, view.yaw, 0.);
        }
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}

pub fn cursor_setup(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    primary_window.cursor_options.visible = false;
}

pub fn cursor_checks(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut view: ResMut<View>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if view.mouse_locked {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            let mut primary_window = q_windows.single_mut().unwrap();
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;

            primary_window.cursor_options.visible = true;
            view.mouse_locked = false;
        }
    } else if !view.mouse_locked && mouse_input.just_pressed(MouseButton::Left) {
        cursor_setup(q_windows);
        view.mouse_locked = true;
    }
}
