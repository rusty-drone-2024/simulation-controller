use bevy::prelude::*;
use network_initializer::initialize_default_network;

use super::structs::RustySC;

pub struct SpawnTopology;

impl Plugin for SpawnTopology {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_sc);
    }
}

fn initialize_sc(mut commands: Commands) {
    let network = initialize_default_network("config.toml");
    commands.spawn(RustySC::new(network));
}
