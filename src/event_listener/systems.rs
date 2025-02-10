use crate::{
    components::Node,
    resources::{DroneListener, LeafListener},
};
use bevy::prelude::*;

// TODO MAYBE HAVE SAME PARAMETERS AND STRUC TFOR BOTH LEAVES TOGHHETER THEN CHANGE DYSPLAY IN UI
use super::resources::{DisplayedInfo, DroneData, LeavesData};
use common_structs::leaf::LeafEvent;
use std::collections::HashMap;
use wg_2024::{
    controller::DroneEvent,
    packet::{Packet, PacketType},
};

pub fn initialize_info(mut commands: Commands) {
    commands.insert_resource(DisplayedInfo {
        drone: HashMap::default(),
        leaf: HashMap::default(),
    });
}

// TODO catch each packet type to log different messages depending on whats happening/wrong
pub fn listen_drones_events(
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
                if let PacketType::MsgFragment(fragment) = p.pack_type {
                    entry.data_dropped += u64::from(fragment.length);
                } else {
                    entry.fouls += 1;
                }
            }
            DroneEvent::PacketSent(p) => {
                if let PacketType::FloodRequest(_) = p.pack_type {
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
                if let PacketType::MsgFragment(fragment) = p.pack_type {
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
                if let PacketType::MsgFragment(_) | PacketType::FloodRequest(_) = p.pack_type {
                    entry.fouls += 1;
                } else {
                    entry.packets_shortcutted += 1;
                    shortcut(&node_query, &p);
                }
            }
        }
    }
}

pub fn listen_leaves_events(
    leaf_listener: Res<LeafListener>,
    node_query: Query<&Node>,
    mut info: ResMut<DisplayedInfo>,
) {
    while let Ok(event) = leaf_listener.receiver.try_recv() {
        match event {
            LeafEvent::PacketSend(p) => {
                let entry = info
                    .leaf
                    .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                    .or_insert(LeavesData {
                        packets_sent: 0,
                        data_sent: 0,
                        pending_requests: 0,
                        avg_bytes_xmessage: 0,
                        fouls: 0,
                        messages: Vec::new(),
                    });
                if let PacketType::MsgFragment(fragment) = p.pack_type {
                    entry.data_sent += u64::from(fragment.length);
                };
            }
            LeafEvent::ControllerShortcut(p) => {
                let entry = info
                    .leaf
                    .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                    .or_insert(LeavesData {
                        packets_sent: 0,
                        data_sent: 0,
                        pending_requests: 0,
                        avg_bytes_xmessage: 0,
                        fouls: 0,
                        messages: Vec::new(),
                    });
                if let PacketType::MsgFragment(_) | PacketType::FloodRequest(_) = p.pack_type {
                    entry.fouls += 1;
                } else {
                    shortcut(&node_query, &p);
                }
            }
            LeafEvent::MessageStartSend(id, _session, m) => {
                let entry = info.leaf.entry(id).or_insert(LeavesData {
                    packets_sent: 0,
                    data_sent: 0,
                    pending_requests: 0,
                    avg_bytes_xmessage: 0,
                    fouls: 0,
                    messages: Vec::new(),
                });
                entry.messages.push(m);
            }
            _ => {}
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
