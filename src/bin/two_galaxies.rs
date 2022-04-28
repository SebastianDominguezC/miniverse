extern crate miniverse;
use miniverse::{colors, Galaxy, Simulation, Vec3};

const TIME_STEP: f32 = 1.0 / 60.0;
const G: f32 = 10.0;
const PARTICLE_RADIUS: f32 = 0.05;

fn main() {
    let camera_pos: Vec3 = Vec3::new(0.0, 0.0, -75.0);

    let mut sim = Simulation::new(TIME_STEP, G, PARTICLE_RADIUS, camera_pos, colors::gray_dark);

    let systems = vec![
        Galaxy {
            amount: 2000,
            arms: 2,
            center_mass: 10.0,
            center_pos: Vec3::new(0.0, 0.0, 0.0),
            center_vel: Vec3::new(-0.3, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 1.0),
            particle_color: colors::blue,
            center_color: colors::red_light,
        },
        Galaxy {
            amount: 2000,
            arms: 2,
            center_mass: 10.0,
            center_pos: Vec3::new(20.0, 20.0, 20.0),
            center_vel: Vec3::new(0.0, -1.0, 0.0),
            normal: Vec3::new(1.0, 1.0, 1.0),
            particle_color: colors::red,
            center_color: colors::gray_light,
        },
    ];
    sim.config(systems);
    sim.run();
}
