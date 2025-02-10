use bevy::prelude::*;

use crate::components::{Drone, Leaf};
use bevy_trait_query::RegisterExt;
use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use wg_2024::{controller::DroneCommand, network::NodeId, packet::Packet};

#[bevy_trait_query::queryable]
pub trait CommandSender {
    fn add_sender(&mut self, nghb_id: NodeId, packet_channel: Sender<Packet>)
        -> Result<(), String>;
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String>;
}

impl CommandSender for Drone {
    fn add_sender(
        &mut self,
        nghb_id: NodeId,
        packet_channel: Sender<Packet>,
    ) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(DroneCommand::AddSender(nghb_id, packet_channel))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(DroneCommand::RemoveSender(nghb_id))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
}

impl CommandSender for Leaf {
    fn add_sender(
        &mut self,
        nghb_id: NodeId,
        packet_channel: Sender<Packet>,
    ) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(LeafCommand::AddSender(nghb_id, packet_channel))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
    fn remove_sender(&mut self, nghb_id: NodeId) -> Result<(), String> {
        if let Err(err) = self
            .command_channel
            .send(LeafCommand::RemoveSender(nghb_id))
        {
            return Err(err.to_string());
        }
        Ok(())
    }
}

pub struct SenderTraitPlugin;

impl Plugin for SenderTraitPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn CommandSender, Drone>()
            .register_component_as::<dyn CommandSender, Leaf>();
    }
}
