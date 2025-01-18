use bevy::prelude::*;
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashSet;
use wg_2024::{
    controller::{DroneCommand, DroneEvent},
    network::NodeId,
    packet::Packet,
};
#[allow(unused)]
#[derive(Component)]
pub struct Node {
    pub id: NodeId,
    pub neighbours: HashSet<NodeId>,
    pub packet_channel: Sender<Packet>,
}

#[allow(unused)]
#[derive(Component)]
pub struct Drone {
    pub pdr: f32,
    pub command_channel: Sender<DroneCommand>,
}

#[allow(unused)]
#[derive(Component)]
pub struct Leaf {
    pub command_channel: Sender<LeafCommand>,
}

#[allow(unused)]
#[derive(Component)]
pub enum LeafType {
    Client,
    Server,
}

#[derive(Bundle)]
pub struct LeafBundle {
    pub node: Node,
    pub leaf: Leaf,
    pub leaf_type: LeafType,
    pub model: SceneRoot,
}

#[derive(Bundle)]
pub struct DroneBundle {
    pub node: Node,
    pub drone: Drone,
    pub model: SceneRoot,
}

#[allow(unused)]
#[derive(Resource)]
pub struct Listeners {
    pub drone_listener: Receiver<DroneEvent>,
    pub leaf_listener: Receiver<LeafEvent>,
}
