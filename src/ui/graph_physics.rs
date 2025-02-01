use super::components::{Edge, EdgeForceGraphMarker, Node, NodeForceGraphMarker, Text};
use bevy::prelude::*;
use force_graph::{ForceGraph, NodeData, SimulationParameters};

#[derive(Resource)]
pub struct MyForceGraph {
    pub data: ForceGraph<NodeData>,
    pub anchor_index: Option<petgraph::stable_graph::NodeIndex<u8>>,
}

impl MyForceGraph {
    pub fn new() -> Self {
        let mut item = MyForceGraph {
            data: ForceGraph::new(SimulationParameters {
                force_charge: 9000.0,
                force_spring: 0.3,
                force_max: 280.0,
                node_speed: 7000.0,
                damping_factor: 0.95,
            }),
            anchor_index: None,
        };
        //Adding an anchor node in the
        let idx = item.data.add_node(NodeData {
            x: -300.0,
            y: 0.0,
            mass: 0.1,
            is_anchor: true,
            ..Default::default()
        });
        item.anchor_index = Some(idx);
        item
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyForceGraph::new());
        app.add_systems(Update, update_graph);
        app.add_systems(Update, update_positions);
        app.add_systems(FixedUpdate, remove_items);
        app.add_systems(Update, update_text_positions);
    }
}

fn update_graph(
    mut commands: Commands,
    mut force_graph: ResMut<MyForceGraph>,
    nodes: Query<(Entity, &Transform), Without<NodeForceGraphMarker>>,
    edges: Query<(Entity, &Edge), Without<EdgeForceGraphMarker>>,
    nodes_in_graph: Query<(&Node, &NodeForceGraphMarker)>,
) {
    let anchor_index = force_graph.anchor_index.unwrap();
    for (entity, transform) in nodes.iter() {
        let petgraph_index = force_graph.data.add_node(NodeData {
            x: transform.translation.x,
            y: transform.translation.y,
            ..Default::default()
        });
        force_graph
            .data
            .add_edge(anchor_index, petgraph_index, Default::default());
        commands.entity(entity).insert(NodeForceGraphMarker {
            index: petgraph_index,
        });
    }
    for (entity, edge) in edges.iter() {
        let mut start_node_index = None;
        let mut end_node_index = None;

        for (node, petgraph) in nodes_in_graph.iter() {
            if edge.start_node == node.id {
                start_node_index = Some(petgraph.index);
            }
            if edge.end_node == node.id {
                end_node_index = Some(petgraph.index);
            }
        }

        if let (Some(start_node), Some(end_node)) = (start_node_index, end_node_index) {
            if start_node != end_node
                && force_graph.data.contains_node(start_node)
                && force_graph.data.contains_node(end_node)
            {
                force_graph
                    .data
                    .add_edge(start_node, end_node, Default::default());
            }
            commands.entity(entity).insert(EdgeForceGraphMarker {
                start_node,
                end_node,
            });
        }
    }
}

fn update_positions(
    mut force_graph: ResMut<MyForceGraph>,
    time: Res<Time>,
    mut nodes: Query<(&mut Transform, &NodeForceGraphMarker), With<Node>>,
) {
    force_graph.data.update(time.delta_secs());
    for (mut transform, petgraph) in nodes.iter_mut() {
        if force_graph.data.contains_node(petgraph.index) {
            let (x, y) = force_graph.data.get_node_position(petgraph.index);
            transform.translation = Vec3::new(x, y, 0.0);
        }
    }
}

fn remove_items(
    mut force_graph: ResMut<MyForceGraph>,
    nodes: Query<&NodeForceGraphMarker>,
    edges: Query<&EdgeForceGraphMarker>,
) {
    let node_count = force_graph.data.get_graph().node_count() - 1;
    if node_count > nodes.iter().count() {
        let indices: Vec<_> = force_graph
            .data
            .get_nodes_indices()
            .iter()
            .cloned()
            .collect();
        for idx in indices.iter() {
            if !nodes.iter().any(|node| node.index == *idx)
                && idx != &force_graph.anchor_index.unwrap()
            {
                force_graph.data.remove_node(*idx);
            }
        }
    }

    let edge_count =
        force_graph.data.get_graph().edge_count() - force_graph.data.get_graph().node_count();
    if edge_count > edges.iter().count() {
        let indices: Vec<_> = force_graph
            .data
            .get_edges_indices()
            .iter()
            .cloned()
            .collect();
        for (start, end) in indices.iter() {
            if !edges
                .iter()
                .any(|edge| edge.start_node == *start && edge.end_node == *end)
                && start != &force_graph.anchor_index.unwrap()
            {
                force_graph.data.remove_edge(*start, *end);
            }
        }
    }
}

fn update_text_positions(mut query_text: Query<(&Text, &mut Transform)>, query_node: Query<(Entity, &Transform), (With<Node>, Without<Text>)>) {
    for (text, mut transform) in query_text.iter_mut() {
        for (entity, node_transform) in query_node.iter() {
            if entity == text.entity_id {
                transform.translation = Vec3::new(node_transform.translation.x, node_transform.translation.y + 15.0, 15.0);
            }
        }
    }
}