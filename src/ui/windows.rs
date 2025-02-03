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
pub struct MainUiState {
    pub new_pdr: Option<String>,
    pub nbhg_1: Option<String>,
    pub nbhg_2: Option<String>,
}
#[derive(Resource, Debug)]
pub struct SelectedUiState {
    pub pdr: Option<String>,
    pub node_to_add: Option<String>,
    pub node_to_rmv: Option<String>,
}

fn initialize_ui_state(mut commands: Commands) {
    commands.insert_resource(MainUiState {
        new_pdr: Some(0.0.to_string()),
        nbhg_1: Some(0.to_string()),
        nbhg_2: Some(0.to_string()),
    });
    commands.insert_resource(SelectedUiState {
        pdr: Some(0.0.to_string()),
        node_to_add: Some(0.to_string()),
        node_to_rmv: Some(0.to_string()),
    });
}

fn window(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut query_drone: Query<(Entity, &Node, &mut Drone), (With<SelectedMarker>, Without<Leaf>)>,
    query_leaf: Query<(&Node, &Leaf), (With<SelectedMarker>, Without<Drone>)>,
    mut query_all_nodes: Query<&Node, Without<SelectedMarker>>,
    mut main_state: ResMut<MainUiState>,
    mut selected_state: ResMut<SelectedUiState>,
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
                inner_margin: egui::Margin::same(4.0),
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
                    ui.label("Create a new drone with PDR:");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(main_state.new_pdr.as_mut().unwrap()),
                    );
                });
                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label("Connected with nodes (id): ");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(main_state.nbhg_1.as_mut().unwrap()),
                    );
                    ui.label(" & ");
                    ui.add_sized(
                        [60.0, 20.0],
                        egui::TextEdit::singleline(main_state.nbhg_2.as_mut().unwrap()),
                    );
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
                                });
                                ui.horizontal(|ui| {
                                    ui.label(format!("Neighbors: {:?}", node.neighbours));
                                    ui.add_space(20.0);
                                    ui.label("PDR:");
                                    ui.add_sized(
                                        [60.0, 20.0],
                                        egui::TextEdit::singleline(selected_state.pdr.as_mut().unwrap()),
                                    );
                                    if ui.button("Update").clicked() {
                                        if let Some(pdr_s) = &selected_state.pdr {
                                            if let Ok(pdr) = pdr_s.parse::<f32>() {
                                                if pdr > 0.0 && pdr <= 1.0 {
                                                    let _res = drone.set_packet_drop_rate(pdr);
                                                    println!(
                                                        "New PDR: {}",
                                                        selected_state.pdr.as_ref().unwrap()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                });
                                ui.add_space(10.0);
                                ui.separator();
                                ui.separator();
                                ui.heading("Infos:");
                                //TODO: Add more infos
                                ui.add_space(100.0);
                                // END TODO
                                ui.add_space(10.0);
                                ui.separator();
                                ui.separator();
                                ui.heading("Actions:");
                                ui.add_space(10.0);
                                ui.horizontal(|ui|{
                                    ui.label("Add a connection with node with id:");
                                    ui.add_sized(
                                        [60.0, 20.0],
                                        egui::TextEdit::singleline(selected_state.node_to_add.as_mut().unwrap()),
                                    );
                                    if ui.button("Add").clicked() {
                                        if let Some(node_id) = &selected_state.node_to_add {
                                            if let Ok(id) = node_id.parse::<u8>() {
                                                println!("Trying to add a connection with node with id: {:?}", id);
                                            }
                                        }
                                    }
                                });
                                ui.add_space(10.0);
                                ui.horizontal(|ui|{
                                    ui.label("Remove a connection with node (id):");
                                    ui.add_sized(
                                        [60.0, 20.0],
                                        egui::TextEdit::singleline(selected_state.node_to_rmv.as_mut().unwrap()),
                                    );
                                    if ui.button("Remove").clicked() {
                                        if let Some(node_id) = &selected_state.node_to_rmv {
                                            if let Ok(id) = node_id.parse::<u8>() {
                                                println!("Trying to remove a connection with node (id): {:?}", id);
                                            }
                                        }
                                    }
                                });

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
