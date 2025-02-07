use crate::ui::commands::utils::CommandSender;
use crate::ui::components::{
    AddDroneEvent, AddEdgeEvent, Edge, Leaf, Node, RmvEdgeEvent, SelectedMarker,
};
use crate::ui::creator::spawn_drone;
use crate::ui::resources::Senders;
use bevy::prelude::*;
use bevy_trait_query::One;
use crossbeam_channel::Sender;
use network_initializer::network::TypeInfo;
use network_initializer::utils::single_creator::create_drone;
use std::collections::HashMap;
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
    mut nodes: Query<(&mut Node, One<&mut dyn CommandSender>), Without<SelectedMarker>>,
) {
    for add_node in er_add_drone.read() {
        let mut ngbs_packet_channels: HashMap<NodeId, Sender<Packet>> = HashMap::new();
        let mut all_ids: Vec<NodeId> = Vec::new();
        for (node, _sender) in nodes.iter() {
            all_ids.push(node.id.clone());
            for ngb_id in &add_node.ngbs {
                if node.id == *ngb_id {
                    // TODO will change to 1
                    if node.neighbours.len() >= 4 {
                        println!("Node {} has too many neighbours", node.id);
                        return;
                    }
                    ngbs_packet_channels.insert(node.id, node.packet_channel.clone());
                }
            }
        }
        if !(all_ids.contains(&add_node.ngbs[0]) && all_ids.contains(&add_node.ngbs[1])) {
            println!("Nodes not present");
            return;
        }
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
            &ngbs_packet_channels,
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
        for (mut node, mut sender) in nodes.iter_mut() {
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
    }
}

pub fn add_edge(
    mut er_add_edge: EventReader<AddEdgeEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nodes: Query<(&mut Node, Option<&Leaf>, One<&mut dyn CommandSender>)>,
) {
    for edge in er_add_edge.read() {
        let mut all_ids: Vec<NodeId> = Vec::new();
        let mut start_is_leaf = false;
        let mut end_is_leaf = false;
        for (node, leaf, _sender) in nodes.iter() {
            //TODO change this to 1
            if node.id == edge.start_node || node.id == edge.end_node {
                if node.neighbours.len() >= 4 {
                    println!("Node {} has too many neighbours", node.id);
                    return;
                }
                if node.id == edge.start_node {
                    start_is_leaf = leaf.is_some();
                }
                if node.id == edge.end_node {
                    end_is_leaf = leaf.is_some();
                }
            }
            if start_is_leaf && end_is_leaf {
                println!("Can't connect two leaves together");
                return;
            }
            all_ids.push(node.id.clone());
        }
        if !(all_ids.contains(&edge.start_node) && all_ids.contains(&edge.end_node)) {
            println!("Can't connect nodes if either of them is not present");
            return;
        }
        let mut inserted = (false, false);
        for (mut node, _leaf, mut sender) in nodes.iter_mut() {
            if node.id == edge.start_node {
                if sender
                    .add_sender(edge.end_node, node.packet_channel.clone())
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
                    .add_sender(edge.start_node, node.packet_channel.clone())
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
    mut nodes: Query<(&mut Node, One<&mut dyn CommandSender>)>,
    edge_query: Query<(Entity, &Edge)>,
) {
    //TODO CHECK if graph is still connected
    for rmv_edge in er_add_edge.read() {
        println!(
            "Removing edge between {} and {}",
            rmv_edge.start_node, rmv_edge.end_node
        );
        let mut all_ids: Vec<NodeId> = Vec::new();
        for (node, _sender) in nodes.iter() {
            all_ids.push(node.id.clone());
        }
        if !(all_ids.contains(&rmv_edge.start_node) && all_ids.contains(&rmv_edge.end_node)) {
            println!("Can't remove edge if either of the nodes is not present");
            return;
        }
        let mut removed = (false, false);
        for (mut node, mut sender) in nodes.iter_mut() {
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
                    return;
                }
            }
        } else {
            println!("Error processing one or both nodes.");
        }
    }
}
