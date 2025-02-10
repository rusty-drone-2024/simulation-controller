/// This module contains the window plugin and its systems.
/// The window plugin is responsible for showing all the info on the side panel.
/// It's also responsible for triggering other events if the user clicks on the buttons entities etc.
///
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
