use bevy::math::bounding::{BoundingVolume, IntersectsVolume};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::{app::Plugin, math::bounding::Aabb2d};

use crate::enemy::Enemy;
use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_SIZE},
    prelude::{Collider, XSpeed},
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ShipTakeDamageEvent>()
            .add_systems(Startup, spawn_ship_system)
            .add_systems(Update, (ship_movement_system, ship_take_damage_listener));
    }
}

#[derive(Component)]
pub struct Ship;

#[derive(Bundle)]
struct ShipBundle {
    speed: XSpeed,
    sprite: SpriteBundle,
    ship: Ship,
    collider: Collider,
}

const SHIP_SPEED: f32 = 5.0;
const SHIP_SCALE: f32 = 0.75;
const SHIP_POSITION: f32 = -(WINDOW_Y_SIZE / 2.0) + 75.0;
const SHIP_COLLIDER_SIZE: (f32, f32) = (99.0, 75.0);

impl ShipBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            ship: Ship,
            collider: Collider(Aabb2d::new(
                vec2(0.0, SHIP_POSITION),
                Vec2::from(SHIP_COLLIDER_SIZE) / 2.0,
            )),
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

/// Spawns the player ship in the screen.
fn spawn_ship_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle: Handle<Image> = asset_server.load("player_ship.png");

    commands.spawn(ShipBundle::new(ship_handle));
}

/// Handles the user input to move the ship horizontally in the screen
fn ship_movement_system(
    mut query: Query<(&mut Transform, &mut Collider, &XSpeed), With<Ship>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut collider, speed) = query.get_single_mut().unwrap();

    for key_code in key.get_pressed() {
        match key_code {
            KeyCode::ArrowLeft => {
                let new_position = transform.translation.x - speed.0;

                if !(new_position < -WINDOW_X_LIMIT) {
                    transform.translation.x = new_position;
                    collider.0.translate_by(vec2(-speed.0, 0.0));
                }
            }
            KeyCode::ArrowRight => {
                let new_position = transform.translation.x + speed.0;

                if !(new_position > WINDOW_X_LIMIT) {
                    transform.translation.x = new_position;
                    collider.0.translate_by(vec2(speed.0, 0.0));
                }
            }

            // Do nothing for other key presses
            _ => {}
        }
    }
}

#[derive(Event)]
pub struct ShipTakeDamageEvent(pub Entity);

/// Listens for enemies and ship or barrier collisions and dispatchs `ShipTakeDamage` event.
fn ship_take_damage_listener(
    enemies_query: Query<(&Collider, Entity), With<Enemy>>,
    ship_query: Query<&Collider, With<Ship>>,
    mut ship_take_damage_event_tx: EventWriter<ShipTakeDamageEvent>,
) {
    for (enemy_collider, enemy_entity) in &enemies_query {
        if let Ok(ship_collider) = ship_query.get_single() {
            if ship_collider.0.intersects(&enemy_collider.0) {
                ship_take_damage_event_tx.send(ShipTakeDamageEvent(enemy_entity));
            }
        }
    }
}
