use bevy::{
    math::{
        bounding::{Aabb2d, BoundingVolume},
        vec2, vec3,
    },
    prelude::*,
};
use rand::Rng;

use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_LIMIT},
    prelude::{Collider, Health, YSpeed},
};

use super::{Enemy, EnemyKind};

const SPAWN_Y_OFFSET: f32 = 45.0;

#[derive(Resource)]
pub struct AsteroidSpawnTimer(pub Timer);

#[derive(Component)]
pub struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    sprite: SpriteBundle,
    speed: YSpeed,
    health: Health,
    asteroid: Asteroid,
    collider: Collider,
    enemy: Enemy,
}

pub const ASTEROID_SCORE: u32 = 1;
pub const ASTEROID_SPAWNER_TRIGGER_INTERVAL: f32 = 2.0;
pub const ASTEROID_ROTATION_SPEED: f32 = 1.25;
pub const ASTEROID_SPEED: f32 = 2.0;
pub const ASTEROID_HEALTH: f32 = 5.0;
pub const ASTEROID_COLLIDER_SIZE: (f32, f32) = (101.0, 84.0); // hardcoded size because we should have the boundaries of the collider and it should not be given by the sprite

impl AsteroidBundle {
    fn new(start_position: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_start_position, y_start_position) = start_position;

        Self {
            asteroid: Asteroid,
            enemy: Enemy(EnemyKind::Asteroid),
            health: Health(ASTEROID_HEALTH),
            speed: YSpeed(ASTEROID_SPEED),
            collider: Collider(Aabb2d::new(
                vec2(x_start_position, y_start_position),
                Vec2::from(ASTEROID_COLLIDER_SIZE) / 2.0,
            )),
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    translation: vec3(x_start_position, y_start_position, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn spawn_asteroids_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut asteroids_spawn_timer: ResMut<AsteroidSpawnTimer>,
) {
    if asteroids_spawn_timer.0.tick(time.delta()).just_finished() {
        let should_spawn = rand::random::<bool>();

        if should_spawn {
            let asteroid_texture = asset_server.load("big_meteor_gray.png");
            let start_position = (
                rand::thread_rng().gen_range(-WINDOW_X_LIMIT..WINDOW_X_LIMIT),
                rand::thread_rng().gen_range(WINDOW_Y_LIMIT..WINDOW_Y_LIMIT + SPAWN_Y_OFFSET),
            );

            commands.spawn(AsteroidBundle::new(start_position, asteroid_texture));
        }
    }
}

pub fn asteroids_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Collider, &YSpeed), With<Asteroid>>,
) {
    for (mut transform, mut collider, speed) in &mut query {
        // Move sprite
        transform.translation.y -= speed.0;
        transform.rotate_z(ASTEROID_ROTATION_SPEED * time.delta_seconds());

        // Move collider
        collider.0.translate_by(vec2(0.0, -speed.0))
    }
}
