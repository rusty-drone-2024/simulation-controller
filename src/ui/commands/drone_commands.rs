use super::super::components::{Drone, Leaf, Node};

use bevy::prelude::*;

#[allow(unused)]
use wg_2024::{
    controller::{DroneCommand, DroneEvent},
    network::NodeId,
    packet::Packet,
};

impl Drone {
    pub fn set_packet_drop_rate(& mut self, pdr: f32) -> Result<(), String> {
        let res = self.command_channel
            .send(wg_2024::controller::DroneCommand::SetPacketDropRate(pdr))
            .map_err(|err| err.to_string());
        if res.is_ok(){
            self.pdr = pdr;
        };
        res
    }
}

// TODO FIGURE OUT HOW TO PASS THE DRONE IM USING
// TO DO FIGURE OUT HOW TO KILL DRONES IN UI
fn crash(
    trigger: Trigger<Pointer<Click>>,
    mut drone_query: Query<(&Drone, &mut Node)>,
    leaf_query: Query<(&Leaf, &mut Node), Without<Drone>>,
) {
    let entity = trigger.entity();

    for (drone, mut drone_node) in drone_query.iter_mut() {
        if entity == drone_node.entity_id {
            //TODO
        }
    }
}
