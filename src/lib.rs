#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod core;
mod physics;
use physics::PhysicsPlugin;
mod settings;
use settings::SettingsPlugin;
mod window;
use window::WindowPlugin;


use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use network_initializer::network::Network;
use core::camera::CameraPlugin;
use core::commands::CommandsPlugin;
use core::drone_system::DronePlugin;
use core::event_listener::EventListenerPlugin;
use core::initializer::SpawnTopologyPlugin;
use core::res_init::InitResourcesPlugin;
use core::resources::NetworkResource;
use core::utils::UtilsPlugins;

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
        .add_plugins(UtilsPlugins)
        .add_plugins(SettingsPlugin)
        .run();
}
