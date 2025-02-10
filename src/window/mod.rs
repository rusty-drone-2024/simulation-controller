mod resources;
mod systems;

use bevy::prelude::*;
use systems::{initialize_ui_state, window};
pub use systems::{observer_drone, observer_leaf};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui_state)
            .add_systems(Update, window);
    }
}
