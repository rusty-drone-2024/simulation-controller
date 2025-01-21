use bevy::prelude::*;
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashSet;
use wg_2024::{
    controller::{DroneCommand, DroneEvent},
    network::NodeId,
    packet::Packet,
};

#[derive(Bundle)]
pub struct LeafBundle {
    pub node: Node,
    pub leaf: Leaf,
    pub leaf_type: LeafType,
}

#[derive(Bundle)]
pub struct DroneBundle {
    pub node: Node,
    pub drone: Drone,
}

#[derive(Component)]
pub struct Node {
    pub id: NodeId,
    pub entity_id: Entity,
    pub neighbours: HashSet<NodeId>,
    pub packet_channel: Sender<Packet>,
}

#[derive(Component)]
pub struct Drone {
    pub command_channel: Sender<DroneCommand>,
    pub pdr: f32,
}

#[derive(Component)]
pub struct Leaf {
    pub command_channel: Sender<LeafCommand>,
}

#[derive(Component)]
pub enum LeafType {
    Client,
    Server,
}

#[derive(Component)]
pub struct Edge {
    pub start_node: NodeId,
    pub end_node: NodeId,
}

#[derive(Resource)]
pub struct Listeners {
    pub drone_listener: Receiver<DroneEvent>,
    pub leaf_listener: Receiver<LeafEvent>,
}
