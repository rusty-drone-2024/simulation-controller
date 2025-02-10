use crate::components::{Leaf, Node};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, color);
    }
}

fn color(mut drone_query: Query<(&Node, &mut Sprite), Without<Leaf>>) {
    for (node, mut sprite) in &mut drone_query {
        let colors = int_to_rgb(node.packet_channel.len());
        sprite.color = Color::srgb(colors.0, colors.1, colors.2);
    }
}

fn int_to_rgb(n: usize) -> (f32, f32, f32) {
    match n {
        0..=10 => {
            let t = (n as f32 - 1.0) / 9.0;
            let r = 255.0 * t;
            let g = 255.0;
            (r, g, 0.0)
        }
        11..=20 => {
            let t = (n as f32 - 11.0) / 9.0;
            let r = 255.0;
            let g = 255.0 * (1.0 - t);
            (r, g, 0.0)
        }
        _ => (255.0, 0.0, 0.0),
    }
}

pub fn is_connected(
    mut nodes: HashMap<u8, HashSet<u8>>,
    removed_node: Option<u8>,
    removed_edge: Option<(u8, u8)>,
) -> bool {
    if let Some(removed_id) = removed_node {
        nodes.remove(&removed_id);
        for neighbors in nodes.values_mut() {
            neighbors.remove(&removed_id);
        }
    }
    if let Some((removed_id1, removed_id2)) = removed_edge {
        if let Some(neighbors) = nodes.get_mut(&removed_id1) {
            neighbors.remove(&removed_id2);
        }
        if let Some(neighbors) = nodes.get_mut(&removed_id2) {
            neighbors.remove(&removed_id1);
        }
    }
    if nodes.is_empty() {
        return true;
    }
    let mut visited = HashSet::new();
    let start_node = *nodes.keys().next().unwrap();
    let mut stack = vec![start_node];
    while let Some(node_id) = stack.pop() {
        if visited.insert(node_id) {
            if let Some(neighbors) = nodes.get(&node_id) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    visited.len() == nodes.len()
}
