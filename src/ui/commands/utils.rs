use bevy::prelude::*;

use crate::ui::components::{Drone, Leaf};
use bevy_trait_query::RegisterExt;
use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::{HashMap, HashSet};
use wg_2024::{controller::DroneCommand, network::NodeId, packet::Packet};

#[bevy_trait_query::queryable]
pub trait CommandSender {
    fn add_sender(&mut self, nghb_id: NodeId, packet_channel: Sender<Packet>)
        -> Result<(), String>;
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String>;
}

impl CommandSender for Drone {
    fn add_sender(
        &mut self,
        nghb_id: NodeId,
        packet_channel: Sender<Packet>,
    ) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(DroneCommand::AddSender(nghb_id, packet_channel))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(DroneCommand::RemoveSender(nghb_id))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
}

impl CommandSender for Leaf {
    fn add_sender(
        &mut self,
        nghb_id: NodeId,
        packet_channel: Sender<Packet>,
    ) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(LeafCommand::AddSender(nghb_id, packet_channel))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(LeafCommand::RemoveSender(nghb_id))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
}

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn CommandSender, Drone>()
            .register_component_as::<dyn CommandSender, Leaf>();
    }
}

pub fn is_connected(
    nodes_to_add: HashMap<u8, (HashSet<u8>, bool)>,
    removed_node: Option<u8>,
    removed_edge: Option<(u8, u8)>,
) -> bool {
    let mut nodes: HashMap<u8, HashSet<u8>> = HashMap::new();
    for (node_id, (neighbours, is_drone)) in nodes_to_add {
        if is_drone {
            nodes.insert(node_id, neighbours);
        }
    }

    if let Some(removed_id) = removed_node {
        nodes.remove(&removed_id);
        for neighbours in nodes.values_mut() {
            neighbours.remove(&removed_id);
        }
    }
    if let Some((removed_id1, removed_id2)) = removed_edge {
        nodes.get_mut(&removed_id1).unwrap().remove(&removed_id2);
        nodes.get_mut(&removed_id2).unwrap().remove(&removed_id1);
    }
    let mut visited = HashSet::new();
    let mut stack = vec![*nodes.keys().next().unwrap()];
    while let Some(node_id) = stack.pop() {
        visited.insert(node_id);
        for neighbour in nodes.get(&node_id).unwrap() {
            if !visited.contains(neighbour) {
                stack.push(*neighbour);
            }
        }
    }
    visited.len() == nodes.len()
}
