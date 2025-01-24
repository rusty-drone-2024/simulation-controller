use super::components::{Node, SelectedMarker};
use bevy::prelude::*;

pub fn observer_drone(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    last_selected_node_query: Query<Entity, With<SelectedMarker>>,
    mut to_select_node_query: Query<(&mut Node, &Transform), Without<SelectedMarker>>,
    mut selector_query: Query<(&mut Transform, &mut Visibility), (Without<Node>, Without<Camera>)>,
) {
    let entity = trigger.entity();

    for entity in last_selected_node_query.iter() {
        commands.entity(entity).remove::<SelectedMarker>();
    }
    for (node, transform) in to_select_node_query.iter_mut() {
        if node.entity_id == entity {
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
) {
    let entity = trigger.entity();

    for entity in last_selected_node_query.iter() {
        commands.entity(entity).remove::<SelectedMarker>();
    }
    for (node, transform) in to_select_node_query.iter_mut() {
        if node.entity_id == entity {
            commands.entity(entity).insert(SelectedMarker);
            for (mut selector, mut visibility) in selector_query.iter_mut() {
                selector.translation =
                    Vec3::new(transform.translation.x, transform.translation.y, -10.0);
                *visibility = Visibility::Visible;
            }
        }
    }
}
