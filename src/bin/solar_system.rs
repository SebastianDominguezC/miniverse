extern crate miniverse;
use miniverse::{colors, AsteroidBelt, Body, Simulation, Vec3};

const TIME_STEP: f32 = 1.0 / 60.0;
const G: f32 = 10.0;
const PARTICLE_RADIUS: f32 = 0.07;

fn main() {
    let camera_pos: Vec3 = Vec3::new(0.0, 0.0, -250.0);

    let mut sim = Simulation::new(TIME_STEP, G, PARTICLE_RADIUS, camera_pos, colors::gray_dark);

    let systems = vec![
        Body {
            mass: 10.0,
            radius: 5.0,
            color: colors::yellow,
            initial_position: Vec3::new(0.0, 0.0, 0.0),
            initial_velocity: Vec3::new(0.0, 0.0, 0.0),
        },
        Body {
            mass: 0.00001,
            radius: 0.7,
            color: colors::blue,
            initial_position: Vec3::new(-30.0, 0.0, 0.0),
            initial_velocity: Vec3::new(0.0, 8.5, 0.0),
        },
        Body {
            mass: 0.00001,
            radius: 0.9,
            color: colors::red,
            initial_position: Vec3::new(-50.0, 0.0, 0.0),
            initial_velocity: Vec3::new(0.0, 10.0, 0.0),
        },
        Body {
            mass: 0.00001,
            radius: 1.2,
            color: colors::green_lime,
            initial_position: Vec3::new(-80.0, 0.0, 0.0),
            initial_velocity: Vec3::new(0.0, 11.5, 0.0),
        },
        AsteroidBelt {
            amount: 700,
            radius: 85.0,
            center_mass: 10.0,
            center_pos: Vec3::new(0.0, 0.0, 0.0),
            center_vel: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            particle_color: colors::gray_light,
        },
        Body {
            mass: 0.001,
            radius: 1.5,
            color: colors::gray_light,
            initial_position: Vec3::new(-120.0, 0.0, 0.0),
            initial_velocity: Vec3::new(0.0, 11.0, 0.0),
        },
    ];
    sim.config(systems);
    sim.run();
}
