use clap::Parser;
use std::time::Duration;

/// Parses duration strings like "3s", "5min", "1d", etc.
fn duration_parser(s: &str) -> Result<Duration, String> {
    let mut total_ms = 0u64;
    let mut chars = s.chars().peekable();

    while chars.peek().is_some() {
        // Skip whitespace
        if chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
            continue;
        }

        // Parse number
        if chars.peek().is_some_and(|c| c.is_numeric()) {
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

            // Parse unit
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
                "" | "s" => 1000, // Default to seconds
                "ms" => 1,
                "min" => 60 * 1000,
                "h" => 60 * 60 * 1000,
                "d" => 24 * 60 * 60 * 1000,
                _ => return Err(format!("Unknown unit: '{}'", unit)),
            };

            total_ms = total_ms
                .checked_add(num.checked_mul(multiplier).ok_or("Duration overflow")?)
                .ok_or("Duration overflow")?;
        } else {
            return Err(format!("Unexpected character in duration string"));
        }
    }

    Ok(Duration::from_millis(total_ms))
}

/// Command line arguments for the snowball animation
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short = 'm', default_value_t = 0.0)]
    /// Mean number of snowballs to spawn per interval (normal distribution)
    pub snowball_mean: f64,

    #[arg(long, short = 't', default_value_t = 2.0)]
    /// Standard deviation for snowball spawn count (normal distribution)
    pub snowball_std: f64,

    #[arg(long, short = 'v', default_value_t = 2.0)]
    /// Snowball fall speed in rows per second
    pub snowball_speed: f32,

    #[arg(long, short = 'd', value_parser=duration_parser, default_value="1d")]
    /// Duration to run the animation (e.g., "3s", "5min", "1d")
    pub duration: Duration,

    #[arg(long, short = 'i', value_parser=duration_parser, default_value="300ms")]
    /// Interval between snowball spawns (e.g., "100ms", "1s")
    pub spawn_interval: Duration,

    #[arg(long, short = 'f', default_value_t = 30.0)]
    /// Target frames per second (FPS)
    pub fps: f32,
}
