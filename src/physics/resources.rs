use bevy::prelude::*;
use force_graph::{ForceGraph, NodeData, SimulationParameters};

#[derive(Resource)]
pub struct MyForceGraph {
    pub data: ForceGraph<NodeData>,
}

impl MyForceGraph {
    pub fn new() -> Self {
        Self {
            data: ForceGraph::new(SimulationParameters {
                force_charge: 4000.0,
                force_spring: 0.1,
                force_max: 140.0,
                node_speed: 4000.0,
                damping_factor: 0.98,
            }),
        }
    }
}
