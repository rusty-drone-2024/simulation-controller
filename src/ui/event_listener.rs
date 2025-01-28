use crate::ui::resources::{DroneListener, LeafListener};
use crate::ui::components::Node;
use bevy::prelude::*;

use common_structs::leaf::LeafEvent;
use wg_2024::controller::DroneEvent;

pub struct EventListenerPlugin;

impl Plugin for EventListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_drones_events);
        app.add_systems(Update, listen_leaves_events);
    }
}

// TODO cach each packettype to log different messages depending on whats happening/wrong
fn listen_drones_events(drone_listener: Res<DroneListener>, node_query: Query<&Node>) {
    loop {
        match drone_listener.receiver.try_recv() {
            Ok(event) => match event {
                DroneEvent::PacketDropped(p) => {
                    println!("Drone {}, has dropped the packet: {:?}", p.routing_header.hops[p.routing_header.hop_index-1], p);
                },
                DroneEvent::PacketSent(p) => {
                    println!("Drone {}, has forwarded the packet: {:?}", p.routing_header.hops[p.routing_header.hop_index-1], p);
                },
                DroneEvent::ControllerShortcut(p) => {
                    for node in node_query.iter() {
                        if let Some(dest) = &p.routing_header.destination() {
                            if *dest == node.id {
                                let res= node.packet_channel.send(p.clone());
                                if res.is_ok(){
                                    println!("Node with ID: {}, has received the packet: {:?}",node.id , p);
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                error!("Got the following error while receiving drone events: {}", e);
                break;
            }
        }
    }
}

fn listen_leaves_events(leaf_listener: Res<LeafListener>, node_query: Query<&Node>) {
    loop {
        match leaf_listener.receiver.try_recv() {
            Ok(event) => match event {
                LeafEvent::PacketSend(p) => {
                    println!("Leaf {}, has sent the packet: {:?}", p.routing_header.hops[p.routing_header.hop_index-1], p);
                },
                LeafEvent::ControllerShortcut(p) => {
                    for node in node_query.iter() {
                        if let Some(dest) = &p.routing_header.destination() {
                            if *dest == node.id {
                                let res= node.packet_channel.send(p.clone());
                                if res.is_ok(){
                                    println!("Node with ID: {}, has received the packet: {:?}",node.id , p);
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                error!("Got the following error while receiving drone events: {}", e);
                break;
            }
        }
    }
}