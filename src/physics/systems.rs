use bevy::prelude::*;
use force_graph::{EdgeData, NodeData};
use crate::core::components::{Edge, Node, Text, SelectedMarker, SelectionSpriteMarker};
use super::{components::{NodeForceGraphMarker, EdgeForceGraphMarker}, resources::MyForceGraph};

pub fn update_graph(
    mut commands: Commands,
    mut force_graph: ResMut<MyForceGraph>,
    nodes: Query<(Entity, &Transform), (With<Node>, Without<NodeForceGraphMarker>)>,
    edges: Query<(Entity, &Edge), Without<EdgeForceGraphMarker>>,
    nodes_in_graph: Query<(&Node, &NodeForceGraphMarker)>,
) {
    for (entity, transform) in nodes.iter() {
        let petgraph_index = force_graph.data.add_node(NodeData {
            x: transform.translation.x,
            y: transform.translation.y,
            ..Default::default()
        });
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
                    .add_edge(start_node, end_node, EdgeData::default());
            }
            commands.entity(entity).insert(EdgeForceGraphMarker {
                start_node,
                end_node,
            });
        }
    }
}

pub fn remove_items(
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
            .copied()
            .collect();
        for idx in &indices {
            if !nodes.iter().any(|node| node.index == *idx) {
                force_graph.data.remove_node(*idx);
            }
        }
    }

    let edge_count =
        force_graph.data.get_graph().edge_count() - force_graph.data.get_graph().node_count();
    if edge_count > edges.iter().count() {
        let indices: Vec<_> = force_graph.data.get_edges_indices().clone();
        for (start, end) in &indices {
            if !edges
                .iter()
                .any(|edge| edge.start_node == *start && edge.end_node == *end)
            {
                force_graph.data.remove_edge(*start, *end);
            }
        }
    }
}

pub fn update_nodes(
    mut force_graph: ResMut<MyForceGraph>,
    time: Res<Time>,
    mut nodes: Query<(&mut Transform, &NodeForceGraphMarker), With<Node>>,
) {
    force_graph.data.update(time.delta_secs());
    for (mut transform, petgraph) in &mut nodes {
        if force_graph.data.contains_node(petgraph.index) {
            let (x, y) = force_graph.data.get_node_position(petgraph.index);
            transform.translation = Vec3::new(x, y, 0.0);
        }
    }
}

pub fn update_edges(
    mut edge_query: Query<(&Edge, &mut Transform)>,
    node_query: Query<(&Node, &Transform), Without<Edge>>,
) {
    for (edge, mut edge_transform) in &mut edge_query {
        let start_node_transform = node_query
            .iter()
            .find(|(node, _)| node.id == edge.start_node)
            .map(|(_, transform)| transform.translation);
        let end_node_transform = node_query
            .iter()
            .find(|(node, _)| node.id == edge.end_node)
            .map(|(_, transform)| transform.translation);

        if let (Some(start_position), Some(end_position)) =
            (start_node_transform, end_node_transform)
        {
            let midpoint = (start_position + end_position) / 2.0;
            let direction = end_position - start_position;
            let angle = direction.y.atan2(direction.x);
            let distance = direction.length() - 40.0;

            edge_transform.translation = midpoint;
            edge_transform.rotation = Quat::from_rotation_z(angle);
            edge_transform.scale = Vec3::new(distance, 1.0, 1.0);
        }
    }
}

pub fn update_text(
    mut query_text: Query<(&Text, &mut Transform)>,
    query_node: Query<(Entity, &Transform), (With<Node>, Without<Text>)>,
) {
    for (text, mut transform) in &mut query_text {
        for (entity, node_transform) in &query_node {
            if entity == text.entity_id {
                transform.translation = Vec3::new(
                    node_transform.translation.x,
                    node_transform.translation.y + 15.0,
                    15.0,
                );
            }
        }
    }
}

pub fn update_selector(
    node_query: Query<&Transform, (With<SelectedMarker>, Without<SelectionSpriteMarker>)>,
    mut selector_query: Query<(&mut Transform, &mut Visibility), With<SelectionSpriteMarker>>,
) {
    if node_query.iter().count() == 0 {
        for (_transform, mut visibility) in &mut selector_query {
            *visibility = Visibility::Hidden;
        }
    }
    for node_transform in node_query.iter() {
        for (mut transform, _visibility) in &mut selector_query {
            transform.translation = node_transform.translation;
        }
    }
}