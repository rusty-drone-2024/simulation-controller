use super::components::{Drone, DroneBundle, Edge, Leaf, LeafBundle, LeafType, Node};
use bevy::{color, prelude::*};
use common_structs::network::TypeInfo;

use network_initializer::initialize_default_network;

use super::windows::{observer_drone, observer_leaf};
use rand::Rng;
use std::collections::HashSet;
use wg_2024::network::NodeId;

pub struct SpawnTopologyPlugin;

impl Plugin for SpawnTopologyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_sc)
            .add_systems(Update, update_edges);
    }
}

fn initialize_sc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let network = initialize_default_network("config.toml");
    let mut rng = rand::thread_rng();

    let scale_factor = Vec3::new(1.0, 1.0, 1.0);
    let mut connection_set: HashSet<(NodeId, NodeId)> = HashSet::new();

    for (node_id, node_info) in network.topology.iter() {
        match &node_info.type_info {
            TypeInfo::Drone(drone_info) => {
                let random_position = Vec3::new(
                    rng.gen_range(-200.0..200.0),
                    rng.gen_range(-150.0..150.0),
                    0.0,
                );
                let entity_id = commands
                    .spawn((
                        DroneBundle {
                            node: Node {
                                id: *node_id,
                                neighbours: node_info.neighbours.clone(),
                                packet_channel: node_info.packet_in_channel.clone(),
                                entity_id: Entity::PLACEHOLDER,
                            },
                            drone: Drone {
                                pdr: 1.0,
                                command_channel: drone_info.command_send_channel.clone(),
                            },
                        },
                        Sprite::from_image(asset_server.load("drone.png")),
                        Transform {
                            translation: random_position,
                            scale: scale_factor / 2.0,
                            ..Default::default()
                        },
                    ))
                    .id();
                commands.entity(entity_id).insert(Node {
                    id: *node_id,
                    neighbours: node_info.neighbours.clone(),
                    packet_channel: node_info.packet_in_channel.clone(),
                    entity_id: Entity::from(entity_id),
                });
                commands.entity(entity_id).observe(observer_drone);
            }
            TypeInfo::Client(leaf_info) => {
                let random_position = Vec3::new(
                    if rng.gen_bool(0.5) {
                        rng.gen_range(-600.0..-200.0)
                    } else {
                        rng.gen_range(200.0..600.0)
                    },
                    if rng.gen_bool(0.5) {
                        rng.gen_range(-400.0..-200.0)
                    } else {
                        rng.gen_range(200.0..400.0)
                    },
                    0.0,
                );
                let entity_id = commands
                    .spawn((
                        LeafBundle {
                            node: Node {
                                id: *node_id,
                                neighbours: node_info.neighbours.clone(),
                                packet_channel: node_info.packet_in_channel.clone(),
                                entity_id: Entity::PLACEHOLDER,
                            },
                            leaf: Leaf {
                                command_channel: leaf_info.command_send_channel.clone(),
                            },
                            leaf_type: LeafType::Client,
                        },
                        Sprite::from_image(asset_server.load("client.png")),
                        Transform {
                            translation: random_position,
                            scale: scale_factor,
                            ..Default::default()
                        },
                    ))
                    .id();
                commands.entity(entity_id).insert(Node {
                    id: *node_id,
                    neighbours: node_info.neighbours.clone(),
                    packet_channel: node_info.packet_in_channel.clone(),
                    entity_id: Entity::from(entity_id),
                });
                commands.entity(entity_id).observe(observer_leaf);
            }
            TypeInfo::Server(leaf_info) => {
                let random_position = Vec3::new(
                    if rng.gen_bool(0.5) {
                        rng.gen_range(-600.0..-200.0)
                    } else {
                        rng.gen_range(200.0..600.0)
                    },
                    if rng.gen_bool(0.5) {
                        rng.gen_range(-400.0..-200.0)
                    } else {
                        rng.gen_range(200.0..400.0)
                    },
                    0.0,
                );
                let entity_id = commands
                    .spawn((
                        LeafBundle {
                            node: Node {
                                id: *node_id,
                                neighbours: node_info.neighbours.clone(),
                                packet_channel: node_info.packet_in_channel.clone(),
                                entity_id: Entity::PLACEHOLDER,
                            },
                            leaf: Leaf {
                                command_channel: leaf_info.command_send_channel.clone(),
                            },
                            leaf_type: LeafType::Server,
                        },
                        Sprite::from_image(asset_server.load("server.png")),
                        Transform {
                            translation: random_position,
                            scale: scale_factor,
                            ..Default::default()
                        },
                    ))
                    .id();
                commands.entity(entity_id).insert(Node {
                    id: *node_id,
                    neighbours: node_info.neighbours.clone(),
                    packet_channel: node_info.packet_in_channel.clone(),
                    entity_id: Entity::from(entity_id),
                });
                commands.entity(entity_id).observe(observer_leaf);
            }
        }
        for neighbour_id in node_info.neighbours.iter() {
            if !connection_set.contains(&(*node_id, *neighbour_id)) {
                commands.spawn((
                    Edge {
                        start_node: *node_id,
                        end_node: *neighbour_id,
                    },
                    Transform::default(),
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                    MeshMaterial2d(materials.add(color::Color::srgb(100.0, 100.0, 100.0))),
                ));
                connection_set.insert((*node_id, *neighbour_id));
                connection_set.insert((*neighbour_id, *node_id));
            }
        }
    }
}
fn update_edges(
    mut edge_query: Query<(&Edge, &mut Transform)>,
    node_query: Query<(&Node, &Transform), Without<Edge>>,
) {
    for (edge, mut edge_transform) in edge_query.iter_mut() {
        let start_node_transform = node_query
            .iter()
            .find(|(node, _)| node.id == edge.start_node)
            .map(|(_, transform)| transform.translation);
        let end_node_transform = node_query
            .iter()
            .find(|(node, _)| node.id == edge.end_node)
            .map(|(_, transform)| transform.translation);

        if let (Some(start_position), Some(end_position)) =
            (start_node_transform, end_node_transform)
        {
            let midpoint = (start_position + end_position) / 2.0;
            let direction = end_position - start_position;
            let angle = direction.y.atan2(direction.x);
            let distance = direction.length() - 40.0;

            edge_transform.translation = midpoint;
            edge_transform.rotation = Quat::from_rotation_z(angle);
            edge_transform.scale = Vec3::new(distance, 1.0, 1.0);
        }
    }
}
