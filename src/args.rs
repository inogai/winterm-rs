use clap::Parser;
use std::time::Duration;

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

#[derive(Parser, bevy::prelude::Resource)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short = 'p', default_value_t = 0.3)]
    pub spawn_chance: f32,

    #[arg(long, short = 'm', default_value_t = 0.0)]
    pub snowball_mean: f64,

    #[arg(long, short = 't', default_value_t = 2.0)]
    pub snowball_std: f64,

    #[arg(long, short = 'v', default_value_t = 5.0)]
    /// rows per s
    pub snowball_speed: f32,

    #[arg(long, short = 'd', value_parser=duration_parser, default_value="1d")]
    pub duration: Duration,

    #[arg(long, short = 'i', value_parser=duration_parser, default_value="200ms")]
    pub spawn_interval: Duration,
    
    #[arg(long, short = 'f', default_value_t = 30.0)]
    pub fps: f32,
}
