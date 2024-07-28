use bevy::{
    app::Update,
    asset::{AssetServer, Handle},
    math::{
        bounding::{Aabb2d, BoundingVolume},
        vec2, vec3, Vec2,
    },
    prelude::{
        Bundle, Commands, Component, Image, Plugin, Query, Res, ResMut, Resource, Transform, With,
    },
    sprite::SpriteBundle,
    time::{Time, Timer, TimerMode},
};

use crate::prelude::{Collider, Damage, YSpeed};

use super::ship::Ship;

const BASE_SHOT_DAMAGE: f32 = 1.0;
const SHOT_SPEED: f32 = 10.0;
const SHOOTING_INTERVAL: f32 = 0.5;
const SHOT_SPAWN_OFFSET: f32 = 35.0;
const SHOT_COLLIDER_SIZE: (f32, f32) = (9.0, 54.0);

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ShootingTimer(Timer::from_seconds(
            SHOOTING_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_shot_system, shot_moving_system));
    }
}

#[derive(Resource)]
struct ShootingTimer(Timer);

#[derive(Component)]
pub struct Shot;

#[derive(Bundle)]
struct ShotBundle {
    speed: YSpeed,
    sprite: SpriteBundle,
    collider: Collider,
    damage: Damage,
    shot: Shot,
}

impl ShotBundle {
    pub fn new(shot_offset: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_offset, y_offset) = shot_offset;

        Self {
            shot: Shot,
            speed: YSpeed(SHOT_SPEED),
            damage: Damage(BASE_SHOT_DAMAGE),
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

/// Spawns player shots on the screen every fixed amount of time
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

/// Handles the shots movement vertically
fn shot_moving_system(mut query: Query<(&mut Transform, &mut Collider, &YSpeed), With<Shot>>) {
    for (mut transform, mut collider, speed) in &mut query {
        // Move sprite
        transform.translation.y += speed.0;
        // Move collider
        collider.0.translate_by(vec2(0.0, speed.0));
    }
}
