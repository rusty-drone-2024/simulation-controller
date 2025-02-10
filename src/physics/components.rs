use bevy::prelude::*;

#[derive(Component)]
pub struct NodeForceGraphMarker {
    pub index: petgraph::stable_graph::NodeIndex<u8>,
}

#[derive(Component)]
pub struct EdgeForceGraphMarker {
    pub start_node: petgraph::stable_graph::NodeIndex<u8>,
    pub end_node: petgraph::stable_graph::NodeIndex<u8>,
}
