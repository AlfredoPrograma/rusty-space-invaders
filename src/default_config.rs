use std::borrow::BorrowMut;

use bevy::{app::Plugin, math::vec3, prelude::*, window, DefaultPlugins};

use crate::prelude::Score;

pub const WINDOW_X_SIZE: f32 = 500.0;
pub const WINDOW_Y_SIZE: f32 = 900.0;

const WINDOW_X_PADDING: f32 = 60.0;
pub const WINDOW_X_LIMIT: f32 = (WINDOW_X_SIZE / 2.0) - WINDOW_X_PADDING;
pub const WINDOW_Y_LIMIT: f32 = WINDOW_Y_SIZE / 2.0;
pub const WINDOW_Y_PADDING: f32 = 30.0;

pub struct DefaultConfigPlugins;

impl Plugin for DefaultConfigPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        let window_plugin = create_window_plugin();

        app.add_plugins(DefaultPlugins.set(window_plugin))
            .add_systems(
                Startup,
                (
                    create_camera_system,
                    create_score_system,
                    render_score_system,
                )
                    .chain(),
            );
    }
}

fn create_window_plugin() -> WindowPlugin {
    let window_config = Window {
        title: "Rusty Space Invaders".into(),
        name: Some("rusty-spacy-invaders.app".into()),
        resolution: (WINDOW_X_SIZE, WINDOW_Y_SIZE).into(),
        enabled_buttons: window::EnabledButtons {
            maximize: false,
            ..Default::default()
        },
        ..Default::default()
    };

    WindowPlugin {
        primary_window: Some(window_config),
        ..Default::default()
    }
}

fn create_camera_system(mut commands: Commands) {
    let camera = Camera2dBundle {
        ..Default::default()
    };

    commands.spawn(camera);
}

fn create_score_system(mut commands: Commands) {
    commands.spawn(Score(0));
}

fn render_score_system(mut commands: Commands, query: Query<&Score, With<Score>>) {
    let score = query.get_single().expect("only should exists one score");

    let score_text = Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: format!("SCORE: {}", score.0),
                ..Default::default()
            }],
            ..Default::default()
        },
        transform: Transform {
            translation: vec3(WINDOW_X_PADDING, WINDOW_Y_LIMIT - WINDOW_Y_PADDING, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(score_text);
}
