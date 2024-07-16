use bevy::prelude::*;
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        println!("Hello enemies plugin")
    }
}
