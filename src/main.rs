use default_config::DefaultConfigPlugins;

mod default_config;

fn main() {
    bevy::app::App::new()
        .add_plugins(DefaultConfigPlugins)
        .run();
}
