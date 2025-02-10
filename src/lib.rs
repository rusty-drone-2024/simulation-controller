#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod components;
mod events;
mod resources;
use resources::NetworkResource;

mod core;
use core::CorePlugin;
mod settings;
use settings::SettingsPlugin;
mod window;
use window::WindowPlugin;
mod physics;
use physics::PhysicsPlugin;
mod command_sender;
use command_sender::CommandsPlugin;
mod event_listener;
use event_listener::ListenerPlugin;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use network_initializer::network::Network;

pub fn loop_forever_sc(network: Network) {
    App::new()
        .insert_resource(NetworkResource { data: network })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(CorePlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(WindowPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CommandsPlugin)
        .add_plugins(ListenerPlugin)
        .run();
}
