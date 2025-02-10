use super::sender_trait::CommandSender;
use crate::components::{
    CrashMarker, Drone, Edge, Leaf, LeafType, Node, SelectedMarker, SelectionSpriteMarker, Text,
};
use crate::core::utils::is_connected;
use crate::settings::ModeConfig;
use bevy::prelude::*;
use bevy_trait_query::One;
use std::collections::{HashMap, HashSet};
use wg_2024::{controller::DroneCommand, network::NodeId};

pub fn crash(
    mut commands: Commands,
    mut drone_to_crash_query: Query<
        (Entity, &Drone, &mut Node),
        (With<SelectedMarker>, With<CrashMarker>),
    >,
    mut nodes_query: Query<
        (&mut Node, Option<&Leaf>, One<&mut dyn CommandSender>),
        Without<SelectedMarker>,
    >,
    mut selected_sprite_query: Query<&mut Visibility, (With<SelectionSpriteMarker>, Without<Node>)>,
    edge_query: Query<(Entity, &Edge)>,
    text_query: Query<(Entity, &Text)>,
    mode: Res<ModeConfig>,
) {
    let Some((entity, drone_crashing, node_crashing)) = drone_to_crash_query.iter_mut().next()
    else {
        return;
    };
    let mut topology: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
    for (node, leaf, _sender) in nodes_query.iter() {
        if let Some(leaf) = leaf {
            if mode.bypass_cheks {
                topology.insert(node.id, node.neighbours.clone());
                continue;
            }
            for ngb_id in &node_crashing.neighbours {
                if node.id == *ngb_id
                    && leaf.leaf_type == LeafType::Server
                    && node.neighbours.len() <= 2
                {
                    println!("Aborting crash: Server should always have at least 2 connections");
                    commands.entity(entity).remove::<CrashMarker>();
                    return;
                }
            }
        } else {
            topology.insert(node.id, node.neighbours.clone());
        };
    }
    topology.insert(node_crashing.id, node_crashing.neighbours.clone());
    if !is_connected(topology, Some(node_crashing.id), None) {
        println!("Aborting crash: Crashing this drone will disconnect the network...aborting");
        commands.entity(entity).remove::<CrashMarker>();
        return;
    }
    let res = drone_crashing
        .command_channel
        .send(DroneCommand::Crash)
        .map_err(|err| err.to_string());
    if res.is_ok() {
        //Sending remove sender command to neighbours
        for (mut node, _leaf, mut sender) in &mut nodes_query {
            if node.neighbours.contains(&node_crashing.id) {
                let res = sender.remove_sender(node_crashing.id);
                if res.is_ok() {
                    node.neighbours.remove(&node_crashing.id);
                } else {
                    println!("Error removing sender from node");
                }
            }
        }
        //Despawning the drone related entities
        commands.entity(entity).despawn();
        for mut visibility in &mut selected_sprite_query {
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
    println!("Crashed successfully");
}
