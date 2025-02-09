use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod ui;

use network_initializer::network::Network;

use ui::camera::CameraPlugin;
use ui::commands::CommandsPlugin;
use ui::drone_system::DronePlugin;
use ui::event_listener::EventListenerPlugin;
use ui::graph_physics::PhysicsPlugin;
use ui::initializer::SpawnTopologyPlugin;
use ui::res_init::InitResourcesPlugin;
use ui::resources::NetworkResource;
use ui::settings::SettingsPlugin;
use ui::utils::AddersPlugins;
use ui::windows::WindowPlugin;

pub fn loop_forever_sc(network: Network) {
    App::new()
        .insert_resource(NetworkResource { data: network })
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
        .add_plugins(SettingsPlugin)
        .run();
}
