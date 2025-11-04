mod entity;
mod snowball;

use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType},
};
use entity::Entity;
use rand::Rng;
use snowball::Snowball;
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short = 'r', default_value_t = 0.3)]
    snowball_chance: f64,

    #[arg(long, short = 's', default_value_t = 3)]
    snowball_cluster_size: u16,

    #[arg(long, short = 't', default_value_t = 1000)]
    max_frames: u32,

    #[arg(long, short = 'f', default_value_t = 100)]
    frame_delay_ms: u64,
}

struct Game {
    objects: Vec<Box<dyn Entity>>,
    rng: rand::rngs::ThreadRng,
    frame_count: u32,
    width: u16,
    height: u16,
    snowball_chance: f64,
    snowball_cluster_size: u16,
    max_frames: u32,
    frame_delay_ms: u64,
}

impl Game {
    fn new(
        width: u16,
        height: u16,
        snowball_chance: f64,
        snowball_cluster_size: u16,
        max_frames: u32,
        frame_delay_ms: u64,
    ) -> Self {
        Self {
            objects: Vec::new(),
            rng: rand::thread_rng(),
            frame_count: 0,
            width,
            height,
            snowball_chance,
            snowball_cluster_size,
            max_frames,
            frame_delay_ms,
        }
    }

    fn spawn_snowballs(&mut self) {
        if self.rng.gen_bool(self.snowball_chance) {
            let count = self.rng.gen_range(1..=self.snowball_cluster_size);
            for _ in 0..count {
                self.objects.push(Box::new(Snowball::new(self.width)));
            }
        }
    }

    fn update_objects(&mut self) {
        for obj in &mut self.objects {
            obj.update();
        }
    }

    fn remove_off_screen_objects(&mut self) {
        self.objects.retain(|obj| !obj.is_off_screen(self.height));
    }

    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for obj in &self.objects {
            obj.render(stdout)?;
        }
        Ok(())
    }

    fn run(&mut self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.spawn_snowballs();
            self.update_objects();
            self.remove_off_screen_objects();

            // Clear screen
            execute!(stdout, Clear(ClearType::All))?;

            self.render(stdout)?;

            // Flush output
            stdout.flush()?;

            // Sleep for animation
            thread::sleep(Duration::from_millis(self.frame_delay_ms));

            self.frame_count += 1;

            // Exit after some time (optional)
            if self.frame_count >= self.max_frames {
                break;
            }
        }
        Ok(())
    }
}

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
        args.max_frames,
        args.frame_delay_ms,
    );
    game.run(&mut stdout)?;

    // Show cursor again
    execute!(stdout, Show)?;

    Ok(())
}
