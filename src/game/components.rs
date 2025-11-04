use bevy::prelude::*;

/// Position component representing x,y coordinates
#[derive(Clone, Copy, Component)]
pub struct Position(pub Vec2);

/// Velocity component representing movement per second
#[derive(Clone, Copy, Component)]
pub struct Velocity(pub Vec2);

/// Text component for rendering character
#[derive(Component)]
pub struct Text(pub String);

/// Bundle for spawning a snowball entity
#[derive(Bundle)]
pub struct SnowballBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub text: Text,
}

impl SnowballBundle {
    /// Creates a new snowball at the given x position with the specified speed
    pub fn new(x: f32, speed: f32) -> Self {
        Self {
            position: Position(Vec2::new(x, 0.0)),
            velocity: Velocity(Vec2::new(0.0, speed)),
            text: Text("●".to_string()),
        }
    }
}
