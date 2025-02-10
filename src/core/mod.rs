mod camera;
use camera::CameraPlugin;

pub mod creator;
pub mod utils;
use utils::UtilsPlugin;
pub mod initializer;
use initializer::SpawnTopologyPlugin;
pub mod sytems;
use sytems::FunctionalityPlugins;

use bevy::{
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.8)))
        .insert_resource(Time::<Fixed>::from_seconds(1.0))
        .add_plugins(CameraPlugin)
        .add_plugins(UtilsPlugin)
        .add_plugins(SpawnTopologyPlugin)
        .add_plugins(FunctionalityPlugins);
    }
}
