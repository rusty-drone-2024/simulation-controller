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
    mut nodes: HashMap<u8, HashSet<u8>>,
    removed_node: Option<u8>,
    removed_edge: Option<(u8, u8)>,
) -> bool {
    // Remove the node and its connections if specified
    if let Some(removed_id) = removed_node {
        nodes.remove(&removed_id);
        for neighbors in nodes.values_mut() {
            neighbors.remove(&removed_id);
        }
    }

    // Remove the specified edge if provided
    if let Some((removed_id1, removed_id2)) = removed_edge {
        if let Some(neighbors) = nodes.get_mut(&removed_id1) {
            neighbors.remove(&removed_id2);
        }
        if let Some(neighbors) = nodes.get_mut(&removed_id2) {
            neighbors.remove(&removed_id1);
        }
    }

    // Handle empty graph case
    if nodes.is_empty() {
        return true;
    }

    let mut visited = HashSet::new();

    // Pick any starting node
    let start_node = *nodes.keys().next().unwrap();
    let mut stack = vec![start_node];

    // DFS loop
    while let Some(node_id) = stack.pop() {
        if visited.insert(node_id) {
            // Visit all unvisited neighbors
            if let Some(neighbors) = nodes.get(&node_id) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    println!("{:?}", visited);
    println!("{:?}", nodes);
    // Check if all nodes were visited
    visited.len() == nodes.len()
}
