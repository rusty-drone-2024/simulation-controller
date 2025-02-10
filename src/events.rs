use bevy::prelude::*;
use wg_2024::network::NodeId;

#[derive(Event)]
pub struct AddDroneEvent {
    pub pdr: f32,
    pub ngbs: Vec<NodeId>,
}

#[derive(Event)]
pub struct AddEdgeEvent {
    pub start_node: NodeId,
    pub end_node: NodeId,
}

#[derive(Event)]
pub struct RmvEdgeEvent {
    pub start_node: NodeId,
    pub end_node: NodeId,
}
