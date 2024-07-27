use asteroid::{
    asteroids_movement_system, spawn_asteroids_system, AsteroidSpawnTimer, ASTEROID_SCORE,
    ASTEROID_SPAWNER_TRIGGER_INTERVAL,
};
use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    math::bounding::IntersectsVolume,
    prelude::{Commands, Component, Entity, Event, EventWriter, IntoSystemConfigs, Query, With},
    time::{Timer, TimerMode},
};

use crate::{
    prelude::{Collider, Damage, Health},
    ship::Shot,
};

pub mod asteroid;

#[derive(Clone, Debug)]
/// Represents the kind of enemy of the game
pub enum EnemyKind {
    Asteroid,
}

impl EnemyKind {
    /// Gets the corresponding score based on the `EnemyKind`.
    pub fn score(&self) -> u32 {
        match self {
            EnemyKind::Asteroid => ASTEROID_SCORE,
        }
    }
}

/// Tags an entity as an `Enemy`.
#[derive(Component)]
pub struct Enemy(pub EnemyKind);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            ASTEROID_SPAWNER_TRIGGER_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_event::<EnemyDiedEvent>()
        .add_systems(
            Update,
            (
                spawn_asteroids_system,
                asteroids_movement_system,
                enemy_take_damage_system,
            )
                .chain(),
        )
        .add_systems(PostUpdate, check_enemy_died);
    }
}

/// Management systems for common properties between enemies

/// It takes care of compute if some `Shot` and some `Enemy` has been collided and then
/// reduces the health of the enemy.
pub fn enemy_take_damage_system(
    shot_query: Query<(&Damage, &Collider, Entity), With<Shot>>,
    mut enemy_query: Query<(&mut Health, &Collider), With<Enemy>>,
    mut commands: Commands,
) {
    for (mut enemy_health, enemy_collider) in &mut enemy_query {
        for (shot_damage, shot_collider, shot_entity) in &shot_query {
            if shot_collider.0.intersects(&enemy_collider.0) {
                enemy_health.0 -= shot_damage.0;

                // Instantly despawns shot which has been collided
                commands.entity(shot_entity).despawn();
            }
        }
    }
}

#[derive(Event)]
pub struct EnemyDiedEvent(pub EnemyKind);

/// Checks if enemies health and despawn them if it is equal or lower than 0.
pub fn check_enemy_died(
    enemies_query: Query<(&Health, &Enemy, Entity), With<Enemy>>,
    mut enemy_died_event_tx: EventWriter<EnemyDiedEvent>,
    mut commands: Commands,
) {
    for (enemy_health, enemy_kind, enemy_entity) in &enemies_query {
        if enemy_health.0 <= 0.0 {
            commands.entity(enemy_entity).despawn();
            enemy_died_event_tx.send(EnemyDiedEvent(enemy_kind.0.clone()));
        }
    }
}
