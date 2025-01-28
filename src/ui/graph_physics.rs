use bevy::prelude::*;
use force_graph::{ForceGraph, NodeData, SimulationParameters};
use petgraph::prelude::NodeIndex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use wg_2024::network::NodeId;

use super::components::Node;

pub struct PhysicsPlugin;

/*
TODO:
add node removal from graph
make node move faster
*/
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Graph::new());
        app.add_systems(Update, fill_graph);
        app.add_systems(Update, update_graph_positions);
        app.add_systems(Update, update_nodes_positions);
    }
}

#[allow(unused)]
fn log_with_timestamp(message: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    info!("[{:.3}] {}", now, message);
}

//TODO make this do it just once, make it also remove nodes
fn fill_graph(query: Query<(&Node, &Transform), Added<Node>>, mut graph: ResMut<Graph>) {
    //log_with_timestamp("fill_graph system called");
    for (node, transform) in query.iter() {
        add_node_to_graph(
            &mut graph,
            node.id,
            transform.translation.x,
            transform.translation.y,
        );
        for &neighbor_id in &node.neighbours {
            add_neighbor_to_graph(&mut graph, node.id, neighbor_id);
        }
    }
}

fn update_graph_positions(mut graph: ResMut<Graph>, time: Res<Time>) {
    //log_with_timestamp("update_graph_positions system called");
    graph.force_graph.update(time.delta_secs());
}

fn update_nodes_positions(graph: Res<Graph>, mut query: Query<(&Node, &mut Transform)>) {
    //log_with_timestamp("update_nodes_positions system called");
    const SHIFT: f32 = 200.0;
    const SCALE: f32 = 1.2;
    for (node, mut transform) in query.iter_mut() {
        if let Some((x, y)) = get_coordinates(&graph, node.id) {
            transform.translation = Vec3::new(x * SCALE - SHIFT, y * SCALE, 0.0);
        }
    }
}

fn get_coordinates(graph: &Graph, node_id: NodeId) -> Option<(f32, f32)> {
    graph
        .correlation
        .get(&node_id)
        .and_then(|node_index| graph.force_graph.get_graph().node_weight(*node_index))
        .map(|node| (node.data.x, node.data.y))
}

//TODO MOVE ALL THE CODE BELOW TO ANOTHER FILE
#[derive(Resource)]
pub struct Graph {
    pub correlation: HashMap<NodeId, NodeIndex>,
    pub force_graph: ForceGraph<NodeData>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            correlation: HashMap::new(),
            force_graph: ForceGraph::new(SimulationParameters {
                force_charge: 100.0,
                force_spring: 0.05,
                force_max: 10.0,
                node_speed: 500.0,
                damping_factor: 0.90,
            }),
        }
    }
}

fn add_node_to_graph(graph: &mut ResMut<Graph>, node_id: NodeId, x: f32, y: f32) {
    if !graph.correlation.contains_key(&node_id) {
        let node_index = graph.force_graph.add_node(NodeData {
            x: x,
            y: y,
            ..Default::default()
        });
        graph.correlation.insert(node_id, node_index);
    }
}

fn delete_node_from_graph(mut graph: ResMut<Graph>, node_id: NodeId) {
    if let Some(node_index) = graph.correlation.remove(&node_id) {
        graph.force_graph.remove_node(node_index);
    }
}

fn add_neighbor_to_graph(graph: &mut ResMut<Graph>, node_id: NodeId, neighbor_id: NodeId) {
    if let (Some(&node_index), Some(&neighbor_index)) = (
        graph.correlation.get(&node_id),
        graph.correlation.get(&neighbor_id),
    ) {
        graph
            .force_graph
            .add_edge(node_index, neighbor_index, Default::default());
    };
}
