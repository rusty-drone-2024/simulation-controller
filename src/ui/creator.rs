use bevy::prelude::*;
use crossbeam_channel::Sender;
use std::collections::HashMap;

use crate::ui::components::{
    Drone, DroneBundle, Leaf, LeafBundle, LeafType, Node, SelectedMarker, Text,
};
use crate::ui::on_click::{observer_drone, observer_leaf};
use network_initializer::network::{DroneInfo, LeafInfo, NodeInfo, TypeInfo};
use network_initializer::utils::single_creator::create_drone;
use wg_2024::{controller::DroneEvent, network::NodeId, packet::Packet};

//TODO add to nbgh channels
pub fn add_drone(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nodes: Query<&Node, Without<SelectedMarker>>,
    node_id: NodeId,
    pdr: f32,
    event_send: Sender<DroneEvent>,
    ngbs_packet_channels: HashMap<NodeId, Sender<Packet>>,
) {
    let node_info = create_drone(node_id, pdr, event_send, &ngbs_packet_channels);
    if let TypeInfo::Drone(drone_info) = &node_info.type_info {
        spawn_drone(
            &mut commands,
            &asset_server,
            node_id,
            &node_info,
            drone_info,
            Vec3::new(-200.0, 0.0, 0.0),
        );
    }
}

const TEXT_SCALE: Vec3 = Vec3::new(0.8, 0.8, 0.8);

pub fn spawn_drone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    node_id: NodeId,
    node_info: &NodeInfo,
    drone_info: &DroneInfo,
    translation: Vec3,
) {
    let entity_id = commands
        .spawn((
            DroneBundle {
                node: Node {
                    id: node_id,
                    neighbours: node_info.neighbours.clone(),
                    packet_channel: node_info.packet_in_channel.clone(),
                    entity_id: Entity::PLACEHOLDER,
                },
                drone: Drone {
                    pdr: drone_info.pdr,
                    command_channel: drone_info.command_send_channel.clone(),
                },
            },
            Sprite::from_image(asset_server.load("drone.png")),
            Transform {
                translation,
                scale: Vec3::new(0.3, 0.3, 0.3),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(entity_id).insert(Node {
        id: node_id,
        neighbours: node_info.neighbours.clone(),
        packet_channel: node_info.packet_in_channel.clone(),
        entity_id,
    });
    commands.entity(entity_id).observe(observer_drone);
    commands.spawn((
        Text { entity_id },
        Text2d(format!("Drone {}", node_id)),
        Transform {
            translation: Vec3::new(translation.x, translation.y + 15.0, 15.0),
            scale: TEXT_SCALE,
            ..Default::default()
        },
    ));
}

pub fn spawn_leaf(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    node_id: NodeId,
    node_info: &NodeInfo,
    leaf_info: &LeafInfo,
    translation: Vec3,
    client: bool,
) {
    let path: &str;
    let leaf_type: LeafType;
    let str: &str;
    if client {
        path = "client.png";
        leaf_type = LeafType::Client;
        str = "Client ";
    } else {
        path = "server.png";
        leaf_type = LeafType::Server;
        str = "Server ";
    }
    let entity_id = commands
        .spawn((
            LeafBundle {
                node: Node {
                    id: node_id,
                    neighbours: node_info.neighbours.clone(),
                    packet_channel: node_info.packet_in_channel.clone(),
                    entity_id: Entity::PLACEHOLDER,
                },
                leaf: Leaf {
                    command_channel: leaf_info.command_send_channel.clone(),
                    leaf_type,
                },
            },
            Sprite::from_image(asset_server.load(path)),
            Transform {
                translation,
                scale: Vec3::new(0.6, 0.6, 0.6),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(entity_id).insert(Node {
        id: node_id,
        neighbours: node_info.neighbours.clone(),
        packet_channel: node_info.packet_in_channel.clone(),
        entity_id,
    });
    commands.entity(entity_id).observe(observer_leaf);
    commands.spawn((
        Text { entity_id },
        Text2d(format!("{}{}", str, node_id)),
        Transform {
            translation: Vec3::new(translation.x, translation.y + 15.0, 15.0),
            scale: TEXT_SCALE,
            ..Default::default()
        },
    ));
}
