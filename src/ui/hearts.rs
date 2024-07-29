use bevy::{
    app::{Plugin, PostUpdate, Startup},
    asset::{AssetServer, Handle},
    math::vec3,
    prelude::{
        Bundle, Commands, Component, Entity, EventReader, Image, Query, Res, Transform, With,
    },
    sprite::SpriteBundle,
};

use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_LIMIT, WINDOW_Y_PADDING},
    player::ship::ShipTakeDamageEvent,
};

// TODO: check how hell are hearts rendering?
// I've done some calculations and added some gap and positioning but really
// I'm not sure how it is working
const USER_LIVES_AMOUNT: i8 = 3;
const HEARTS_GAP: f32 = 35.0;

pub struct HeartsPlugin;

impl Plugin for HeartsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_lives_system)
            .add_systems(PostUpdate, decrease_life_system);
    }
}

/// Provides the `Heart` attribute
///
/// It represents the user's health
#[derive(Component)]
struct Heart;

/// Represents the heart element of the players hearts counter
#[derive(Bundle)]
struct HeartBundle {
    sprite: SpriteBundle,
    heart: Heart,
}

impl HeartBundle {
    fn new(x_position: f32, texture: Handle<Image>) -> Self {
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
            heart: Heart,
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
            HEARTS_GAP * f32::from(value),
            heart_texture,
        ));
    }
}

/// Reduces player's lives by one
fn decrease_life_system(
    lives_query: Query<Entity, With<Heart>>,
    mut ship_take_damage_event_rx: EventReader<ShipTakeDamageEvent>,
    mut commands: Commands,
) {
    for event in ship_take_damage_event_rx.read() {
        // Despawn the collided enemy
        commands.entity(event.0).despawn();

        // Skip one heart bundle because we want make player lose on 3 collisions
        if let Some(heart_entity) = lives_query.iter().skip(1).next() {
            // Despawn heart
            commands.entity(heart_entity).despawn();
        } else {
            panic!("Implement losing game state")
        }
    }
}
