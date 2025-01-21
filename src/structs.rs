use bevy::prelude::*;
use crossbeam_channel::Receiver;
use std::collections::HashMap;

use common_structs::leaf::LeafEvent;
use common_structs::network::{Network, NodeInfo, TypeInfo};
use wg_2024::{controller::DroneEvent, network::NodeId, packet::Packet};

#[allow(unused)]
#[derive(Component)]
pub struct RustySC {
    pub topology: HashMap<NodeId, NodeInfo>,
    pub drone_listener: Receiver<DroneEvent>,
    pub leaf_listener: Receiver<LeafEvent>,
}

#[allow(unused)]
impl RustySC {
    pub fn new(network: Network) -> Self {
        RustySC {
            topology: network.topology,
            drone_listener: network.simulation_channels.drone_event_listener,
            leaf_listener: network.simulation_channels.leaf_event_listener,
        }
    }
    pub fn handle_drone_events(&mut self, event: DroneEvent) {
        match event {
            DroneEvent::PacketDropped(p) => todo!(),
            DroneEvent::PacketSent(p) => todo!(),
            DroneEvent::ControllerShortcut(p) => self.forward_shortcut(p),
        }
    }
    pub fn forward_shortcut(&self, packet: Packet) {
        let node_id: NodeId = packet.routing_header.hops[packet.routing_header.hops.len() - 1];
        let node = self.topology.get(&node_id).unwrap();
    }
    
}
