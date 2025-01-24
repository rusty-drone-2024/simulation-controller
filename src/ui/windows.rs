use super::components::{Drone, Leaf, Node, SelectedMarker};

use bevy::prelude::*;
use bevy_egui::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window);
    }
}



fn window(
    mut contexts: EguiContexts,
    query_drone: Query<(&Node, &Drone), (With<SelectedMarker>, Without<Leaf>)>,
    query_leaf: Query<(&Node, &Leaf), (With<SelectedMarker>, Without<Drone>)>,
) {
    egui::SidePanel::right("Info")
        .resizable(false)
        .min_width(400.0)
        .frame(egui::Frame {
            fill: egui::Color32::BLACK,
            stroke: egui::Stroke::new(4.0, egui::Color32::GRAY),
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    if query_drone.iter().count() > 0 {
                        for (node, drone) in query_drone.iter() {
                            ui.heading(format!("Drone with id: {:?}", node.id));
                            ui.label(format!("PDR: {:?}", drone.pdr));
                            ui.label(format!("Neighbors: {:?}", node.neighbours));
                        }
                    }
                    else if query_leaf.iter().count() > 0 {
                        for (node, leaf) in query_leaf.iter() {
                            ui.heading(format!("{} with id: {:?}",leaf.leaf_type, node.id));
                            ui.label(format!("Neighbors: {:?}", node.neighbours));
                        }
                    }
                });
        });
}