use super::components::{Drone, Node, SelectedMarker};
use super::windows::SelectedUiState;

use bevy::prelude::*;

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
    for (node, drone, transform) in to_select_node_query.iter_mut() {
        if node.entity_id == entity {
            selected_state.pdr = Some(drone.pdr.clone().to_string());
            commands.entity(entity).insert(SelectedMarker);
            for (mut selector, mut visibility) in selector_query.iter_mut() {
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
    for (node, transform) in to_select_node_query.iter_mut() {
        if node.entity_id == entity {
            selected_state.pdr = None;
            commands.entity(entity).insert(SelectedMarker);
            for (mut selector, mut visibility) in selector_query.iter_mut() {
                selector.translation =
                    Vec3::new(transform.translation.x, transform.translation.y, -10.0);
                *visibility = Visibility::Visible;
            }
        }
    }
}
