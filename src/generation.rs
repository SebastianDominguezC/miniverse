use bevy::prelude::*;

#[derive(Copy, Clone)]
pub enum Prefab {
    Particle {
        initial_position: Vec3,
        initial_velocity: Vec3,
        color: Color,
    },
    Body {
        mass: f32,
        radius: f32,
        color: Color,
        initial_position: Vec3,
        initial_velocity: Vec3,
    },
    Galaxy {
        amount: u32,
        arms: u32,
        center_mass: f32,
        center_pos: Vec3,
        center_vel: Vec3,
        normal: Vec3,
        particle_color: Color,
        center_color: Color,
    },
    AsteroidBelt {
        amount: u32,
        radius: f32,
        center_mass: f32,
        center_pos: Vec3,
        center_vel: Vec3,
        normal: Vec3,
        particle_color: Color,
    },
}
