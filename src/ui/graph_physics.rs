use bevy::prelude::*;

use super::components::Node;
use force_graph::{ForceGraph, NodeData};
use petgraph::prelude::NodeIndex;
use std::collections::HashMap;
use wg_2024::network::NodeId;

#[allow(unused)]
#[derive(Resource)]
pub struct Graph {
    pub correlation: HashMap<NodeId, NodeIndex>,
    pub force_graph: ForceGraph<NodeData>,
}
#[allow(unused)]
impl Graph {
    pub fn new() -> Self {
        Graph {
            correlation: HashMap::new(),
            force_graph: ForceGraph::new(Default::default()),
        }
    }
}
#[allow(unused)]
fn initialize_graph(mut commands: Commands) {
    commands.insert_resource(Graph::new());
}

#[allow(unused)]
fn add_node_to_graph(mut graph: ResMut<Graph>, node_id: NodeId, x: f32, y: f32) {
    if !graph.correlation.contains_key(&node_id) {
        let node_index = graph.force_graph.add_node(NodeData {
            x: x,
            y: y,
            ..Default::default()
        });
        graph.correlation.insert(node_id, node_index);
    }
}
#[allow(unused)]
fn delete_node_from_graph(mut graph: ResMut<Graph>, node_id: NodeId) {
    if let Some(node_index) = graph.correlation.remove(&node_id) {
        graph.force_graph.remove_node(node_index);
    }
}

#[allow(unused)]
fn update_graph(graph: &mut ResMut<Graph>, time: Res<Time>) {
    graph.force_graph.update(time.delta_secs());
}

#[allow(unused)]
fn get_coordinates(graph: &Graph, node_id: NodeId) -> (f32, f32) {
    let node_index: &NodeIndex = graph.correlation.get(&node_id).unwrap();
    let node = &graph
        .force_graph
        .get_graph()
        .node_weight(*node_index)
        .unwrap()
        .data;
    (node.x, node.y)
}

#[allow(unused)]
fn update_graph_positions(mut graph: ResMut<Graph>, time: Res<Time>) {
    graph.force_graph.update(time.delta_secs());
}

#[allow(unused)]
fn update_nodes_positions(graph: &mut ResMut<Graph>, mut query: Query<(&Node, &mut Transform)>) {
    for (node, mut transform) in query.iter_mut() {
        let (x, y) = get_coordinates(&graph, node.id);
        transform.translation = Vec3::new(x, y, 0.0);
    }
}
