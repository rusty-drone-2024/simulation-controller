use crate::components::{Drone, DroneBundle, Edge, Leaf, LeafBundle, LeafType, Node, Text};
use crate::window::{observer_drone, observer_leaf};
use bevy::prelude::*;
use network_initializer::network::{DroneInfo, LeafInfo, NodeInfo};
use wg_2024::network::NodeId;

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
        Text2d(format!("Drone {node_id}")),
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
        Text2d(format!("{str}{node_id}")),
        Transform {
            translation: Vec3::new(translation.x, translation.y + 15.0, 15.0),
            scale: TEXT_SCALE,
            ..Default::default()
        },
    ));
}

pub fn spawn_edge(
    commands: &mut Commands,
    start_node: NodeId,
    end_node: NodeId,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Edge {
            start_node,
            end_node,
        },
        Transform::default(),
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(materials.add(Color::srgb(100.0, 100.0, 100.0))),
    ));
}
