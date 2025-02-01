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
    pub start_node: Option<String>,
    pub end_node: Option<String>,
}

fn initialize_ui_state(mut commands: Commands) {
    commands.insert_resource(UiState {
        pdr: None,
        start_node: Some(0.to_string()),
        end_node: Some(0.to_string()),
    });
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
            fill: egui::Color32::from_rgb(102, 102, 204),
            stroke: egui::Stroke::new(4.0, egui::Color32::from_rgb(102, 102, 204)),
            inner_margin: egui::Margin::same(4.0),
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            let frame_style = egui::Frame {
                fill: egui::Color32::from_rgb(30, 30, 30),
                stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 250)),
                rounding: egui::Rounding::same(10.0),
                inner_margin: egui::Margin::same(4.0), // Keep inner frame stroke within bounds
                ..Default::default()
            };

            frame_style.show(ui, |ui| {
                ui.set_min_height(200.0);
                ui.add_space(10.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Menu:");
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label("Create edge from:  ");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(ui_state.start_node.as_mut().unwrap()),
                    );
                    ui.label("to:");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(ui_state.end_node.as_mut().unwrap()),
                    );
                    if ui.button("Add").clicked() {}
                });
                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label("Delete edge from: ");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(ui_state.start_node.as_mut().unwrap()),
                    );
                    ui.label("to:");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(ui_state.end_node.as_mut().unwrap()),
                    );
                    if ui.button("Remove").clicked() {}
                });
                ui.add_space(2.0);
                ui.separator();
                ui.add_space(10.0);
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(400.0, 40.0),
                    egui::Layout::bottom_up(egui::Align::Center),
                    |ui| {
                        if ui
                            .add_sized(
                                [140.0, 40.0],
                                egui::Button::new("Spawn a new drone")
                                    .fill(egui::Color32::DARK_GREEN),
                            )
                            .clicked()
                        {
                            println!("Trying to spawn a drone");
                        }
                    },
                );
                ui.add_space(10.0);
            });

            ui.add_space(10.0);

            frame_style.show(ui, |ui| {
                let available_size = ui.available_size();
                egui::ScrollArea::vertical()
                    .max_height(available_size.y)
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        if query_drone.iter().count() > 0 {
                            for (entity, node, mut drone) in query_drone.iter_mut() {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(format!("Drone with id: {:?}", node.id));
                                    ui.add_space(10.0);
                                    ui.separator();
                                    ui.add_space(10.0);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("PDR:");
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

                                ui.with_layout(
                                    egui::Layout::bottom_up(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .add_sized(
                                                [100.0, 40.0],
                                                egui::Button::new("Crash")
                                                    .fill(egui::Color32::DARK_RED),
                                            )
                                            .clicked()
                                        {
                                            println!(
                                                "Trying to crash the drone with id: {:?}",
                                                node.id
                                            );
                                            commands.entity(entity).insert(CrashMarker);
                                        }
                                    },
                                );
                            }
                        } else if query_leaf.iter().count() > 0 {
                            for (node, leaf) in query_leaf.iter() {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(format!(
                                        "{} with id: {:?}",
                                        leaf.leaf_type, node.id
                                    ));
                                    ui.add_space(10.0);
                                    ui.separator();
                                    ui.add_space(10.0);
                                });
                                ui.label(format!("Neighbors: {:?}", node.neighbours));
                            }
                        } else {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                ui.heading("Click on a node to see its info");
                                ui.add_space(10.0);
                                ui.label("Once you selected a node you can perform actions on it");
                                ui.add_space(10.0);
                            });
                        }
                    });
            });
        });
}
