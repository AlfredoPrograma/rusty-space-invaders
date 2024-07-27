use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Handle},
    math::vec3,
    prelude::{Bundle, Commands, Component, Image, Res, Transform},
    sprite::SpriteBundle,
};

use crate::default_config::{WINDOW_X_LIMIT, WINDOW_Y_LIMIT, WINDOW_Y_PADDING};

// TODO: check how hell are hearts rendering?
// I've done some calculations and added some gap and positioning but really
// I'm not sure how it is working
const USER_LIVES_AMOUNT: i8 = 3;
const HEARTS_GAP: f32 = 35.0;

pub struct HeartsPlugin;

impl Plugin for HeartsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_lives_system);
    }
}

/// Provides the `Heart` attribute.
///
/// It represents the user's health.
#[derive(Component)]
struct Heart(pub i8);

/// Represents the heart element of the players hearts counter
#[derive(Bundle)]
struct HeartBundle {
    sprite: SpriteBundle,
    hearth: Heart,
}

impl HeartBundle {
    fn new(heart_value: i8, x_position: f32, texture: Handle<Image>) -> Self {
        HeartBundle {
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    translation: vec3(
                        WINDOW_X_LIMIT + x_position - HEARTS_GAP,
                        WINDOW_Y_LIMIT - WINDOW_Y_PADDING,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            hearth: Heart(heart_value),
        }
    }
}

/// Spawns the lives of the user and renders in screen using hearts
fn create_lives_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Need to iterate from one based values to give correct values
    // to hearts and position for sprites
    for value in 0..USER_LIVES_AMOUNT {
        let heart_texture: Handle<Image> = asset_server.load("player_heart.png");

        commands.spawn(HeartBundle::new(
            value + 1,
            HEARTS_GAP * f32::from(value),
            heart_texture,
        ));
    }
}
