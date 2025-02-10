pub mod resources;
mod systems;

use bevy::prelude::*;
pub use resources::DisplayedInfo;
use systems::{initialize_info, listen_drones_events, listen_leaves_events};

pub struct ListenerPlugin;

impl Plugin for ListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_info)
            .add_systems(Update, listen_drones_events)
            .add_systems(Update, listen_leaves_events);
    }
}
