use bevy::prelude::*;
use std::time::Duration;

use crate::args::Args;

/// Game configuration resource
#[derive(Resource)]
pub struct GameConfig {
    pub width: u16,
    pub height: u16,
    pub snowball_chance: f64,
    pub snowball_speed: f32,
    pub spawn_interval: Duration,
    pub last_spawn_time: Duration,
}

impl GameConfig {
    pub fn new(args: Args, width: u16, height: u16) -> Self {
        GameConfig {
            width,
            height,
            snowball_chance: args.snowball_chance,
            snowball_speed: args.snowball_speed,
            spawn_interval: args.spawn_interval,
            last_spawn_time: Duration::ZERO,
        }
    }
}

/// Stdout resource for terminal output
#[derive(Resource)]
pub struct StdoutResource(pub std::io::Stdout);

/// RNG resource for random number generation
#[derive(Resource)]
pub struct RngResource(pub rand::rngs::StdRng);
