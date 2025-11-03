use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

#[derive(Clone)]
struct Snowball {
    x: f32,
    y: f32,
    speed: f32,
}

impl Snowball {
    fn new(width: u16) -> Self {
        let mut rng = rand::thread_rng();
        Snowball {
            x: rng.gen_range(0.0..width as f32),
            y: 0.0,
            speed: rng.gen_range(0.5..2.0),
        }
    }

    fn update(&mut self) {
        self.y += self.speed;
    }

    fn is_off_screen(&self, height: u16) -> bool {
        self.y > height as f32
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Hide cursor
    execute!(stdout, Hide)?;

    // Get terminal size
    let (width, height) = crossterm::terminal::size()?;

    let mut snowballs: Vec<Snowball> = Vec::new();
    let mut frame_count = 0;

    loop {
        // Spawn new snowballs occasionally
        if frame_count % 5 == 0 {
            snowballs.push(Snowball::new(width));
        }

        // Update snowballs
        for snowball in &mut snowballs {
            snowball.update();
        }

        // Remove snowballs that are off screen
        snowballs.retain(|s| !s.is_off_screen(height));

        // Clear screen
        execute!(stdout, Clear(ClearType::All))?;

        // Draw snowballs
        for snowball in &snowballs {
            let x = snowball.x as u16;
            let y = snowball.y as u16;
            if x < width && y < height {
                execute!(stdout, MoveTo(x, y), Print("●"))?;
            }
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
