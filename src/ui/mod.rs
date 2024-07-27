use bevy::app::Plugin;
use hearts::HeartsPlugin;
use score::ScorePlugin;

pub mod hearts;
pub mod score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ScorePlugin, HeartsPlugin));
    }
}
