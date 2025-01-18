use bevy::prelude::*;

mod ui;

use ui::spawn::SpawnTopology;
use ui::camera::CameraPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.6, 0.6, 0.9)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpawnTopology)
        .run();
}
