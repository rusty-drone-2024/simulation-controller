use crate::ui::components::Node;
use crate::ui::resources::{DroneListener, LeafListener};
use bevy::prelude::*;

use common_structs::leaf::LeafEvent;
use wg_2024::controller::DroneEvent;
use wg_2024::packet::Packet;

pub struct EventListenerPlugin;

impl Plugin for EventListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_drones_events);
        app.add_systems(Update, listen_leaves_events);
    }
}

// TODO catch each packettype to log different messages depending on whats happening/wrong
fn listen_drones_events(drone_listener: Res<DroneListener>, node_query: Query<&Node>) {
    while let Ok(event) = drone_listener.receiver.try_recv() {
        match event {
            DroneEvent::PacketDropped(_p) => {
                //println!("Drone {}, has dropped the packet: {}", _p.routing_header.hops[p.routing_header.hop_index-1], _p);
            }
            DroneEvent::PacketSent(_p) => {
                //println!("Drone {}, has forwarded the packet: {}", _p.routing_header.hops[p.routing_header.hop_index-1], _p);
            }
            DroneEvent::ControllerShortcut(p) => {
                shortcut(&node_query, p);
            }
        }
    }
}

fn listen_leaves_events(leaf_listener: Res<LeafListener>, node_query: Query<&Node>) {
    while let Ok(event) = leaf_listener.receiver.try_recv() {
        match event {
            LeafEvent::PacketSend(_p) => {
                //println!("Leaf {}, has sent the packet: {}", p.routing_header.hops[p.routing_header.hop_index-1], _p);
            }
            LeafEvent::ControllerShortcut(p) => {
                shortcut(&node_query, p);
            }
        }
    }
}

fn shortcut(node_query: &Query<&Node>, packet: Packet) {
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
        println!("### SHORTCUT: failed to shortcut");
    }
}
