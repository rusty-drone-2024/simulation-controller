use bevy::prelude::*;
use bevy_egui::*;

use super::components::Node;
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rigth_window);
    }
}

fn rigth_window(mut contexts: EguiContexts) {
    egui::SidePanel::right("Info")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Hello World!");
            ui.button("text").clicked();
        });
}

pub fn observer_drone(trigger: Trigger<Pointer<Click>>, mut query: Query<(&Node, &mut Transform)>) {
    let entity = trigger.entity();

    for (node, mut transform) in query.iter_mut() {
        if node.entity_id == entity {
            println!("Node with ID {:?} clicked!", node.id);
            transform.translation += Vec3::new(0.0, 10.0, 0.0);
        }
    }
}

pub fn observer_leaf(trigger: Trigger<Pointer<Click>>, mut query: Query<(&Node, &mut Transform)>) {
    let entity = trigger.entity();

    for (node, mut transform) in query.iter_mut() {
        if node.entity_id == entity {
            println!("Node with ID {:?} clicked!", node.id);
            transform.translation += Vec3::new(0.0, -10.0, 0.0);
        }
    }
}
