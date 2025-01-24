use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod ui;

use ui::camera::CameraPlugin;
use ui::drone_system::DronePlugin;
use ui::graph_physics::PhysicsPlugin;
use ui::res_init::InitResourcesPlugin;
use ui::spawn_topology::SpawnTopologyPlugin;
use ui::windows::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(InitResourcesPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpawnTopologyPlugin)
        .add_plugins(WindowPlugin)
        .add_plugins(DronePlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}
