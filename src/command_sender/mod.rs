/// This module contains the command sender plugin.
/// The command sender plugin is responsible for sending commands to the drones and leaves.
mod components;
pub mod sender_trait;
mod systems;

use bevy::prelude::*;

pub use sender_trait::SenderTraitPlugin;
use systems::crash;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SenderTraitPlugin);
        app.add_systems(Update, crash);
    }
}
