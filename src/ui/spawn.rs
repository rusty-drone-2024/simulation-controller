use super::components::{Drone, DroneBundle, Leaf, LeafBundle, LeafType, Node};
use bevy::prelude::*;
use common_structs::network::TypeInfo;
use network_initializer::initialize_default_network;
use rand::Rng;

pub struct SpawnTopology;

impl Plugin for SpawnTopology {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_sc);
    }
}

fn initialize_sc(mut commands: Commands, asset_server: Res<AssetServer>) {
    let network = initialize_default_network("config.toml");
    let mut rng = rand::thread_rng();

    let scale_factor = Vec3::splat(2.0);

    for (node_id, node_info) in network.topology.iter() {
        let random_position = Vec3::new(
            rng.gen_range(-80.0..80.0),
            rng.gen_range(-80.0..80.0),
            0.0,
        );

        match &node_info.type_info {
            TypeInfo::Drone(drone_info) => {
                let model_scene =
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("drone.glb"));

                commands.spawn((
                    DroneBundle {
                        node: Node {
                            id: *node_id,
                            neighbours: node_info.neighbours.clone(),
                            packet_channel: node_info.packet_in_channel.clone(),
                        },
                        drone: Drone {
                            pdr: 1.0,
                            command_channel: drone_info.command_send_channel.clone(),
                        },
                        model: SceneRoot(model_scene),
                    },
                    Transform{
                        translation: random_position,
                        scale: scale_factor,
                        rotation: Quat::from_euler(
                            bevy::math::EulerRot::XYZ,
                            0.0,
                            90.0,
                            0.0,
                        ),
                    },
                ));
            }
            TypeInfo::Client(leaf_info) => {
                let model_scene =
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("client.glb"));

                commands.spawn((
                    LeafBundle {
                        node: Node {
                            id: *node_id,
                            neighbours: node_info.neighbours.clone(),
                            packet_channel: node_info.packet_in_channel.clone(),
                        },
                        leaf: Leaf {
                            command_channel: leaf_info.command_send_channel.clone(),
                        },
                        leaf_type: LeafType::Client,
                        model: SceneRoot(model_scene),
                    },
                    Transform{
                        translation: random_position,
                        scale: scale_factor*8.0,
                        rotation: Quat::from_euler(
                            bevy::math::EulerRot::XYZ,
                            90.0,
                            90.0,
                            0.0,
                        ),
                    },
                ));
            }
            TypeInfo::Server(leaf_info) => {
                let model_scene =
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("server.glb"));

                commands.spawn((
                    LeafBundle {
                        node: Node {
                            id: *node_id,
                            neighbours: node_info.neighbours.clone(),
                            packet_channel: node_info.packet_in_channel.clone(),
                        },
                        leaf: Leaf {
                            command_channel: leaf_info.command_send_channel.clone(),
                        },
                        leaf_type: LeafType::Server,
                        model: SceneRoot(model_scene),
                    },
                    Transform{
                        translation: random_position,
                        scale: scale_factor*1.5,
                        rotation: Quat::from_euler(
                            bevy::math::EulerRot::XYZ,
                            0.0,
                            0.0,
                            0.0,
                        ),
                    },
                ));
            }
        }
    }
}