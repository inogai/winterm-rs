use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
};
use std::io::Stdout;

pub trait Entity {
    fn update(&mut self);
    fn pos(&self) -> (f32, f32);
    fn text(&self) -> &str;

    fn render(&self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let (x, y) = self.pos();
        let x = x as u16;
        let y = y as u16;
        execute!(stdout, MoveTo(x, y), Print(self.text()))?;
        Ok(())
    }
    fn is_off_screen(&self, height: u16) -> bool;
}

