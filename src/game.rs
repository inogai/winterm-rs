use bevy::prelude::*;
use crossterm::execute;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use std::time::Duration;

use crate::args::Args;

#[derive(Clone, Copy, Component)]
pub struct Position(pub Vec2);

#[derive(Clone, Copy, Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Text(pub String);

#[derive(Resource)]
pub struct GameConfig {
    pub width: u16,
    pub height: u16,
    pub snowball_mean: f64,
    pub snowball_std: f64,
    pub snowball_speed: f32,
    pub spawn_interval: Duration,
    pub last_spawn_time: Duration,
}

impl GameConfig {
    pub fn new(args: Args, width: u16, height: u16) -> Self {
        GameConfig {
            width,
            height,
            snowball_mean: args.snowball_mean,
            snowball_std: args.snowball_std,
            snowball_speed: args.snowball_speed,
            spawn_interval: args.spawn_interval,
            last_spawn_time: Duration::ZERO,
        }
    }
}

#[derive(Resource)]
pub struct StdoutResource(pub std::io::Stdout);

#[derive(Resource)]
pub struct RngResource(pub rand::rngs::StdRng);

fn spawn_snowballs(
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
            commands.spawn((
                Position(Vec2::new(x, 0.0)),
                Velocity(Vec2::new(0.0, config.snowball_speed)),
                Text("●".to_string()),
            ));
        }
        config.last_spawn_time = time.elapsed();
    }
}

fn update_positions(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let delta = time.delta_secs();
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0 * delta;
    }
}

fn remove_off_screen(
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

fn render_system(query: Query<(&Position, &Text)>, mut stdout: ResMut<StdoutResource>) {
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

pub fn create_app(config: GameConfig) -> App {
    let mut app = App::new();

    // Add minimal Bevy plugins needed for Time and basic ECS functionality
    app.add_plugins(bevy::time::TimePlugin)
        .insert_resource(config)
        .insert_resource(StdoutResource(std::io::stdout()))
        .insert_resource(RngResource(rand::rngs::StdRng::from_os_rng()))
        .add_systems(Update, spawn_snowballs)
        .add_systems(Update, update_positions)
        .add_systems(Update, remove_off_screen)
        .add_systems(Update, render_system);

    app
}
