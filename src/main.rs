use bevy::app::Plugin;

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        println!("Hello")
    }
}

fn main() {
    bevy::app::App::new().add_plugins(HelloPlugin).run();
}
