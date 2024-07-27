use default_config::DefaultConfigPlugins;
use enemy::EnemiesPlugin;
use player::ship::ShipPlugin;
use ui::score::ScorePlugin;

mod default_config;
mod enemy;
mod player;
mod prelude;
mod ui;

fn main() {
    bevy::app::App::new()
        .add_plugins((DefaultConfigPlugins, ScorePlugin, ShipPlugin, EnemiesPlugin))
        .run();
}
