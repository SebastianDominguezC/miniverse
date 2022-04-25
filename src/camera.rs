use crate::Settings;
use bevy::{prelude::*, render::camera::Camera3d};

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
        let dir_vec = dir_vec.normalize();
        camera.translation += dir_vec * settings.camera_speed * settings.time_step;
    }
}
