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
    prelude::{Collider, Damage, Health, Score, YSpeed},
    ship::{shot_collision, Shot},
};

const SPAWN_Y_OFFSET: f32 = 45.0;
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            ASTEROID_SPAWNER_TRIGGER_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                spawn_asteroids_system,
                asteroids_movement_system,
                take_damage,
            )
                .chain(),
        );
    }
}

#[derive(Resource)]
struct AsteroidSpawnTimer(Timer);

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    sprite: SpriteBundle,
    speed: YSpeed,
    health: Health,
    asteroid: Asteroid,
    collider: Collider,
}

const ASTEROID_SPAWNER_TRIGGER_INTERVAL: f32 = 2.0;
const ASTEROID_ROTATION_SPEED: f32 = 1.25;
const ASTEROID_SPEED: f32 = 2.0;
const ASTEROID_HEALTH: f32 = 7.0;
const ASTEROID_COLLIDER_SIZE: (f32, f32) = (101.0, 84.0); // hardcoded size because we should have the boundaries of the collider and it should not be given by the sprite

impl AsteroidBundle {
    fn new(start_position: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_start_position, y_start_position) = start_position;

        Self {
            asteroid: Asteroid,
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

fn spawn_asteroids_system(
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

fn asteroids_movement_system(
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

fn take_damage(
    shot_query: Query<(&Damage, &Collider, Entity), With<Shot>>,
    mut score_query: Query<(&mut Score, &mut Text), With<Score>>,
    mut enemy_query: Query<(&mut Health, &Collider, Entity), With<Asteroid>>,
    mut commands: Commands,
) {
    for (mut enemy_health, enemy_collider, enemy_entity) in &mut enemy_query {
        for (shot_damage, shot_collider, shot_entity) in &shot_query {
            if shot_collision(shot_collider.0, enemy_collider.0) {
                let updated_enemy_health = enemy_health.0 - shot_damage.0;

                if updated_enemy_health == 0.0 {
                    let (mut score_counter, mut score_text) = score_query
                        .get_single_mut()
                        .expect("should exist only one score");

                    // TODO:
                    // Fix bug where two shots collides at same time with
                    // asteroid and throw warn about invalid despawning and
                    // duplicated counter update
                    commands.entity(enemy_entity).despawn();

                    // TODO:
                    // Extract to `update_counter` function
                    // Check if text could be changed without create new section
                    let updated_counter = score_counter.0 + 1;
                    score_counter.0 = updated_counter;
                    score_text.sections = vec![TextSection {
                        value: format!("SCORE: {}", updated_counter),
                        ..Default::default()
                    }];
                } else {
                    enemy_health.0 = updated_enemy_health;
                }

                commands.entity(shot_entity).despawn();
            }
        }
    }
}
