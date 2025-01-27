use bevy::prelude::*;

use common_structs::leaf::LeafEvent;
use crossbeam_channel::Receiver;
use wg_2024::controller::DroneEvent;

//Use to listen events from the network
#[derive(Resource)]
pub struct Listeners {
    pub drone_listener: Receiver<DroneEvent>,
    pub leaf_listener: Receiver<LeafEvent>,
}
