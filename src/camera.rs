use crate::Settings;
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera3d,
};

pub fn move_camera(
    settings: Res<Settings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let mut moved = false;
    let mut dir_vec = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::W) {
        dir_vec.z += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        dir_vec.z -= 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        dir_vec.x -= 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        dir_vec.x += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::E) {
        dir_vec.y += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        dir_vec.y -= 1.0;
        moved = true;
    }

    if moved {
        let mut camera = camera.single_mut();
        let dir_vec = dir_vec.normalize_or_zero();
        camera.translation += dir_vec * settings.camera_speed * settings.time_step;
    }
}

pub fn rotate_camera(
    settings: Res<Settings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let mut moved = false;
    let mut dir_vec = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::K) {
        dir_vec.y += -1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::I) {
        dir_vec.y += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::J) {
        dir_vec.x += -1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::L) {
        dir_vec.x += 1.0;
        moved = true;
    }

    if moved {
        let mut transform = camera.single_mut();
        let dir_vec = dir_vec.normalize_or_zero();
        let rotation_change = Quat::from_rotation_x(-dir_vec.y * 2.0 * settings.time_step);
        transform.rotate(rotation_change);
        let rotation_change = Quat::from_rotation_y(-dir_vec.x * 2.0 * settings.time_step);
        transform.rotate(rotation_change);
    }
}
