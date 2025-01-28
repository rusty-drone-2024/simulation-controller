use bevy::prelude::*;
use force_graph::{ForceGraph, NodeData};
use petgraph::prelude::NodeIndex;
use std::time::{SystemTime, UNIX_EPOCH};
use bevy::window::RequestRedraw;
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
        app.add_systems(FixedUpdate, update_graph_positions);
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
fn fill_graph(query: Query<(&Node, &Transform), Added<Node>>, mut graph: ResMut<Graph>, mut redraw_request_events: EventWriter<RequestRedraw>) {
    redraw_request_events.send(RequestRedraw);
    if graph.initialized {
        return;
    }
    graph.initialized = true;

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

fn update_graph_positions(mut graph: ResMut<Graph>, time: Res<Time>, mut query: Query<(&Node, &mut Transform)>) {
    graph.force_graph.update(time.delta_secs());

    const SHIFT: f32 = 200.0;
    const SCALE: f32 = 0.4;
    for (node, mut transform) in query.iter_mut() {
        if let Some((x, y)) = get_coordinates(&graph, node.id) {
            transform.translation = Vec3::new(x * SCALE - SHIFT, y * SCALE, 0.0);
        }
    }
}

fn get_coordinates(graph: &Graph, node_id: NodeId) -> Option<(f32, f32)> {
    let index = graph.correlation[node_id as usize]?;
    let node = graph.force_graph.get_graph().node_weight(index)?;
    Some((node.data.x, node.data.y))
}

//TODO MOVE ALL THE CODE BELOW TO ANOTHER FILE
#[derive(Resource)]
pub struct Graph {
    pub correlation: Vec<Option<NodeIndex>>,
    pub force_graph: ForceGraph<NodeData>,
    pub initialized: bool,
}

impl Graph {
    pub fn new() -> Self {
        let force_graph = ForceGraph::new(Default::default());

        Graph {
            correlation: vec![None; 256],
            force_graph,
            initialized: false
        }
    }
}

fn add_node_to_graph(graph: &mut ResMut<Graph>, node_id: NodeId, x: f32, y: f32) {
    let node_index = graph.force_graph.add_node(NodeData {
        x,
        y,
        ..Default::default()
    });
    graph.correlation[node_id as usize] = Some(node_index);
}

fn delete_node_from_graph(mut graph: ResMut<Graph>, node_id: NodeId) {
    if let Some(node_index) = graph.correlation[node_id as usize].take() {
        graph.force_graph.remove_node(node_index);
    }
}

fn add_neighbor_to_graph(graph: &mut ResMut<Graph>, node_id: NodeId, neighbor_id: NodeId) {
    if let (Some(node_index), Some(neighbor_index)) = (
        graph.correlation[node_id as usize],
        graph.correlation[neighbor_id as usize],
    ) {
        graph
            .force_graph
            .add_edge(node_index, neighbor_index, Default::default());
    };
}
