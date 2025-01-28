use bevy::prelude::*;

use common_structs::leaf::LeafEvent;
use crossbeam_channel::Receiver;
use wg_2024::controller::DroneEvent;

#[derive(Resource)]
pub struct DroneListener {
    pub receiver: Receiver<DroneEvent>,
}

#[derive(Resource)]
pub struct LeafListener {
    pub receiver: Receiver<LeafEvent>,
}
