mod entity;
mod snowball;

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

const SNOWBALL_CHANCE: f64 = 0.3; // Percentage chance to spawn a snowball each frame
const SNOWBALL_CLUSTER_SIZE: u16 = 3;
const MAX_FRAMES: u32 = 1000;
const FRAME_DELAY_MS: u64 = 100;

struct Game {
    objects: Vec<Box<dyn Entity>>,
    rng: rand::rngs::ThreadRng,
    frame_count: u32,
    width: u16,
    height: u16,
}

impl Game {
    fn new(width: u16, height: u16) -> Self {
        Self {
            objects: Vec::new(),
            rng: rand::thread_rng(),
            frame_count: 0,
            width,
            height,
        }
    }

    fn spawn_snowballs(&mut self) {
        if self.rng.gen_bool(SNOWBALL_CHANCE) {
            let count = self.rng.gen_range(1..=SNOWBALL_CLUSTER_SIZE);
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
            thread::sleep(Duration::from_millis(FRAME_DELAY_MS));

            self.frame_count += 1;

            // Exit after some time (optional)
            if self.frame_count >= MAX_FRAMES {
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

    let mut game = Game::new(width, height);
    game.run(&mut stdout)?;

    // Show cursor again
    execute!(stdout, Show)?;

    Ok(())
}
