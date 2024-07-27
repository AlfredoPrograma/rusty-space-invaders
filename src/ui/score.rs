use bevy::{
    app::{Plugin, Startup, Update},
    math::vec3,
    prelude::{Bundle, Commands, Component, EventReader, Query, Transform, With},
    text::{Text, Text2dBundle, TextSection},
};

use crate::{
    default_config::{WINDOW_X_LIMIT, WINDOW_Y_LIMIT, WINDOW_Y_PADDING},
    enemy::EnemyDiedEvent,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_score_system)
            .add_systems(Update, increase_score_listener);
    }
}

#[derive(Component)]
pub struct Score(pub u32);

#[derive(Bundle)]
struct ScoreBundle {
    text: Text2dBundle,
    score: Score,
}

impl ScoreBundle {
    fn new() -> Self {
        let score_counter = Score(0);
        let score_text = Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("SCORE: {}", score_counter.0),
                    ..Default::default()
                }],
                ..Default::default()
            },
            transform: Transform {
                translation: vec3(-WINDOW_X_LIMIT, WINDOW_Y_LIMIT - WINDOW_Y_PADDING, 0.0),
                ..Default::default()
            },
            ..Default::default()
        };

        ScoreBundle {
            text: score_text,
            score: score_counter,
        }
    }
}

/// Spawns score counter in the screen
fn create_score_system(mut commands: Commands) {
    commands.spawn(ScoreBundle::new());
}

/// Listens for enemy deads and increases the score based on the defeated enemy kind
fn increase_score_listener(
    mut enemy_died_event_rx: EventReader<EnemyDiedEvent>,
    mut score_query: Query<(&mut Score, &mut Text), With<Score>>,
) {
    for event in enemy_died_event_rx.read() {
        if let Ok(score) = score_query.get_single_mut() {
            let (mut score_counter, mut score_text) = score;

            score_counter.0 += event.0.score();
            score_text.sections = vec![TextSection {
                value: format!("SCORE: {}", score_counter.0),
                ..Default::default()
            }];
        }
    }
}
