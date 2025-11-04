use super::entity::Entity;
use rand::Rng;

#[derive(Clone)]
pub struct Snowball {
    x: f32,
    y: f32,
    speed: f32,
}

impl Snowball {
    pub fn new(width: u16) -> Self {
        let mut rng = rand::thread_rng();
        Snowball {
            x: rng.gen_range(0.0..width as f32),
            y: 0.0,
            speed: rng.gen_range(0.5..2.0),
        }
    }
}

impl Entity for Snowball {
    fn update(&mut self) {
        self.y += self.speed;
    }

    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn text(&self) -> &str {
        "●"
    }

    fn is_off_screen(&self, height: u16) -> bool {
        self.y > height as f32
    }
}
