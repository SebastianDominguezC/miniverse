use crate::components::{Acceleration, LastPos, Mass, Particle};
use crate::Settings;

use bevy::prelude::*;

// Algorithm based on Bevy's examples of iterable combinations
pub fn interact_bodies(
    settings: Res<Settings>,
    mut query: Query<(&Mass, &Transform, &mut Acceleration)>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
        iter.fetch_next()
    {
        let delta = transform2.translation - transform1.translation;
        let distance_sq: f32 = delta.length_squared();

        let f = settings.gravity_constant / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

// Algorithm based on Bevy's examples of iterable combinations
pub fn interact_particles_with_bodies(
    settings: Res<Settings>,
    mut particles: Query<(&Particle, &Transform, &mut Acceleration)>,
    bodies: Query<(&Transform, &Mass)>,
) {
    for (_, transform, mut acc) in particles.iter_mut() {
        for (trans, m) in bodies.iter() {
            let delta = trans.translation - transform.translation;
            let distance_sq: f32 = delta.length_squared();

            let f = settings.gravity_constant / distance_sq;
            let force_unit_mass = delta * f;
            acc.0 += force_unit_mass * m.0;
        }
    }
}

// Algorithm based on Bevy's examples of iterable combinations
pub fn integrate(
    settings: Res<Settings>,
    mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>,
) {
    let dt_sq = (settings.time_step * settings.time_step) as f32;
    for (mut acceleration, mut transform, mut last_pos) in query.iter_mut() {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)

        let new_pos =
            transform.translation + transform.translation - last_pos.0 + acceleration.0 * dt_sq;
        acceleration.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}
