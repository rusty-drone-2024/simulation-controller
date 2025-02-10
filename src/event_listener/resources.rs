use bevy::prelude::*;
use std::fmt;
use std::ops::{AddAssign, Div};

use common_structs::message::Message;
use common_structs::types::Session;
use std::collections::HashMap;
use wg_2024::network::NodeId;

type Packets = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytes(pub u64);

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;

        if value < 1024 {
            write!(f, "{value} bytes")
        } else if value < 1024 * 1024 {
            write!(f, "{:.2} KB", value as f64 / 1024.0)
        } else if value < 1024 * 1024 * 1024 {
            write!(f, "{:.2} MB", value as f64 / (1024.0 * 1024.0))
        } else {
            write!(f, "{:.2} GB", value as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

impl Div<u64> for Bytes {
    type Output = Bytes;

    fn div(self, rhs: u64) -> Bytes {
        Bytes(self.0 / rhs)
    }
}

impl AddAssign<u64> for Bytes {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}

#[derive(Debug, Clone)]
pub struct DroneData {
    // Number of packets sent and shortcutted are disjoint
    pub packets_sent: Packets,
    pub packets_shortcutted: u64,
    // In bytes
    pub data_sent: Bytes,
    pub data_dropped: Bytes,
    // Value is the n of packets & data sent to each neighbour
    pub neighbours: HashMap<NodeId, Bytes>,
}

impl DroneData {
    pub fn neighbour_usage_percentages(&self) -> Vec<(NodeId, u32)> {
        let total_bytes_sent = self.data_sent.clone();

        if total_bytes_sent == Bytes(0) {
            return self.neighbours.keys().map(|&id| (id, 0)).collect();
        }

        let mut percentages: Vec<(NodeId, u32)> = self
            .neighbours
            .iter()
            .map(|(&node_id, bytes)| {
                let percentage = ((bytes.0 as f64 / total_bytes_sent.0 as f64) * 100.0)
                    .clamp(1.0, 100.0)
                    .round() as u32;
                (node_id, percentage)
            })
            .collect();

        percentages.sort_by(|a, b| b.1.cmp(&a.1));
        percentages
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct LeavesData {
    pub packets_sent: Packets,
    // In bytes
    pub data_sent: Bytes,
    // Number of requests / responses
    pub msg_n: u64,
    // Messages
    pub messages: HashMap<Session, (Message, NodeId, bool)>,
}

#[derive(Debug, Resource)]
pub struct DisplayedInfo {
    pub drone: HashMap<NodeId, DroneData>,
    pub leaf: HashMap<NodeId, LeavesData>,
}
