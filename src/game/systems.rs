use bevy::prelude::*;
use crossterm::execute;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::components::{Position, SnowballBundle, Text, Velocity};
use super::resources::{GameConfig, RngResource, StdoutResource};

/// System to spawn snowballs at regular intervals
pub fn spawn_snowballs(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<GameConfig>,
    mut rng: ResMut<RngResource>,
) {
    if time.elapsed() - config.last_spawn_time >= config.spawn_interval {
        let normal = Normal::new(config.snowball_mean, config.snowball_std).unwrap();
        let count = (normal.sample(&mut rng.0).round() as i32).max(1) as usize;

        for _ in 0..count {
            let x = rng.0.random_range(0.0..config.width as f32);
            commands.spawn(SnowballBundle::new(x, config.snowball_speed));
        }

        config.last_spawn_time = time.elapsed();
    }
}

/// System to update positions based on velocity and delta time
pub fn update_positions(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let delta = time.delta_secs();
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0 * delta;
    }
}

/// System to remove entities that have moved off screen
pub fn remove_off_screen(
    mut commands: Commands,
    query: Query<(Entity, &Position)>,
    config: Res<GameConfig>,
) {
    for (entity, pos) in query.iter() {
        if pos.0.y > config.height as f32 {
            commands.entity(entity).despawn();
        }
    }
}

/// System to render all entities to terminal
pub fn render_system(query: Query<(&Position, &Text)>, mut stdout: ResMut<StdoutResource>) {
    for (pos, text) in query.iter() {
        let x = pos.0.x as u16;
        let y = pos.0.y as u16;
        let _ = execute!(
            stdout.0,
            crossterm::cursor::MoveTo(x, y),
            crossterm::style::Print(&text.0)
        );
    }
}
