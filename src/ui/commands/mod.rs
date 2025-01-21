use bevy::prelude::*;

mod drone_commands;
mod leaf_commands;
mod utils;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
