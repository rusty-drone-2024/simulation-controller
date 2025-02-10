use std::cmp::min;

use super::resources::{MainUiState, SelectedUiState};
use crate::components::{
    CrashMarker, Drone, Leaf,
    LeafType::{Client, Server},
    Node, SelectedMarker,
};
use crate::event_listener::resources::Bytes;
use crate::event_listener::DisplayedInfo;
use crate::events::{AddDroneEvent, AddEdgeEvent, RmvEdgeEvent};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn initialize_ui_state(mut commands: Commands) {
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

pub fn observer_drone(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    last_selected_node_query: Query<Entity, With<SelectedMarker>>,
    mut to_select_node_query: Query<(&mut Node, &Drone, &Transform), Without<SelectedMarker>>,
    mut selector_query: Query<(&mut Transform, &mut Visibility), (Without<Node>, Without<Camera>)>,
    mut selected_state: ResMut<SelectedUiState>,
) {
    let entity = trigger.entity();

    for entity in last_selected_node_query.iter() {
        commands.entity(entity).remove::<SelectedMarker>();
    }
    for (node, drone, transform) in &mut to_select_node_query {
        if node.entity_id == entity {
            selected_state.pdr = Some(drone.pdr.clone().to_string());
            commands.entity(entity).insert(SelectedMarker);
            for (mut selector, mut visibility) in &mut selector_query {
                selector.translation =
                    Vec3::new(transform.translation.x, transform.translation.y, -10.0);
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn observer_leaf(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    last_selected_node_query: Query<Entity, With<SelectedMarker>>,
    mut to_select_node_query: Query<(&mut Node, &Transform), Without<SelectedMarker>>,
    mut selector_query: Query<(&mut Transform, &mut Visibility), (Without<Node>, Without<Camera>)>,
    mut selected_state: ResMut<SelectedUiState>,
) {
    let entity = trigger.entity();

    for entity in last_selected_node_query.iter() {
        commands.entity(entity).remove::<SelectedMarker>();
    }
    for (node, transform) in &mut to_select_node_query {
        if node.entity_id == entity {
            selected_state.pdr = None;
            commands.entity(entity).insert(SelectedMarker);
            for (mut selector, mut visibility) in &mut selector_query {
                selector.translation =
                    Vec3::new(transform.translation.x, transform.translation.y, -10.0);
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn window(
    mut commands: Commands,
    mut ew_add_drone: EventWriter<AddDroneEvent>,
    mut ew_add_edge: EventWriter<AddEdgeEvent>,
    mut ew_rmv_edge: EventWriter<RmvEdgeEvent>,
    mut contexts: EguiContexts,
    mut main_state: ResMut<MainUiState>,
    mut selected_state: ResMut<SelectedUiState>,
    mut query_drone: Query<(Entity, &Node, &mut Drone), (With<SelectedMarker>, Without<Leaf>)>,
    query_leaf: Query<(&Node, &Leaf), (With<SelectedMarker>, Without<Drone>)>,
    info: Res<DisplayedInfo>,
) {
    egui::SidePanel::right("Info")
        .resizable(false)
        .min_width(400.0)
        .max_width(400.0)
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
                            if let (Some(pdr_s), Some(nghb_1_s), Some(nghb_2_s)) =
                                (&main_state.new_pdr, &main_state.nbhg_1, &main_state.nbhg_2)
                            {
                                if let (Ok(pdr), Ok(nghb_1), Ok(nghb_2)) = (
                                    pdr_s.parse::<f32>(),
                                    nghb_1_s.parse::<u8>(),
                                    nghb_2_s.parse::<u8>(),
                                ) {
                                    if (0.0..=1.0).contains(&pdr)
                                        && nghb_1 > 0
                                        && nghb_2 > 0
                                        && nghb_1 != nghb_2
                                    {
                                        println!("Trying to spawn a new drone");
                                        ew_add_drone.send(AddDroneEvent {
                                            pdr,
                                            ngbs: vec![nghb_1, nghb_2],
                                        });
                                    }
                                }
                            }
                        }
                    },
                );
                ui.add_space(10.0);
            });
            ui.add_space(10.0);

            // NODE PART OF THE UI WINDOW
            frame_style.show(ui, |ui| {
                let available_size = ui.available_size();
                egui::ScrollArea::vertical()
                    .max_height(available_size.y)
                    .show(ui, |ui| {
                        ui.add_space(10.0);

                        // SELECTED NODE IS DRONE
                        if query_drone.iter().count() > 0 {
                            for (entity, node, mut drone) in &mut query_drone {
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
                                        egui::TextEdit::singleline(
                                            selected_state.pdr.as_mut().unwrap(),
                                        ),
                                    );
                                    if ui.button("Update").clicked() {
                                        if let Some(pdr_s) = &selected_state.pdr {
                                            if let Ok(pdr) = pdr_s.parse::<f32>() {
                                                if (0.0..=1.0).contains(&pdr) {
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

                                // Drone info
                                ui.separator();
                                ui.heading("Infos:");
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(6.0);
                                    ui.label(format!(
                                        "Packets sent: {:?}",
                                        info.drone
                                            .contains_key(&node.id)
                                            .then(|| { info.drone[&node.id].packets_sent })
                                            .unwrap_or(0)
                                    ));
                                    ui.add_space(80.0);
                                    ui.label(format!(
                                        "Packets shortcutted: {:?}",
                                        info.drone
                                            .contains_key(&node.id)
                                            .then(|| { info.drone[&node.id].packets_shortcutted })
                                            .unwrap_or(0)
                                    ));
                                });
                                ui.horizontal(|ui| {
                                    ui.add_space(6.0);
                                    ui.label(format!(
                                        "Data sent: {}",
                                        info.drone
                                            .contains_key(&node.id)
                                            .then(|| { info.drone[&node.id].data_sent.clone() })
                                            .unwrap_or(Bytes(0))
                                    ));
                                    ui.add_space(86.0);
                                    ui.label(format!(
                                        "Data dropped: {}",
                                        info.drone
                                            .contains_key(&node.id)
                                            .then(|| { info.drone[&node.id].data_dropped.clone() })
                                            .unwrap_or(Bytes(0))
                                    ));
                                });
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.add_space(6.0);
                                    ui.label("Neighbour usage percentages:");
                                });
                                ui.horizontal(|ui| {
                                    ui.add_space(6.0);
                                    let usage_percentage =
                                        info.drone[&node.id].neighbour_usage_percentages();
                                    for (id, percentage) in usage_percentage {
                                        ui.add_space(2.0);
                                        ui.label(format!("[{id}] {percentage}%"));
                                    }
                                });
                                ui.add_space(10.0);
                                // Drone info end

                                ui.add_space(10.0);
                                ui.separator();
                                ui.separator();
                                ui.heading("Actions:");
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.label("Add a connection with node (id):");
                                    ui.add_sized(
                                        [60.0, 20.0],
                                        egui::TextEdit::singleline(
                                            selected_state.node_to_add.as_mut().unwrap(),
                                        ),
                                    );
                                    if ui.button("Add").clicked() {
                                        if let Some(node_id) = &selected_state.node_to_add {
                                            if let Ok(id) = node_id.parse::<u8>() {
                                                ew_add_edge.send(AddEdgeEvent {
                                                    start_node: node.id,
                                                    end_node: id,
                                                });
                                            }
                                        }
                                    }
                                });
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.label("Remove a connection with node (id):");
                                    ui.add_sized(
                                        [60.0, 20.0],
                                        egui::TextEdit::singleline(
                                            selected_state.node_to_rmv.as_mut().unwrap(),
                                        ),
                                    );
                                    if ui.button("Remove").clicked() {
                                        if let Some(node_id) = &selected_state.node_to_rmv {
                                            if let Ok(id) = node_id.parse::<u8>() {
                                                ew_rmv_edge.send(RmvEdgeEvent {
                                                    start_node: node.id,
                                                    end_node: id,
                                                });
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
                                                "Trying to crash the drone (id): {:?}",
                                                node.id
                                            );
                                            commands.entity(entity).insert(CrashMarker);
                                        }
                                    },
                                );
                            }
                        // SELECTED NODE IS LEAF
                        } else if query_leaf.iter().count() > 0 {
                            for (node, leaf) in query_leaf.iter() {
                                // LEAF IS CLIENT
                                if leaf.leaf_type == Client {
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            ui.heading(format!("Client with id: {:?}", node.id));
                                            ui.add_space(10.0);
                                            ui.separator();
                                            ui.add_space(10.0);
                                        },
                                    );
                                    ui.label(format!("Neighbors: {:?}", node.neighbours));

                                    ui.separator();
                                    ui.separator();

                                    // Client info
                                    let bytes = &info.leaf[&node.id].data_sent;
                                    let msg_n = info.leaf[&node.id].msg_n;
                                    ui.heading("Infos:");
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(6.0);
                                        ui.label(format!(
                                            "Packets sent: {:?}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].packets_sent })
                                                .unwrap_or(0)
                                        ));
                                        ui.add_space(20.0);
                                        ui.label(format!(
                                            "Data sent: {}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].data_sent.clone() })
                                                .unwrap_or(Bytes(0))
                                        ));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add_space(6.0);
                                        ui.label(format!(
                                            "Requests sent: {:?}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].msg_n })
                                                .unwrap_or(0)
                                        ));
                                        ui.add_space(20.0);

                                        if *bytes == Bytes(0) || msg_n == 0 {
                                            ui.label("Average data per message: -");
                                        } else {
                                            ui.label(format!(
                                                "Average data per message: {}",
                                                bytes.clone() / msg_n
                                            ));
                                        }
                                    });
                                    ui.separator();
                                    ui.heading("Last messages:");
                                    egui::ScrollArea::vertical()
                                        .max_height(600.0)
                                        .auto_shrink([false; 2])
                                        .show(ui, |ui| {
                                            let len = min(
                                                10,
                                                info.leaf
                                                    .contains_key(&node.id)
                                                    .then(|| info.leaf[&node.id].messages.len())
                                                    .unwrap_or(0),
                                            );
                                            for i in 0..len {
                                                ui.horizontal(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.horizontal(|ui| {
                                                        let info = info.leaf[&node.id]
                                                            .messages
                                                            .values()
                                                            .nth(i)
                                                            .unwrap();
                                                        if info.2 {
                                                            ui.label("Sent: ");
                                                        } else {
                                                            ui.label("Sending: ");
                                                        }
                                                        ui.add_space(20.0);
                                                        ui.label(format!("{:?}", info.0));
                                                        ui.add_space(20.0);
                                                        ui.label(format!("To: {:?}", info.1));
                                                    })
                                                });
                                            }
                                        });

                                // LEAF IS SERVER
                                } else if leaf.leaf_type == Server {
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            ui.heading(format!("Server with id: {:?}", node.id));
                                            ui.add_space(10.0);
                                            ui.separator();
                                            ui.add_space(10.0);
                                        },
                                    );
                                    ui.label(format!("Neighbors: {:?}", node.neighbours));

                                    ui.separator();
                                    ui.separator();

                                    // Server info
                                    let bytes = &info.leaf[&node.id].data_sent;
                                    let msg_n = info.leaf[&node.id].msg_n;
                                    ui.heading("Infos:");
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(6.0);
                                        ui.label(format!(
                                            "Packets sent: {:?}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].packets_sent })
                                                .unwrap_or(0)
                                        ));
                                        ui.add_space(20.0);
                                        ui.label(format!(
                                            "Data sent: {}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].data_sent.clone() })
                                                .unwrap_or(Bytes(0))
                                        ));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add_space(6.0);
                                        ui.label(format!(
                                            "Responses sent: {:?}",
                                            info.leaf
                                                .contains_key(&node.id)
                                                .then(|| { info.leaf[&node.id].msg_n })
                                                .unwrap_or(0)
                                        ));
                                        ui.add_space(20.0);

                                        if *bytes == Bytes(0) || msg_n == 0 {
                                            ui.label("Average data per message: -");
                                        } else {
                                            ui.label(format!(
                                                "Average data per message: {}",
                                                bytes.clone() / msg_n
                                            ));
                                        }
                                    });
                                    ui.separator();
                                    ui.heading("Last messages:");
                                    egui::ScrollArea::vertical()
                                        .max_height(600.0)
                                        .auto_shrink([false; 2])
                                        .show(ui, |ui| {
                                            let len = min(
                                                10,
                                                info.leaf
                                                    .contains_key(&node.id)
                                                    .then(|| info.leaf[&node.id].messages.len())
                                                    .unwrap_or(0),
                                            );
                                            for i in 0..len {
                                                ui.horizontal(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.horizontal(|ui| {
                                                        let info = info.leaf[&node.id]
                                                            .messages
                                                            .values()
                                                            .nth(i)
                                                            .unwrap();
                                                        if info.2 {
                                                            ui.label("Sent: ");
                                                        } else {
                                                            ui.label("Sending: ");
                                                        }
                                                        ui.add_space(20.0);
                                                        ui.label(format!("{:?}", info.0));
                                                        ui.add_space(20.0);
                                                        ui.label(format!("To: {:?}", info.1));
                                                    })
                                                });
                                            }
                                        });
                                };
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
