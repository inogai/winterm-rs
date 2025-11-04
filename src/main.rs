mod args;
mod entity;
mod game;
mod snowball;

use args::Args;
use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    execute,
};
use game::Game;
use std::{io::stdout, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Hide cursor
    execute!(stdout, Hide)?;

    // Get terminal size
    let (width, height) = crossterm::terminal::size()?;

    let args = Args::parse();

    let mut game = Game::new(
        width,
        height,
        args.snowball_chance,
        args.snowball_cluster_size,
        args.snowball_speed,
        args.duration,
        Duration::from_millis((1000.0 / args.fps) as u64),
        args.spawn_interval,
    );
    game.run(&mut stdout)?;

    // Show cursor again
    execute!(stdout, Show)?;

    Ok(())
}
