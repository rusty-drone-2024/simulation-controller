use super::components::{Edge, Node, SelectionSpriteMarker};
use super::creator::{spawn_drone, spawn_leaf};
use crate::ui::resources::NetworkResource;
use crate::ui::resources::{DroneListener, LeafListener, Senders};
use crate::ui::settings::MusicResource;
use bevy::prelude::*;
use network_initializer::network::TypeInfo;
use rand::Rng;
use std::collections::HashSet;
use wg_2024::network::NodeId;

pub struct SpawnTopologyPlugin;

impl Plugin for SpawnTopologyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_soundtrack)
            .add_systems(PreStartup, initialize_sc)
            .add_systems(PreStartup, initialize_items)
            .add_systems(Update, update_edges);
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

fn spawn_soundtrack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut music: ResMut<MusicResource>,
) {
    let entity = commands
        .spawn((
            AudioPlayer::new(asset_server.load("soundtrack.mp3")),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(0.5),
                ..Default::default()
            },
        ))
        .id();
    music.entity = Some(entity);
}

fn update_edges(
    mut edge_query: Query<(&Edge, &mut Transform)>,
    node_query: Query<(&Node, &Transform), Without<Edge>>,
) {
    for (edge, mut edge_transform) in &mut edge_query {
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
