use bevy::prelude::*;

/// # Miniverse building blocks
/// Prefabs are systems, which the simulation will render and run. They are the building blocks of creating your own miniverse.
#[derive(Copy, Clone)]
pub enum Prefab {
    /// A massless particle. Hence they don't exert forces on other bodies (but they do feel forces from bodies).
    Particle {
        initial_position: Vec3,
        initial_velocity: Vec3,
        color: Color,
    },
    /// A celestial body. Like a particle but with mass.
    Body {
        mass: f32,
        radius: f32,
        color: Color,
        initial_position: Vec3,
        initial_velocity: Vec3,
    },
    /// A galaxy, with a center body and spiraling particles. Normal vector is the direction that the galaxy faces.
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
    /// A circularily generated particle asteroid belt. Normal vector is the direction that the belt faces.
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
