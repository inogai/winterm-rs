use crate::entity::Entity;
use crate::snowball::Snowball;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::{
    io::Write,
    thread,
    time::{Duration, Instant},
};

pub struct Game {
    pub objects: Vec<Box<dyn Entity>>,
    rng: rand::rngs::ThreadRng,
    start_time: Instant,
    pub width: u16,
    pub height: u16,
    snowball_chance: f64,
    snowball_cluster_size: u16,
    duration: Duration,
    frame_delay: Duration,
    spawn_interval: Duration,
    last_spawn_time: Instant,
}

impl Game {
    pub fn new(
        width: u16,
        height: u16,
        snowball_chance: f64,
        snowball_cluster_size: u16,
        duration: Duration,
        frame_delay_ms: Duration,
        spawn_interval: Duration,
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
            spawn_interval,
            last_spawn_time: Instant::now(),
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

    pub fn run(&mut self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if self.last_spawn_time.elapsed() >= self.spawn_interval {
                self.spawn_snowballs();
                self.last_spawn_time = Instant::now();
            }
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

