/// This module contains the small settings systems
/// This simple module just handles the small settings window of the simulation.
mod events;
mod resources;
mod systems;
pub use resources::{ModeConfig, MusicResource};

use bevy::prelude::*;
use events::{ModeEvent, MusicEvent, ResetInfosEvent};
use resources::StateResource;
use systems::{
    reset_infos, settings_window, spawn_soundtrack, update_soundtrack, update_unchecked,
};
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MusicResource {
            entity: None,
            playing: true,
        });
        app.insert_resource(StateResource { unchecked: false });
        app.insert_resource(ModeConfig {
            bypass_cheks: false,
        });
        app.add_event::<MusicEvent>();
        app.add_event::<ModeEvent>();
        app.add_event::<ResetInfosEvent>();
        app.add_systems(Update, settings_window);
        app.add_systems(Startup, spawn_soundtrack);
        app.add_systems(Update, update_soundtrack);
        app.add_systems(Update, update_unchecked);
        app.add_systems(Update, reset_infos);
    }
}
