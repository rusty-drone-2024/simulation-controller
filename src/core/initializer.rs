use super::creator::{spawn_drone, spawn_leaf};
use crate::components::{Edge, SelectionSpriteMarker};
use crate::resources::{DroneListener, LeafListener, NetworkResource, Senders};
use bevy::prelude::*;
use network_initializer::network::TypeInfo;
use rand::Rng;
use std::collections::HashSet;
use wg_2024::network::NodeId;

pub struct SpawnTopologyPlugin;

impl Plugin for SpawnTopologyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_sc)
            .add_systems(PreStartup, initialize_items);
    }
}

fn initialize_sc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    network: ResMut<NetworkResource>,
) {
    commands.insert_resource(DroneListener {
        receiver: network
            .data
            .simulation_channels
            .drone_event_listener
            .clone(),
    });
    commands.insert_resource(LeafListener {
        receiver: network.data.simulation_channels.leaf_event_listener.clone(),
    });
    commands.insert_resource(Senders {
        drone_sender: network.data.simulation_channels.drone_event_sender.clone(),
        _leaf_sender: network.data.simulation_channels.leaf_event_sender.clone(),
    });

    let mut rng = rand::rng();
    let mut connection_set: HashSet<(NodeId, NodeId)> = HashSet::new();

    for (node_id, node_info) in &network.data.topology {
        let random_position = Vec3::new(
            rng.random_range(-200.0..100.0),
            rng.random_range(-150.0..150.0),
            0.0,
        );
        match &node_info.type_info {
            TypeInfo::Drone(drone_info) => {
                spawn_drone(
                    &mut commands,
                    &asset_server,
                    *node_id,
                    node_info,
                    drone_info,
                    random_position,
                );
            }
            TypeInfo::Client(leaf_info) => {
                spawn_leaf(
                    &mut commands,
                    &asset_server,
                    *node_id,
                    node_info,
                    leaf_info,
                    random_position,
                    true,
                );
            }
            TypeInfo::Server(leaf_info) => {
                spawn_leaf(
                    &mut commands,
                    &asset_server,
                    *node_id,
                    node_info,
                    leaf_info,
                    random_position,
                    false,
                );
            }
        }
        for neighbour_id in &node_info.neighbours {
            if !connection_set.contains(&(*node_id, *neighbour_id)) {
                commands.spawn((
                    Edge {
                        start_node: *node_id,
                        end_node: *neighbour_id,
                    },
                    Transform::default(),
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                    MeshMaterial2d(materials.add(Color::srgb(100.0, 100.0, 100.0))),
                ));
                connection_set.insert((*node_id, *neighbour_id));
                connection_set.insert((*neighbour_id, *node_id));
            }
        }
    }
}

fn initialize_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("selected.png"),
            color: Color::srgb(1.0, 0.8, 0.8),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Visibility::Hidden,
        SelectionSpriteMarker,
    ));
}
