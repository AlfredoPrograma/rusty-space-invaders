use bevy::app::Plugin;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_SIZE},
    prelude::XSpeed,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_ship_system)
            .add_systems(Update, ship_movement_system);
    }
}

#[derive(Component)]
pub struct Ship;

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

/// Spawns the player ship in the screen.
fn spawn_ship_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle: Handle<Image> = asset_server.load("player_ship.png");

    commands.spawn(ShipBundle::new(ship_handle));
}

/// Handles the user input to move the ship horizontally in the screen
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

            // Do nothing for other key presses
            _ => {}
        }
    }
}
