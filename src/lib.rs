
use common_structs::network::{Network, TypeInfo};
use wg_2024::{
    controller::DroneCommand,
    network::NodeId,
};

#[allow(dead_code)]
pub struct RustySC {
    network: Network,
}

impl RustySC {
    pub fn start(network: Network) {
        let controller = RustySC { network };
        controller.run();
    }
}

impl RustySC {
    fn run(&self) {
        println!("Running simulation controller");
    }

    pub fn remove_sender (&mut self, nghb_id: NodeId, id: NodeId) -> Result<(), ()> {
        let node = self.network.topology.get(&id).unwrap();
        match node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(DroneCommand::RemoveSender(nghb_id));
                self.network.topology.get_mut(&id).unwrap().neighbours.remove(&nghb_id);
                self.network.topology.get_mut(&nghb_id).unwrap().neighbours.remove(&id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn add_sender (&mut self, nghb_id: NodeId, id: NodeId) -> Result<(), ()> {
        let node = self.network.topology.get(&id).unwrap();
        let sender = self.network.simulation_channels.leaf_event_sender.clone();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(DroneCommand::AddSender(nghb_id, sender));
                self.network.topology.get_mut(&id).unwrap().neighbours.insert(nghb_id);
                self.network.topology.get_mut(&nghb_id).unwrap().neighbours.insert(id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn set_packet_drop_rate (&self, id: NodeId, pdr: f32) -> Result<(), ()> {
        let node = self.network.topology.get(&id).unwrap();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(DroneCommand::SetPacketDropRate(pdr));
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn crash (&mut self, id: NodeId) -> Result<(), ()> {
        let node = self.network.topology.get(&id).unwrap();
        match &node.type_info {
            TypeInfo::Drone(ref drone_info) => {
                let _ = drone_info.command_send_channel.send(DroneCommand::Crash);
                
                let neighbours = node.neighbours.clone();
                self.network.topology.remove(&id);
                for nghb_id in neighbours.iter() {
                    self.network.topology.get_mut(nghb_id).unwrap().neighbours.remove(&id);
                }
                
                Ok(())
            }
            _ => Err(()),
        }
    }
}
