mod systems;
mod components;
mod resources;

use bevy::prelude::*;
use systems::{update_graph, remove_items, update_nodes, update_edges, update_text, update_selector};
use resources::MyForceGraph;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyForceGraph::new());
        app.add_systems(Update, update_graph);
        app.add_systems(FixedUpdate, remove_items);
        app.add_systems(Update, update_nodes);
        app.add_systems(Update, update_edges);
        app.add_systems(Update, update_text);
        app.add_systems(Update, update_selector);
    }
}