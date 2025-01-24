use super::components::{Leaf, Node, SelectedMarker, SelectionSpriteMarker};
use bevy::prelude::*;

pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drone_color);
        app.add_systems(Update, update_selected_position);
    }
}

fn drone_color(mut query: Query<(&Node, &mut Sprite), Without<Leaf>>) {
    for (node, mut sprite) in query.iter_mut() {
        let colors = int_to_rgb(node.packet_channel.len());
        sprite.color = Color::srgb(colors.0, colors.1, colors.2);
    }
}

fn int_to_rgb(n: usize) -> (f32, f32, f32) {
    match n {
        0..=10 => {
            let t = (n as f32 - 1.0) / 9.0;
            let r = 255.0 * t;
            let g = 255.0;
            (r, g, 0.0)
        }
        11..=20 => {
            let t = (n as f32 - 11.0) / 9.0;
            let r = 255.0;
            let g = 255.0 * (1.0 - t);
            (r, g, 0.0)
        }
        _ => (255.0, 0.0, 0.0),
    }
}

fn update_selected_position(
    node_query: Query<&Transform, (With<SelectedMarker>, Without<SelectionSpriteMarker>)>,
    mut selector_query: Query<&mut Transform, With<SelectionSpriteMarker>>,
) {
    for node_transform in node_query.iter() {
        for mut transform in selector_query.iter_mut() {
            transform.translation = node_transform.translation;
        }
    }
}
