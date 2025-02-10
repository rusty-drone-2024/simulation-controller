use bevy::prelude::*;

use common_structs::message::Message;
use std::collections::HashMap;
use wg_2024::network::NodeId;

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
pub struct LeavesData {
    pub packets_sent: Packets,
    // In bytes
    pub data_sent: Bytes,
    // Number of pending and fullfilled requests
    pub pending_requests: u32,
    // Average number of bytes per message
    pub avg_bytes_xmessage: u64,
    // Number of unpermitted actions executed
    pub fouls: u64,
    // Messages
    pub messages: Vec<Message>,
}

#[derive(Debug, Resource)]
pub struct DisplayedInfo {
    pub drone: HashMap<NodeId, DroneData>,
    pub leaf: HashMap<NodeId, LeavesData>,
}
