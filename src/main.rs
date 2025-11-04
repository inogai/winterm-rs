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
    time::{Duration, Instant},
};

fn duration_parser(s: &str) -> Result<Duration, String> {
    let mut total_ms = 0u64;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c.is_numeric() {
            let mut num_str = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let num: u64 = num_str
                .parse()
                .map_err(|_| format!("Invalid number: {}", num_str))?;
            let mut unit = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphabetic() {
                    unit.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let multiplier = match unit.as_str() {
                "" => 1000, // Default to seconds if no unit
                "d" => 24 * 60 * 60 * 1000,
                "h" => 60 * 60 * 1000,
                "min" => 60 * 1000,
                "s" => 1000,
                "ms" => 1,
                _ => return Err(format!("Unknown unit: {}", unit)),
            };
            total_ms = total_ms
                .checked_add(num.checked_mul(multiplier).ok_or("Overflow")?)
                .ok_or("Overflow")?;
        } else {
            return Err("Unexpected character".to_string());
        }
    }
    Ok(Duration::from_millis(total_ms))
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short = 'r', default_value_t = 0.3)]
    snowball_chance: f64,

    #[arg(long, short = 's', default_value_t = 3)]
    snowball_cluster_size: u16,

    #[arg(long, short = 'd', value_parser=duration_parser, default_value="1d")]
    duration: Duration,

    #[arg(long, short = 'f', default_value_t = 60.0)]
    fps: f64,
}

struct Game {
    objects: Vec<Box<dyn Entity>>,
    rng: rand::rngs::ThreadRng,
    start_time: Instant,
    width: u16,
    height: u16,
    snowball_chance: f64,
    snowball_cluster_size: u16,
    duration: Duration,
    frame_delay: Duration,
}

impl Game {
    fn new(
        width: u16,
        height: u16,
        snowball_chance: f64,
        snowball_cluster_size: u16,
        duration: Duration,
        frame_delay_ms: Duration,
    ) -> Self {
        Self {
            objects: Vec::new(),
            rng: rand::thread_rng(),
            start_time: Instant::now(),
            width,
            height,
            snowball_chance,
            snowball_cluster_size,
            duration,
            frame_delay: frame_delay_ms,
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
            thread::sleep(self.frame_delay);

            // Exit after some time (optional)
            if self.start_time.elapsed() >= self.duration {
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
        args.duration,
        Duration::from_millis((1000.0 / args.fps) as u64),
    );
    game.run(&mut stdout)?;

    // Show cursor again
    execute!(stdout, Show)?;

    Ok(())
}
