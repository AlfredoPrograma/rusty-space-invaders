use bevy::{math::vec3, prelude::*};
use rand::Rng;

use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_LIMIT},
    prelude::YSpeed,
};

const SPAWN_Y_OFFSET: f32 = 45.0;
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            ASTEROID_SPAWNER_TRIGGER_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_asteroids_system, asteroids_movement_system));
    }
}

#[derive(Resource)]
struct AsteroidSpawnTimer(Timer);

#[derive(Component)]
struct Health(pub i32);

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    sprite: SpriteBundle,
    speed: YSpeed,
    health: Health,
    asteroid: Asteroid,
}

const ASTEROID_SPAWNER_TRIGGER_INTERVAL: f32 = 2.0;
const ASTEROID_ROTATION_SPEED: f32 = 1.25;
const ASTEROID_SPEED: f32 = 2.0;
const ASTEROID_HEALTH: i32 = 30;

impl AsteroidBundle {
    fn new(start_position: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_start_position, y_start_position) = start_position;

        Self {
            asteroid: Asteroid,
            health: Health(ASTEROID_HEALTH),
            speed: YSpeed(ASTEROID_SPEED),
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
    mut query: Query<(&mut Transform, &YSpeed), With<Asteroid>>,
) {
    for (mut transform, speed) in &mut query {
        transform.translation.y -= speed.0;
        transform.rotate_z(ASTEROID_ROTATION_SPEED * time.delta_seconds());
    }
}
