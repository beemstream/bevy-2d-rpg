use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Damage(pub f32);

#[derive(Debug, Component)]
pub struct Health(pub f32);

#[derive(Debug, Component)]
pub struct MaxHealth(pub f32);
