use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod ui;

use ui::camera::CameraPlugin;
use ui::commands::CommandsPlugin;
use ui::drone_system::DronePlugin;
use ui::event_listener::EventListenerPlugin;
use ui::graph_physics::PhysicsPlugin;
use ui::initializer::SpawnTopologyPlugin;
use ui::res_init::InitResourcesPlugin;
use ui::utils::AddersPlugins;
use ui::windows::WindowPlugin;

pub fn loop_forever_sc() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(InitResourcesPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpawnTopologyPlugin)
        .add_plugins(WindowPlugin)
        .add_plugins(DronePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CommandsPlugin)
        .add_plugins(EventListenerPlugin)
        .add_plugins(AddersPlugins)
        .run();
}
