use bevy::{math::bounding::Aabb2d, prelude::*};

/// Provides `XSpeed` to an entity.
///
/// It basically represents the movement speed of the entity in x-axis.
#[derive(Component)]
pub struct XSpeed(pub f32);

/// Provides `YSpeed` to an entity.
///
///  It basically represents the movement speedo of the entity in y-axis.
#[derive(Component)]
pub struct YSpeed(pub f32);

/// Provides `Health` attribute to an entity.
#[derive(Component)]
pub struct Health(pub f32);

/// Provides `Collider` attribute to an entity.
///
/// It will be used to define the bounding box to the entity to allow collision management.
#[derive(Component)]
pub struct Collider(pub Aabb2d);

/// Provides `Damage` attribute to an entity.
#[derive(Component)]
pub struct Damage(pub f32);
