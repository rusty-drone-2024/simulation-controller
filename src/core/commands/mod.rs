use bevy::prelude::*;

pub mod drone_commands;
pub mod utils;

use drone_commands::crash;
use utils::CommandPlugin;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommandPlugin);
        app.add_systems(Update, crash);
    }
}
