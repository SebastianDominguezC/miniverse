use bevy::prelude::*;
use {rand::prelude::*, std::f32::consts::PI};

use crate::components::{Acceleration, Body, LastPos, Mass, Particle};
use crate::Settings;

pub fn create_particle(
    initial_position: Vec3,
    initial_velocity: Vec3,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    settings: &Res<Settings>,
) {
    commands
        .spawn()
        .insert(Particle)
        .insert(Acceleration(initial_velocity / settings.time_step))
        .insert(LastPos(initial_position))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: settings.particle_radius,
                subdivisions: 3,
            })),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(
                initial_position.x,
                initial_position.y,
                initial_position.z,
            ),
            ..default()
        });
}

pub fn create_body(
    mass: f32,
    radius: f32,
    color: Color,
    initial_position: Vec3,
    initial_velocity: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    settings: &Res<Settings>,
) {
    commands
        .spawn()
        .insert(Body)
        .insert(Mass(mass))
        .insert(Acceleration(initial_velocity / settings.time_step))
        .insert(LastPos(initial_position))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 3,
            })),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(
                initial_position.x,
                initial_position.y,
                initial_position.z,
            ),
            ..default()
        });
}

// Galaxy generation algorithm used from nbody simulation: https://github.com/timokoesters/nbodysim
pub fn create_galaxy(
    amount: u32,
    arms: u32,
    center_mass: f32,
    center_pos: Vec3,
    center_vel: Vec3,
    mut normal: Vec3,
    particle_color: Color,
    center_color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    settings: &Res<Settings>,
) {
    // Helpers
    normal = normal.normalize();
    let tangent = normal.cross(Vec3::new(-normal.z, normal.x, normal.y));
    let bitangent = normal.cross(tangent);

    // Center of galaxy
    create_body(
        1.0,
        1.0,
        center_color,
        center_pos,
        center_vel,
        commands,
        meshes,
        materials,
        settings,
    );

    // Generate spiral arms of the galaxy
    for _ in 0..amount {
        // Choose arm
        let arm = rand_distr::Uniform::from(0..arms).sample(&mut thread_rng());

        let radius = 5.0
            + (rand_distr::Normal::<f32>::new(0.0, 50.0)
                .unwrap()
                .sample(&mut thread_rng()))
            .abs();

        let angle = arm as f32 / arms as f32 * 2.0 * PI - radius * 1E-11
            + rand_distr::Normal::new(0.0, PI / 16.0)
                .unwrap()
                .sample(&mut thread_rng());

        let diff = tangent * angle.sin() + bitangent * angle.cos();

        let fly_direction = diff.cross(normal).normalize();

        let pos = center_pos + diff * radius;

        // Fg = Fg
        // G * m1 * m2 / (r^2 + C) = m1 * v^2 / r
        // sqrt(G * m2 * r / (r^2 + C)) = v
        let speed = (settings.gravity_constant as f64 * center_mass as f64 * radius as f64
            / (radius as f64 * radius as f64))
            .sqrt() as f32;
        let vel = center_vel + fly_direction * speed;

        create_particle(
            pos,
            vel,
            particle_color,
            commands,
            meshes,
            materials,
            settings,
        );
    }
}

pub fn create_asteroid_belt(
    amount: u32,
    radius: f32,
    center_mass: f32,
    center_pos: Vec3,
    center_vel: Vec3,
    mut normal: Vec3,
    particle_color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    settings: &Res<Settings>,
) {
    // Helpers
    normal = normal.normalize();
    let tangent = normal.cross(Vec3::new(-normal.z, normal.x, normal.y));
    let bitangent = normal.cross(tangent);

    // Generate a particle asteroid
    for _ in 0..amount {
        let angle = rand_distr::Normal::new(0.0, 2.0 * PI)
            .unwrap()
            .sample(&mut thread_rng());

        let diff = tangent * angle.sin() + bitangent * angle.cos();

        let fly_direction = diff.cross(normal).normalize();

        let pos = center_pos + diff * radius;

        // Fg = Fg
        // G * m1 * m2 / (r^2 + C) = m1 * v^2 / r
        // sqrt(G * m2 * r / (r^2 + C)) = v
        let speed = (settings.gravity_constant as f64 * center_mass as f64 * radius as f64
            / (radius as f64 * radius as f64)) as f32
            * settings.gravity_constant;
        let vel = center_vel + fly_direction * speed;

        create_particle(
            pos,
            vel,
            particle_color,
            commands,
            meshes,
            materials,
            settings,
        );
    }
}
