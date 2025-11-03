mod entity;
mod snowball;

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType},
};
use entity::Entity;
use snowball::Snowball;
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Hide cursor
    execute!(stdout, Hide)?;

    // Get terminal size
    let (width, height) = crossterm::terminal::size()?;

    let mut objects: Vec<Box<dyn Entity>> = Vec::new();
    let mut frame_count = 0;

    loop {
        // Spawn new snowballs occasionally
        if frame_count % 5 == 0 {
            objects.push(Box::new(Snowball::new(width)));
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
