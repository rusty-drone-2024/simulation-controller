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

    pub fn remove_sender(&mut self, nghb_id: NodeId, id: NodeId) -> Result<(), ()> {
        let node = self.topology.get(&id).unwrap();
        match node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info
                    .command_send_channel
                    .send(wg_2024::controller::DroneCommand::RemoveSender(id));
                self.topology
                    .get_mut(&id)
                    .unwrap()
                    .neighbours
                    .remove(&nghb_id);
                self.topology
                    .get_mut(&nghb_id)
                    .unwrap()
                    .neighbours
                    .remove(&id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn add_sender(&mut self, nghb_id: NodeId, id: NodeId) -> Result<(), ()> {
        let node = self.topology.get(&id).unwrap();
        let sender = self
            .topology
            .get(&nghb_id)
            .unwrap()
            .packet_in_channel
            .clone();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(
                    wg_2024::controller::DroneCommand::AddSender(nghb_id, sender),
                );
                self.topology
                    .get_mut(&id)
                    .unwrap()
                    .neighbours
                    .insert(nghb_id);
                self.topology
                    .get_mut(&nghb_id)
                    .unwrap()
                    .neighbours
                    .insert(id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn set_packet_drop_rate(&self, id: NodeId, pdr: f32) -> Result<(), ()> {
        let node = self.topology.get(&id).unwrap();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info
                    .command_send_channel
                    .send(wg_2024::controller::DroneCommand::SetPacketDropRate(pdr));
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn crash(&mut self, id: NodeId) -> Result<(), ()> {
        let node = self.topology.get(&id).unwrap();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info
                    .command_send_channel
                    .send(wg_2024::controller::DroneCommand::Crash);

                let neighbours = node.neighbours.clone();
                self.topology.remove(&id);
                for nghb_id in neighbours.iter() {
                    self.topology
                        .get_mut(nghb_id)
                        .unwrap()
                        .neighbours
                        .remove(&id);
                }

                Ok(())
            }
            _ => Err(()),
        }
    }
}
