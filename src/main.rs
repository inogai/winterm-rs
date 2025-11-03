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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Hide cursor
    execute!(stdout, Hide)?;

    // Get terminal size
    let (width, height) = crossterm::terminal::size()?;

    let mut objects: Vec<Box<dyn Entity>> = Vec::new();
    let mut frame_count = 0;

    let mut rng = rand::thread_rng();

    loop {
        if rng.gen_bool(SNOWBALL_CHANCE) {
            let count = rng.gen_range(1..=SNOWBALL_CLUSTER_SIZE);

            for _ in 0..count {
                objects.push(Box::new(Snowball::new(width)));
            }
        }

        // Update objects
        for obj in &mut objects {
            obj.update();
        }

        // Remove objects that are off screen
        objects.retain(|obj| !obj.is_off_screen(height));

        // Clear screen
        execute!(stdout, Clear(ClearType::All))?;

        // Draw objects
        for obj in &objects {
            obj.render(&mut stdout)?;
        }

        // Flush output
        stdout.flush()?;

        // Sleep for animation
        thread::sleep(Duration::from_millis(100));

        frame_count += 1;

        // Exit after some time (optional)
        if frame_count > 1000 {
            break;
        }
    }

    // Show cursor again
    execute!(stdout, Show)?;

    Ok(())
}
