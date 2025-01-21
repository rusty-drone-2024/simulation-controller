use super::super::components::{Drone, Leaf, Node};
use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use wg_2024::{controller::DroneCommand, network::NodeId, packet::Packet};

pub trait CommandChannel {
    fn send_remove(&mut self, nghb_id: NodeId) -> Result<(), String>;
    fn send_add(&mut self, nghb_id: NodeId, packet_channel: Sender<Packet>) -> Result<(), String>;
}

impl CommandChannel for Drone {
    fn send_remove(&mut self, nghb_id: NodeId) -> Result<(), String> {
        self.command_channel
            .send(DroneCommand::RemoveSender(nghb_id))
            .map_err(|err| err.to_string())
    }

    fn send_add(&mut self, nghb_id: NodeId, packet_channel: Sender<Packet>) -> Result<(), String> {
        self.command_channel
            .send(DroneCommand::AddSender(nghb_id, packet_channel))
            .map_err(|err| err.to_string())
    }
}

impl CommandChannel for Leaf {
    fn send_remove(&mut self, nghb_id: NodeId) -> Result<(), String> {
        self.command_channel
            .send(LeafCommand::RemoveSender(nghb_id))
            .map_err(|err| err.to_string())
    }

    fn send_add(&mut self, nghb_id: NodeId, packet_channel: Sender<Packet>) -> Result<(), String> {
        self.command_channel
            .send(LeafCommand::AddSender(nghb_id, packet_channel))
            .map_err(|err| err.to_string())
    }
}

pub trait SenderOperations {
    fn remove_sender(
        command_channel: &mut impl CommandChannel,
        node: &mut Node,
        nghb_id: NodeId,
    ) -> Result<(), String>;

    fn add_sender(
        command_channel: &mut impl CommandChannel,
        node: &mut Node,
        nghb_id: NodeId,
    ) -> Result<(), String>;
}

impl SenderOperations for () {
    fn remove_sender(
        command_channel: &mut impl CommandChannel,
        node: &mut Node,
        nghb_id: NodeId,
    ) -> Result<(), String> {
        let res = command_channel.send_remove(nghb_id);
        if res.is_ok() {
            node.neighbours.remove(&nghb_id);
        }
        res
    }

    fn add_sender(
        command_channel: &mut impl CommandChannel,
        node: &mut Node,
        nghb_id: NodeId,
    ) -> Result<(), String> {
        let res = command_channel.send_add(nghb_id, node.packet_channel.clone());
        if res.is_ok() {
            node.neighbours.insert(nghb_id);
        }
        res
    }
}
