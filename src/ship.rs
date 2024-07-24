use bevy::app::Plugin;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::prelude::Collider;
use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_SIZE},
    prelude::{XSpeed, YSpeed},
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ShootingTimer(Timer::from_seconds(
            SHOOTING_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, spawn_ship_system)
        .add_systems(
            Update,
            (ship_movement_system, spawn_shot_system, shot_moving_system),
        );
    }
}

#[derive(Component)]
struct Ship;

#[derive(Bundle)]
struct ShipBundle {
    speed: XSpeed,
    sprite: SpriteBundle,
    ship: Ship,
}

const SHIP_SPEED: f32 = 5.0;
const SHIP_SCALE: f32 = 0.75;
const SHIP_POSITION: f32 = -(WINDOW_Y_SIZE / 2.0) + 75.0;

impl ShipBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            ship: Ship,
            speed: XSpeed(SHIP_SPEED),
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    scale: vec3(SHIP_SCALE, SHIP_SCALE, 0.0),
                    translation: vec3(0.0, SHIP_POSITION, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

fn spawn_ship_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle: Handle<Image> = asset_server.load("player_ship.png");

    commands.spawn(ShipBundle::new(ship_handle));
}

fn ship_movement_system(
    mut query: Query<(&mut Transform, &XSpeed), With<Ship>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, speed) = query.get_single_mut().unwrap();

    for key_code in key.get_pressed() {
        match key_code {
            KeyCode::ArrowLeft => {
                let new_position = transform.translation.x - speed.0;

                if !(new_position < -WINDOW_X_LIMIT) {
                    transform.translation.x = new_position
                }
            }
            KeyCode::ArrowRight => {
                let new_position = transform.translation.x + speed.0;

                if !(new_position > WINDOW_X_LIMIT) {
                    transform.translation.x = new_position
                }
            }

            // Do nothing xd
            _ => {}
        }
    }
}

const SHOT_SPEED: f32 = 10.0;
const SHOOTING_INTERVAL: f32 = 0.5;
const SHOT_SPAWN_OFFSET: f32 = 35.0;
const SHOT_COLLIDER_SIZE: (f32, f32) = (9.0, 54.0);

#[derive(Resource)]
struct ShootingTimer(Timer);

#[derive(Component)]
pub struct Shot;

#[derive(Bundle)]
struct ShotBundle {
    speed: YSpeed,
    sprite: SpriteBundle,
    shot: Shot,
    collider: Collider,
}

impl ShotBundle {
    pub fn new(shot_offset: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_offset, y_offset) = shot_offset;

        Self {
            shot: Shot,
            speed: YSpeed(SHOT_SPEED),
            collider: Collider(Aabb2d::new(
                vec2(x_offset, y_offset),
                Vec2::from(SHOT_COLLIDER_SIZE) / 2.0,
            )),
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    translation: vec3(x_offset, y_offset, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

fn spawn_shot_system(
    mut shooting_timer: ResMut<ShootingTimer>,
    mut commands: Commands,
    query: Query<&Transform, With<Ship>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let shot_texture: Handle<Image> = asset_server.load("laser_base.png");
    let ship_position = query.get_single().unwrap().translation;

    if shooting_timer.0.tick(time.delta()).just_finished() {
        commands.spawn(ShotBundle::new(
            (
                ship_position.x + SHOT_SPAWN_OFFSET,
                ship_position.y + SHOT_SPAWN_OFFSET,
            ),
            shot_texture.clone(),
        ));
        commands.spawn(ShotBundle::new(
            (
                ship_position.x - SHOT_SPAWN_OFFSET,
                ship_position.y + SHOT_SPAWN_OFFSET,
            ),
            shot_texture,
        ));
    }
}

fn shot_moving_system(mut query: Query<(&mut Transform, &mut Collider, &YSpeed), With<Shot>>) {
    for (mut transform, mut collider, speed) in &mut query {
        // Move sprite
        transform.translation.y += speed.0;
        // Move collider
        collider.0.translate_by(vec2(0.0, speed.0));
    }
}

pub fn shot_collision(shot: Aabb2d, bounding_box: Aabb2d) -> bool {
    if !shot.intersects(&bounding_box) {
        return false;
    }

    return true;
}
