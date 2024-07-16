use bevy::app::Plugin;
use bevy::math::vec3;
use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_ship_system);
    }
}

#[derive(Component)]
struct Ship;

#[derive(Bundle)]
struct ShipBundle {
    sprite: SpriteBundle,
    ship: Ship,
}

impl ShipBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            ship: Ship,
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    scale: vec3(0.75, 0.75, 0.0),
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
