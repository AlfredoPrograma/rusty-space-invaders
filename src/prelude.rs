use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct XSpeed(pub f32);

#[derive(Component)]
pub struct YSpeed(pub f32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Collider(pub Aabb2d);

#[derive(Component)]
pub struct Damage(pub f32);
