use default_config::DefaultConfigPlugins;
use enemies::EnemiesPlugin;
use ship::ShipPlugin;

mod default_config;
mod enemies;
mod prelude;
mod ship;

fn main() {
    bevy::app::App::new()
        .add_plugins((DefaultConfigPlugins, ShipPlugin, EnemiesPlugin))
        .run();
}
