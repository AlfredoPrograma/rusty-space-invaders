use default_config::DefaultConfigPlugins;
use enemies::EnemiesPlugin;
use score::ScorePlugin;
use ship::ShipPlugin;

mod default_config;
mod enemies;
mod prelude;
mod score;
mod ship;

fn main() {
    bevy::app::App::new()
        .add_plugins((DefaultConfigPlugins, ScorePlugin, ShipPlugin, EnemiesPlugin))
        .run();
}
