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
