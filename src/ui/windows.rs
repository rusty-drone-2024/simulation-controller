use crate::ui::components::{CrashMarker, Drone, Leaf, Node, SelectedMarker};

use bevy::prelude::*;
use bevy_egui::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui_state)
            .add_systems(Update, window);
    }
}

#[derive(Resource, Debug)]
pub struct UiState {
    pub pdr: Option<String>,
}

fn initialize_ui_state(mut commands: Commands) {
    commands.insert_resource(UiState { pdr: None });
}

fn window(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut query_drone: Query<(Entity, &Node, &mut Drone), (With<SelectedMarker>, Without<Leaf>)>,
    query_leaf: Query<(&Node, &Leaf), (With<SelectedMarker>, Without<Drone>)>,
    mut ui_state: ResMut<UiState>,
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
                        for (entity, node, mut drone) in query_drone.iter_mut() {
                            ui.heading(format!("Drone with id: {:?}", node.id));
                            ui.horizontal(|ui| {
                                ui.label(format!("PDR:"));
                                ui.text_edit_singleline(ui_state.pdr.as_mut().unwrap());
                                if ui.button("Update").clicked() {
                                    if let Some(pdr_s) = &ui_state.pdr {
                                        if let Ok(pdr) = pdr_s.parse::<f32>() {
                                            if pdr > 0.0 && pdr <= 1.0 {
                                                let _res = drone.set_packet_drop_rate(pdr);
                                                println!(
                                                    "New PDR: {}",
                                                    ui_state.pdr.as_ref().unwrap()
                                                );
                                            }
                                        }
                                    }
                                }
                            });
                            ui.label(format!("Neighbors: {:?}", node.neighbours));

                            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                                if ui
                                    .add_sized(
                                        [50.0, 10.0],
                                        egui::Button::new("Crash").fill(egui::Color32::RED),
                                    )
                                    .clicked()
                                {
                                    println!("Trying to crash the drone with id: {:?}", node.id);
                                    commands.entity(entity).insert(CrashMarker);
                                }
                            });
                        }
                    } else if query_leaf.iter().count() > 0 {
                        for (node, leaf) in query_leaf.iter() {
                            ui.heading(format!("{} with id: {:?}", leaf.leaf_type, node.id));
                            ui.label(format!("Neighbors: {:?}", node.neighbours));
                        }
                    } else {
                        ui.heading("Click on a node to see its info");
                        ui.label("Once you selected a node you can perform actions on it");
                    }
                });
        });
}
