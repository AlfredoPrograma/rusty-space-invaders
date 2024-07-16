use default_config::DefaultConfigPlugins;
use ship::ShipPlugin;

mod default_config;
mod ship;

fn main() {
    bevy::app::App::new()
        .add_plugins((DefaultConfigPlugins, ShipPlugin))
        .run();
}
