use crate::ui::commands::utils::{is_connected, CommandSender};
use crate::ui::components::{
    AddDroneEvent, AddEdgeEvent, Edge, Leaf, LeafType, Node, RmvEdgeEvent,
};
use crate::ui::creator::spawn_drone;
use crate::ui::resources::Senders;
use bevy::prelude::*;
use bevy_trait_query::One;
use crossbeam_channel::Sender;
use network_initializer::network::TypeInfo;
use network_initializer::utils::single_creator::create_drone;
use std::collections::{HashMap, HashSet};
use wg_2024::{network::NodeId, packet::Packet};

use super::creator::spawn_edge;

pub struct AddersPlugins;

impl Plugin for AddersPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<AddDroneEvent>();
        app.add_event::<AddEdgeEvent>();
        app.add_event::<RmvEdgeEvent>();
        app.add_systems(Update, add_drone);
        app.add_systems(Update, add_edge);
        app.add_systems(Update, remove_edge);
    }
}

pub fn add_drone(
    mut er_add_drone: EventReader<AddDroneEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sender: Res<Senders>,
    mut nodes: Query<(&mut Node, Option<&Leaf>, One<&mut dyn CommandSender>)>,
) {
    for add_node in er_add_drone.read() {
        let mut node_info: HashMap<NodeId, Sender<Packet>> = HashMap::new();
        for (node, leaf, _sender) in nodes.iter() {
            for ngb_id in &add_node.ngbs {
                if node.id == *ngb_id {
                    if let Some(leaf) = leaf {
                        if leaf.leaf_type == LeafType::Client {
                            if node.neighbours.len() > 1 {
                                println!("Client should be connected to at most 2 drones");
                                return;
                            }
                        }
                    }
                    node_info.insert(node.id, node.packet_channel.clone());
                }
            }
        }
        if !(node_info.contains_key(&add_node.ngbs[0]) && node_info.contains_key(&add_node.ngbs[1]))
        {
            println!("Nodes not present");
            return;
        }
        let mut all_ids: Vec<NodeId> = nodes.iter().map(|(node, _, _)| node.id).collect();
        all_ids.sort();
        let mut node_id = 1;
        for id in all_ids {
            if id == node_id {
                node_id += 1;
            } else {
                break;
            }
        }

        let node_info = create_drone(
            node_id,
            add_node.pdr,
            sender.drone_sender.clone(),
            &node_info,
        );
        if let TypeInfo::Drone(drone_info) = &node_info.type_info {
            spawn_drone(
                &mut commands,
                &asset_server,
                node_id,
                &node_info,
                drone_info,
                Vec3::new(-200.0, 0.0, 0.0),
            );
        } else {
            println!("Wrong NI behaviour");
            return;
        }
        for (mut node, _leaf, mut sender) in nodes.iter_mut() {
            for ngb_id in &add_node.ngbs {
                if node.id == *ngb_id {
                    if sender
                        .add_sender(node_id, node_info.packet_in_channel.clone())
                        .is_ok()
                    {
                        node.neighbours.insert(node_id);
                        spawn_edge(&mut commands, node_id, *ngb_id, &mut meshes, &mut materials);
                    } else {
                        println!("Error adding sender");
                        return;
                    }
                }
            }
        }
        println!("Drone spawned successfully");
    }
}

pub fn add_edge(
    mut er_add_edge: EventReader<AddEdgeEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nodes: Query<(&mut Node, Option<&Leaf>, One<&mut dyn CommandSender>)>,
    edges: Query<&Edge>,
) {
    for edge in er_add_edge.read() {
        if edges.iter().any(|e| {
            (e.start_node == edge.start_node && e.end_node == edge.end_node)
                || (e.start_node == edge.end_node && e.end_node == edge.start_node)
        }) {
            println!("Edge already exists");
            return;
        }
        let mut node_info: HashMap<NodeId, Sender<Packet>> = HashMap::new();
        for (node, leaf, _sender) in nodes.iter() {
            if node.id == edge.start_node || node.id == edge.end_node {
                node_info.insert(node.id, node.packet_channel.clone());
                if let Some(leaf) = leaf {
                    if leaf.leaf_type == LeafType::Client {
                        if node.neighbours.len() > 1 {
                            println!("Client should be connected to at most 2 drones");
                            return;
                        }
                    }
                }
            }
        }
        if !(node_info.contains_key(&edge.start_node) && node_info.contains_key(&edge.end_node)) {
            println!("Can't connect nodes if either of them is not present");
            return;
        }
        let mut inserted = (false, false);

        for (mut node, _leaf, mut sender) in nodes.iter_mut() {
            if node.id == edge.start_node {
                if sender
                    .add_sender(
                        edge.end_node,
                        node_info.get(&edge.end_node).unwrap().clone(),
                    )
                    .is_ok()
                {
                    node.neighbours.insert(edge.end_node);
                    inserted.0 = true;
                } else {
                    println!("Error adding sender for node {}", edge.start_node);
                    return;
                }
            }
            if node.id == edge.end_node {
                if sender
                    .add_sender(
                        edge.start_node,
                        node_info.get(&edge.start_node).unwrap().clone(),
                    )
                    .is_ok()
                {
                    node.neighbours.insert(edge.start_node);
                    inserted.1 = true;
                } else {
                    println!("Error adding sender for node {}", edge.end_node);
                    return;
                }
            }
            if inserted.0 && inserted.1 {
                break;
            }
        }

        if inserted.0 && inserted.1 {
            spawn_edge(
                &mut commands,
                edge.start_node,
                edge.end_node,
                &mut meshes,
                &mut materials,
            );
        } else {
            println!("Error processing one or both nodes.");
        }
    }
}

pub fn remove_edge(
    mut commands: Commands,
    mut er_add_edge: EventReader<RmvEdgeEvent>,
    mut nodes: Query<(&mut Node, Option<&Leaf>, One<&mut dyn CommandSender>)>,
    edge_query: Query<(Entity, &Edge)>,
) {
    for rmv_edge in er_add_edge.read() {
        let mut topology: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
        for (node, leaf, _sender) in nodes.iter() {
            if let Some(leaf) = leaf {
                if leaf.leaf_type == LeafType::Server {
                    if node.neighbours.len() < 2 {
                        println!("Server should always have at least 2 connections");
                        return;
                    }
                }
            }
            topology.insert(node.id, node.neighbours.clone());
        }
        if !(topology.contains_key(&rmv_edge.start_node)
            && topology.contains_key(&rmv_edge.end_node))
        {
            println!("Can't remove edge if either of the nodes is not present");
            return;
        }
        if !is_connected(
            topology,
            None,
            Some((rmv_edge.start_node, rmv_edge.end_node)),
        ) {
            println!("Removing this edge will disconnect the network...aborting");
            return;
        }
        let mut removed = (false, false);
        for (mut node, _leaf, mut sender) in nodes.iter_mut() {
            if node.id == rmv_edge.start_node {
                if sender.remove_sender(rmv_edge.end_node).is_ok() {
                    node.neighbours.remove(&rmv_edge.end_node);
                    removed.0 = true;
                } else {
                    println!("Error removing sender for node {}", rmv_edge.start_node);
                    return;
                }
            }
            if node.id == rmv_edge.end_node {
                if sender.remove_sender(rmv_edge.start_node).is_ok() {
                    node.neighbours.remove(&rmv_edge.start_node);
                    removed.1 = true;
                } else {
                    println!("Error removing sender for node {}", rmv_edge.end_node);
                    return;
                }
            }
            if removed.0 && removed.1 {
                break;
            }
        }
        if removed.0 && removed.1 {
            for (entity, edge) in edge_query.iter() {
                if (edge.start_node == rmv_edge.start_node && edge.end_node == rmv_edge.end_node)
                    || (edge.start_node == rmv_edge.end_node
                        && edge.end_node == rmv_edge.start_node)
                {
                    commands.entity(entity).despawn();
                    println!("Edge removed successfully");
                    return;
                }
            }
        }
    }
}
