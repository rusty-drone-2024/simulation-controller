use std::collections::{HashMap, HashSet};

use crate::ui::components::{
    CrashMarker, Drone, Edge, Leaf, Node, SelectedMarker, SelectionSpriteMarker, Text,
};

use bevy::prelude::*;

use common_structs::leaf::LeafCommand;
use wg_2024::controller::DroneCommand;

impl Drone {
    pub fn set_packet_drop_rate(&mut self, pdr: f32) -> Result<(), String> {
        let res = self
            .command_channel
            .send(DroneCommand::SetPacketDropRate(pdr))
            .map_err(|err| err.to_string());
        if res.is_ok() {
            self.pdr = pdr;
        };
        res
    }
}

pub fn crash(
    mut commands: Commands,
    mut drone_to_crash_query: Query<
        (Entity, &Drone, &mut Node),
        (With<SelectedMarker>, With<CrashMarker>),
    >,
    mut drone_query: Query<(&Drone, &mut Node), (Without<SelectedMarker>, Without<Leaf>)>,
    mut leaf_query: Query<(&Leaf, &mut Node), Without<Drone>>,
    mut selected_sprite_query: Query<
        &mut Visibility,
        (With<SelectionSpriteMarker>, Without<Drone>, Without<Leaf>),
    >,
    edge_query: Query<(Entity, &Edge)>,
    text_query: Query<(Entity, &Text)>,
) {
    let (entity, drone_crashing, node_crashing) = match drone_to_crash_query.iter_mut().next() {
        Some((entity, drone, node)) => (entity, drone, node),
        None => return,
    };
    if !is_still_connected(&drone_query, &leaf_query, node_crashing.id) {
        return;
    }
    let res = drone_crashing
        .command_channel
        .send(DroneCommand::Crash)
        .map_err(|err| err.to_string());
    if res.is_ok() {
        for (drone, mut node) in drone_query.iter_mut() {
            if node.neighbours.contains(&node_crashing.id) {
                let res = drone
                    .command_channel
                    .send(DroneCommand::RemoveSender(node_crashing.id))
                    .map_err(|err| err.to_string());
                if res.is_ok() {
                    node.neighbours.remove(&node_crashing.id);
                }
            }
        }
        for (leaf, mut node) in leaf_query.iter_mut() {
            if node.neighbours.contains(&node_crashing.id) {
                let res = leaf
                    .command_channel
                    .send(LeafCommand::RemoveSender(node_crashing.id))
                    .map_err(|err| err.to_string());
                if res.is_ok() {
                    node.neighbours.remove(&node_crashing.id);
                }
            }
        }
        commands.entity(entity).despawn();
        for mut visibility in selected_sprite_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for (entity, edge) in edge_query.iter() {
            if edge.start_node == node_crashing.id || edge.end_node == node_crashing.id {
                commands.entity(entity).despawn();
            }
        }
        for (text_entity, text) in text_query.iter() {
            if text.entity_id == entity {
                commands.entity(text_entity).despawn();
            }
        }
    }
}

fn is_still_connected(
    drone_query: &Query<(&Drone, &mut Node), (Without<SelectedMarker>, Without<Leaf>)>,
    leaf_query: &Query<(&Leaf, &mut Node), Without<Drone>>,
    removed_id: u8,
) -> bool {
    let mut nodes = HashMap::new();
    for (_drone, node) in drone_query.iter() {
        nodes.insert(node.id, node.neighbours.clone());
    }
    for (_leaf, node) in leaf_query.iter() {
        nodes.insert(node.id, node.neighbours.clone());
    }
    let mut visited = HashSet::new();
    let mut stack = vec![*nodes.keys().next().unwrap()];
    while let Some(node_id) = stack.pop() {
        visited.insert(node_id);
        for neighbour in nodes.get(&node_id).unwrap() {
            if neighbour != &removed_id && !visited.contains(neighbour) {
                stack.push(*neighbour);
            }
        }
    }
    visited.len() == nodes.len()
}
