use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct LastPos(pub Vec3);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Camera;
