use wg_2024::{controller::{self, DroneEvent}, network::NodeId, packet::Packet};
use common_structs::network::{Network, TypeInfo};
use network_initializer::{initialize_default_network,initialize_network_with_implementation};

use bevy::prelude::*;
mod ui;


fn main() {
    let network = initialize_default_network();
    let controller = RustySC { network };
    controller.run();
}

#[allow(dead_code)]
pub struct RustySC {
    network: Network,
}

impl RustySC {
    fn run(&self) {
        App::new()
            .add_plugins(DefaultPlugins)
            .run();
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
        let node = self.network.topology.get(&node_id).unwrap();
        
    }

    pub fn remove_sender(&mut self, nghb_id: NodeId, id: NodeId) -> Result<(), ()> {
        let node = self.network.topology.get(&id).unwrap();
        match node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info
                    .command_send_channel
                    .send(wg_2024::controller::DroneCommand::RemoveSender(id));
                self.network
                    .topology
                    .get_mut(&id)
                    .unwrap()
                    .neighbours
                    .remove(&nghb_id);
                self.network
                    .topology
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
        let node = self.network.topology.get(&id).unwrap();
        let sender = self.network.topology.get(&nghb_id).unwrap().packet_in_channel.clone();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(
                    wg_2024::controller::DroneCommand::AddSender(nghb_id, sender),
                );
                self.network
                    .topology
                    .get_mut(&id)
                    .unwrap()
                    .neighbours
                    .insert(nghb_id);
                self.network
                    .topology
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
        let node = self.network.topology.get(&id).unwrap();
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
        let node = self.network.topology.get(&id).unwrap();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info
                    .command_send_channel
                    .send(wg_2024::controller::DroneCommand::Crash);

                let neighbours = node.neighbours.clone();
                self.network.topology.remove(&id);
                for nghb_id in neighbours.iter() {
                    self.network
                        .topology
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
