mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::MyForceGraph;
use systems::{
    remove_items, update_edges, update_graph, update_nodes, update_selector, update_text,
};

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
