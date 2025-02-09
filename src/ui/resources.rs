use bevy::prelude::*;

use common_structs::leaf::LeafEvent;
use crossbeam_channel::{Receiver, Sender};
use wg_2024::controller::DroneEvent;

#[derive(Resource)]
pub struct DroneListener {
    pub receiver: Receiver<DroneEvent>,
}

#[derive(Resource)]
pub struct LeafListener {
    pub receiver: Receiver<LeafEvent>,
}

#[derive(Resource)]
pub struct Senders {
    pub drone_sender: Sender<DroneEvent>,
    pub _leaf_sender: Sender<LeafEvent>,
}

#[derive(Resource)]
pub struct ModeConfig {
    pub bypass_cheks: bool,
}
