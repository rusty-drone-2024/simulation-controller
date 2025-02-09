use crate::ui::components::{LeafType, Node};
use crate::ui::resources::{DroneListener, LeafListener};
use bevy::prelude::*;

use bevy::utils::HashMap;
use common_structs::leaf::LeafEvent;
use wg_2024::packet::PacketType::{FloodRequest, MsgFragment};
use wg_2024::{controller::DroneEvent, network::NodeId, packet::Packet};

use super::components::Leaf;

type Packets = u64;
type Bytes = u64;

#[derive(Debug)]
pub struct DroneData {
    // Number of packets sent and shortcutted are disjoint
    pub packets_sent: Packets,
    pub packets_shortcutted: u64,
    // In bytes
    pub data_sent: Bytes,
    pub data_dropped: u64,
    // Number of wrong packets sent and shortcutted
    pub faulty_packets_sent: u64,
    // Number of unpermitted actions executed
    pub fouls: u64,
    // Value is the n of packets & data sent to each neighbour
    pub neighbours: HashMap<NodeId, (Packets, Bytes)>,
    // Average added delay expressed in ms
    pub latency: u64,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ClientData {
    pub packets_sent: Packets,
    // In bytes
    pub data_received: Bytes,
    // Number of pending and fullfilled requests
    pub pending_requests: u32,
    // Average number of bytes per message
    pub avg_bytes_xmessage: u64,
    // Number of unpermitted actions executed
    pub fouls: u64,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ServerData {
    pub packets_sent: Packets,
    // In bytes
    pub data_sent: Bytes,
    // Number of pending and fullfilled requests
    pub pending_requests: u32,
    pub fullfilled_requests: u64,
    // Average number of bytes per message
    pub avg_bytes_xmessage: u64,
    // Number of unpermitted actions executed
    pub fouls: u64,
}

#[derive(Debug, Resource)]
pub struct DisplayedInfo {
    pub drone: HashMap<NodeId, DroneData>,
    pub client: HashMap<NodeId, ClientData>,
    pub server: HashMap<NodeId, ServerData>,
}

pub struct EventListenerPlugin;

impl Plugin for EventListenerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayedInfo {
            drone: HashMap::default(),
            client: HashMap::default(),
            server: HashMap::default(),
        });
        app.add_systems(Update, listen_drones_events);
        app.add_systems(Update, listen_leaves_events);
    }
}

// TODO catch each packettype to log different messages depending on whats happening/wrong
fn listen_drones_events(
    drone_listener: Res<DroneListener>,
    node_query: Query<&Node>,
    mut info: ResMut<DisplayedInfo>,
) {
    while let Ok(event) = drone_listener.receiver.try_recv() {
        match event {
            DroneEvent::PacketDropped(p) => {
                let entry = info
                    .drone
                    .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                    .or_insert(DroneData {
                        packets_sent: 0,
                        packets_shortcutted: 0,
                        data_sent: 0,
                        data_dropped: 0,
                        faulty_packets_sent: 0,
                        fouls: 0,
                        neighbours: HashMap::default(),
                        latency: 0,
                    });
                if let MsgFragment(fragment) = p.pack_type {
                    entry.data_dropped += u64::from(fragment.length);
                } else {
                    entry.fouls += 1;
                }
            }
            DroneEvent::PacketSent(p) => {
                if let FloodRequest(_) = p.pack_type {
                    continue;
                }
                let entry = info
                    .drone
                    .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                    .or_insert(DroneData {
                        packets_sent: 0,
                        packets_shortcutted: 0,
                        data_sent: 0,
                        data_dropped: 0,
                        faulty_packets_sent: 0,
                        fouls: 0,
                        neighbours: HashMap::default(),
                        latency: 0,
                    });
                entry.packets_sent += 1;
                entry
                    .neighbours
                    .entry(p.routing_header.hops[p.routing_header.hop_index])
                    .or_insert((0, 0))
                    .0 += 1;

                // Check for error in routing
                if let Some(node) = node_query
                    .iter()
                    .find(|&node| node.id == p.routing_header.hops[p.routing_header.hop_index - 1])
                {
                    if !node
                        .neighbours
                        .contains(&p.routing_header.hops[p.routing_header.hop_index])
                    {
                        entry.faulty_packets_sent += 1;
                    }
                };
                // TODO: check for destination is drone
                ////
                if let MsgFragment(fragment) = p.pack_type {
                    entry.data_sent += u64::from(fragment.length);
                    entry
                        .neighbours
                        .entry(p.routing_header.hops[p.routing_header.hop_index])
                        .or_insert((0, 0))
                        .1 += u64::from(fragment.length);
                }
            }
            DroneEvent::ControllerShortcut(p) => {
                let entry = info
                    .drone
                    .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                    .or_insert(DroneData {
                        packets_sent: 0,
                        packets_shortcutted: 0,
                        data_sent: 0,
                        data_dropped: 0,
                        faulty_packets_sent: 0,
                        fouls: 0,
                        neighbours: HashMap::default(),
                        latency: 0,
                    });
                if let MsgFragment(_) | FloodRequest(_) = p.pack_type {
                    entry.fouls += 1;
                } else {
                    entry.packets_shortcutted += 1;
                    shortcut(&node_query, &p);
                }
            }
        }
    }
}

fn listen_leaves_events(
    leaf_listener: Res<LeafListener>,
    leaf_query: Query<(&Node, &Leaf)>,
    mut info: ResMut<DisplayedInfo>,
) {
    while let Ok(event) = leaf_listener.receiver.try_recv() {
        match event {
            LeafEvent::PacketSend(p) => {
                if let Some((node, leaf)) = leaf_query.iter().find(|(node, _)| {
                    node.id == p.routing_header.hops[p.routing_header.hop_index - 1]
                }) {
                    if leaf.leaf_type == LeafType::Client {
                        let entry = info.client.entry(node.id).or_insert(ClientData {
                            packets_sent: 0,
                            data_received: 0,
                            pending_requests: 0,
                            avg_bytes_xmessage: 0,
                            fouls: 0,
                        });
                    }
                }
            }
            LeafEvent::ControllerShortcut(p) => {
                //shortcut(&node_query, p);
            }
        }
    }
}

fn shortcut(node_query: &Query<&Node>, packet: &Packet) {
    let Some(dest) = &packet.routing_header.destination() else {
        return eprintln!("### SHORTCUT: NO DESTINATION");
    };

    let Some(node) = node_query.iter().find(|&node| *dest == node.id) else {
        return eprintln!("### SHORTCUT: DIDN'T FIND DESTINATION");
    };

    if node.packet_channel.send(packet.clone()).is_ok() {
        println!(
            "### SHORTCUT: Node with ID: {}, has received the packet: {}",
            node.id, packet
        );
    } else {
        println!("### SHORTCUT: failed to shortcut");
    }
}
