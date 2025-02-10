use crate::{
    components::Node,
    resources::{DroneListener, LeafListener},
};
use bevy::prelude::*;

use super::resources::{Bytes, DisplayedInfo, DroneData, LeavesData};
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

pub fn listen_drones_events(
    drone_listener: Res<DroneListener>,
    node_query: Query<&Node>,
    mut info: ResMut<DisplayedInfo>,
) {
    while let Ok(event) = drone_listener.receiver.try_recv() {
        match event {
            DroneEvent::PacketDropped(p) => {
                if p.routing_header.hop_index > 0
                    && p.routing_header.hop_index <= p.routing_header.hops.len()
                {
                    let hop = p.routing_header.hops[p.routing_header.hop_index - 1];
                    let entry = info.drone.entry(hop).or_insert(DroneData {
                        packets_sent: 0,
                        packets_shortcutted: 0,
                        data_sent: Bytes(0),
                        data_dropped: Bytes(0),
                        neighbours: HashMap::default(),
                    });

                    if let PacketType::MsgFragment(fragment) = p.pack_type {
                        entry.data_dropped += u64::from(fragment.length);
                    }
                } else {
                    eprintln!("Invalid routing header: {:?}", p.routing_header);
                }
            }
            DroneEvent::PacketSent(p) => {
                if let PacketType::FloodRequest(_) = p.pack_type {
                    continue;
                }
                if p.routing_header.hop_index > 0
                    && p.routing_header.hop_index <= p.routing_header.hops.len()
                {
                    let entry = info
                        .drone
                        .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                        .or_insert(DroneData {
                            packets_sent: 0,
                            packets_shortcutted: 0,
                            data_sent: Bytes(0),
                            data_dropped: Bytes(0),
                            neighbours: HashMap::default(),
                        });
                    entry.packets_sent += 1;
                    if let PacketType::MsgFragment(fragment) = p.pack_type {
                        entry.data_sent += u64::from(fragment.length);
                        entry
                            .neighbours
                            .entry(p.routing_header.hops[p.routing_header.hop_index])
                            .or_insert(Bytes(0))
                            .0 += u64::from(fragment.length);
                    }
                } else {
                    eprintln!("Invalid routing header: {:?}", p.routing_header);
                }
            }
            DroneEvent::ControllerShortcut(p) => {
                if p.routing_header.hop_index > 0
                    && p.routing_header.hop_index <= p.routing_header.hops.len()
                {
                    let entry = info
                        .drone
                        .entry(p.routing_header.hops[p.routing_header.hop_index - 1])
                        .or_insert(DroneData {
                            packets_sent: 0,
                            packets_shortcutted: 0,
                            data_sent: Bytes(0),
                            data_dropped: Bytes(0),
                            neighbours: HashMap::default(),
                        });
                    entry.packets_shortcutted += 1;
                    shortcut(&node_query, &p);
                } else {
                    eprintln!("Invalid routing header: {:?}", p.routing_header);
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
                if let PacketType::FloodRequest(_) = p.pack_type {
                    continue;
                }
                if p.routing_header.hop_index > 0
                    && p.routing_header.hop_index <= p.routing_header.hops.len()
                {
                    let hop = p.routing_header.hops[p.routing_header.hop_index - 1];
                    let entry = info.leaf.entry(hop).or_insert(LeavesData {
                        packets_sent: 0,
                        data_sent: Bytes(0),
                        msg_n: 0,
                        messages: HashMap::default(),
                    });

                    entry.packets_sent += 1;

                    if let PacketType::MsgFragment(fragment) = p.pack_type {
                        entry.data_sent += u64::from(fragment.length);
                    }
                } else {
                    eprintln!("Invalid routing header: {:?}", p.routing_header);
                }
            }
            LeafEvent::ControllerShortcut(p) => {
                shortcut(&node_query, &p);
            }
            LeafEvent::MessageStartSend {
                start,
                session,
                dest,
                message: m,
            } => {
                let entry = info.leaf.entry(start).or_insert(LeavesData {
                    packets_sent: 0,
                    data_sent: Bytes(0),
                    msg_n: 0,
                    messages: HashMap::default(),
                });
                entry.messages.insert(session, (m, dest, false));
            }
            LeafEvent::MessageFullySent(start, session) => {
                let entry = info.leaf.entry(start).or_insert(LeavesData {
                    packets_sent: 0,
                    data_sent: Bytes(0),
                    msg_n: 0,
                    messages: HashMap::default(),
                });
                if let Some((_, _, ended)) = entry.messages.get_mut(&session) {
                    if !*ended {
                        *ended = true;
                        entry.msg_n += 1;
                    }
                }
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
        eprintln!("### SHORTCUT: failed to shortcut");
    }
}
