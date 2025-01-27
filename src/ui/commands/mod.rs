use bevy::prelude::*;

pub mod drone_commands;
mod leaf_commands;
mod utils;

use drone_commands::crash;



pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crash);
    }
}
