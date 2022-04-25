#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use bevy::{core::FixedTimestep, prelude::*};

mod camera;
mod components;
mod creation;
mod generation;
mod gravity;
pub mod utils;
pub use bevy::math::Vec3;
pub use generation::{Prefab, Prefab::*};
pub use utils::colors;

// Stages
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

// Resources
/// Settings resource for Bevy
pub struct Settings {
    time_step: f32,
    gravity_constant: f32,
    particle_radius: f32,
    camera_position: f32,
    camera_speed: f32,
}

struct Systems(Vec<Prefab>);

/// # A struct in charge of configuring and running the simulation.
/// The simulation struct is in charge of configuring the simulation details, like time step, gravity constant, etc.
/// This struct also runs Bevy's game engine in order to run the simulation.
pub struct Simulation {
    time_step: f32,
    gravity_constant: f32,
    particle_radius: f32,
    camera_speed: f32,
    camera_position: f32,
    background_color: Color,
    systems: Vec<Prefab>,
}

impl Simulation {
    /// Creates a new simulation with user defined configuration.
    pub fn new(
        time_step: f32,
        gravity_constant: f32,
        particle_radius: f32,
        camera_speed: f32,
        camera_position: f32,
        background_color: Color,
    ) -> Self {
        Self {
            time_step,
            gravity_constant,
            particle_radius,
            camera_speed,
            camera_position,
            background_color,
            systems: vec![],
        }
    }

    /// Configures the simulation systems to use. Systems referring to celestial bodies, galaxies, asteroid belts, etc.
    pub fn config(&mut self, systems: Vec<Prefab>) {
        self.systems = systems;
    }

    /// Runs the simulation, it opens a window and runs the configuration. See the repo's examples.
    pub fn run(&self) {
        App::new()
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(self.background_color))
            .insert_resource(Settings {
                time_step: self.time_step,
                gravity_constant: self.gravity_constant,
                particle_radius: self.particle_radius,
                camera_position: self.camera_position,
                camera_speed: self.camera_speed,
            })
            .insert_resource(Systems(self.systems.clone()))
            .add_plugins(DefaultPlugins)
            .add_startup_system(Self::setup)
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(self.time_step as f64))
                    .with_system(gravity::interact_bodies)
                    .with_system(gravity::interact_particles_with_bodies)
                    .with_system(gravity::integrate),
            )
            .add_system(camera::move_camera)
            .add_system(camera::rotate_camera)
            .run();
    }

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        settings: Res<Settings>,
        systems: Res<Systems>,
    ) {
        for syst in systems.0.iter() {
            match syst {
                Particle {
                    initial_position,
                    initial_velocity,
                    color,
                } => {
                    creation::create_particle(
                        *initial_position,
                        *initial_velocity,
                        *color,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &settings,
                    );
                }
                Body {
                    mass,
                    radius,
                    initial_position,
                    initial_velocity,
                    color,
                } => {
                    creation::create_body(
                        *mass,
                        *radius,
                        *color,
                        *initial_position,
                        *initial_velocity,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &settings,
                    );
                }
                Galaxy {
                    amount,
                    arms,
                    center_mass,
                    center_pos,
                    center_vel,
                    normal,
                    particle_color,
                    center_color,
                } => {
                    creation::create_galaxy(
                        *amount,
                        *arms,
                        *center_mass,
                        *center_pos,
                        *center_vel,
                        *normal,
                        *particle_color,
                        *center_color,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &settings,
                    );
                }
                AsteroidBelt {
                    amount,
                    radius,
                    center_mass,
                    center_pos,
                    center_vel,
                    normal,
                    particle_color,
                } => {
                    creation::create_asteroid_belt(
                        *amount,
                        *radius,
                        *center_mass,
                        *center_pos,
                        *center_vel,
                        *normal,
                        *particle_color,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &settings,
                    );
                }
            }
        }

        // light
        commands.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        });

        // camera
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, settings.camera_position)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }
}
