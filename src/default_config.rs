use bevy::{app::Plugin, prelude::*, window, DefaultPlugins};

pub const WINDOW_X_SIZE: f32 = 500.0;
pub const WINDOW_Y_SIZE: f32 = 900.0;

pub struct DefaultConfigPlugins;

impl Plugin for DefaultConfigPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        let window_plugin = create_window_plugin();

        app.add_plugins(DefaultPlugins.set(window_plugin));
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
