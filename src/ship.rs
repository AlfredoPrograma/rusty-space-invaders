use bevy::app::Plugin;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::prelude::{XSpeed, YSpeed};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ShootingTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, spawn_ship_system)
        .add_systems(Update, (ship_movement_system, spawn_shot_system));
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

impl ShipBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            ship: Ship,
            speed: XSpeed(SHIP_SPEED),
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    scale: vec3(SHIP_SCALE, SHIP_SCALE, 0.0),
                    translation: vec3(0.0, 0.0, 0.0),
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
            KeyCode::ArrowLeft => transform.translation.x -= speed.0,
            KeyCode::ArrowRight => transform.translation.x += speed.0,
            // Do nothing xd
            _ => {}
        }
    }
}

const SHOT_SPEED: f32 = 10.0;

#[derive(Resource)]
struct ShootingTimer(Timer);

#[derive(Component)]
struct Shot;

#[derive(Bundle)]
struct ShotBundle {
    speed: YSpeed,
    sprite: SpriteBundle,
    shot: Shot,
}

impl ShotBundle {
    pub fn new(shot_offset: (f32, f32), texture: Handle<Image>) -> Self {
        let (x_offset, y_offset) = shot_offset;

        Self {
            shot: Shot,
            speed: YSpeed(SHOT_SPEED),
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
    let transform = query.get_single().unwrap();
    let (x_offset, y_offset) = (
        transform.translation.x + 35.0,
        transform.translation.y + 35.0,
    );

    if shooting_timer.0.tick(time.delta()).just_finished() {
        commands.spawn(ShotBundle::new((-x_offset, y_offset), shot_texture.clone()));
        commands.spawn(ShotBundle::new((x_offset, y_offset), shot_texture));
    }
}
