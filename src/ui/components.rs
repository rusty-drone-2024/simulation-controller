use bevy::prelude::*;
use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::HashSet;
use std::fmt::Display;
use wg_2024::{controller::DroneCommand, network::NodeId, packet::Packet};

#[derive(Bundle)]
pub struct LeafBundle {
    pub node: Node,
    pub leaf: Leaf,
}

#[derive(Bundle)]
pub struct DroneBundle {
    pub node: Node,
    pub drone: Drone,
}

#[derive(Clone, Component)]
pub struct Node {
    pub id: NodeId,
    pub entity_id: Entity,
    pub neighbours: HashSet<NodeId>,
    pub packet_channel: Sender<Packet>,
}

#[derive(Component)]
pub struct SelectedMarker;

#[derive(Component)]
pub struct SelectionSpriteMarker;

#[derive(Clone, Component)]
pub struct Drone {
    pub command_channel: Sender<DroneCommand>,
    pub pdr: f32,
}

#[derive(Clone, Component)]
pub struct Leaf {
    pub command_channel: Sender<LeafCommand>,
    pub leaf_type: LeafType,
}

#[derive(Debug, Clone, Component)]
pub enum LeafType {
    Client,
    Server,
}
impl Display for LeafType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeafType::Client => write!(f, "Client"),
            LeafType::Server => write!(f, "Server"),
        }
    }
}

#[derive(Component)]
pub struct Edge {
    pub start_node: NodeId,
    pub end_node: NodeId,
}
