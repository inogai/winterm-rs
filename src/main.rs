mod args;
mod game;

use args::Args;
use bevy::app::{App, AppExit};
use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{Write, stdout},
    time::{Duration, Instant},
};

use crate::game::GameConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Parse command line arguments
    let args = Args::parse();

    // Get terminal size
    let (width, height) = crossterm::terminal::size()?;

    // Enable raw mode and hide cursor
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    // Store configuration for the runner
    let duration = args.duration;
    let frame_delay = Duration::from_millis((1000.0 / args.fps) as u64);

    // Create the Bevy app with all systems
    let mut app = game::create_app(GameConfig::new(args, width, height));

    // Set custom runner that manages the terminal refresh
    app.set_runner(move |mut app: App| {
        let mut term_stdout = std::io::stdout();
        let start_time = Instant::now();

        loop {
            let frame_start = Instant::now();

            // Check for keyboard input (Ctrl-C, q, or Esc to exit)
            if event::poll(Duration::ZERO).unwrap_or(false)
                && let Ok(Event::Key(key_event)) = event::read()
            {
                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break AppExit::Success;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        break AppExit::Success;
                    }
                    _ => {}
                }
            }

            // Clear screen and run frame
            let _ = execute!(term_stdout, Clear(ClearType::All));
            app.update();
            let _ = term_stdout.flush();

            // Frame pacing - sleep for remaining time to maintain consistent frame rate
            let frame_time = frame_start.elapsed();
            if frame_time < frame_delay {
                std::thread::sleep(frame_delay - frame_time);
            }

            // Exit after duration
            if start_time.elapsed() >= duration {
                break AppExit::Success;
            }
        }
    });

    // Run the app with custom runner
    app.run();

    // Cleanup: Show cursor and disable raw mode
    execute!(stdout, Show)?;
    disable_raw_mode()?;

    Ok(())
}
