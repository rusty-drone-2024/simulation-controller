use super::components::{Edge, EdgeForceGraphMarker, Node, NodeForceGraphMarker};
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
                force_charge: 10000.0,
                force_spring: 0.3,
                force_max: 200.0,
                node_speed: 5000.0,
                damping_factor: 0.92,
            }),
            anchor_index: None,
        };
        //Adding an anchor node in the
        let idx = item.data.add_node(NodeData {
            x: -200.0,
            y: 0.0,
            mass: 0.1,
            is_anchor: true,
            ..Default::default()
        });
        item.anchor_index = Some(idx);
        item
    }
}

//TODO removal
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyForceGraph::new());
        app.add_systems(Update, update_graph);
        app.add_systems(Update, update_positions);
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
                && force_graph.data.contains(start_node)
                && force_graph.data.contains(end_node)
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
        if force_graph.data.contains(petgraph.index) {
            let (x, y) = force_graph.data.get_node_position(petgraph.index);
            transform.translation = Vec3::new(x, y, 0.0);
        }
    }
}
