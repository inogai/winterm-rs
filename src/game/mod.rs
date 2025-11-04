mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use rand::SeedableRng;

// Re-export only what's needed by external modules
pub use resources::GameConfig;

// Internal imports for systems
use resources::{RngResource, StdoutResource};
use systems::{remove_off_screen, render_system, spawn_snowballs, update_positions};

/// Creates and configures the Bevy app with all necessary systems and resources
pub fn create_app(config: GameConfig) -> App {
    let mut app = App::new();

    app.add_plugins(bevy::time::TimePlugin)
        .insert_resource(config)
        .insert_resource(StdoutResource(std::io::stdout()))
        .insert_resource(RngResource(rand::rngs::StdRng::from_os_rng()))
        .add_systems(
            Update,
            (
                spawn_snowballs,
                update_positions,
                remove_off_screen,
                render_system,
            ),
        );

    app
}
