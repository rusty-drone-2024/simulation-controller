use bevy::{prelude::*, winit::WinitSettings};

pub struct InitResourcesPlugin;

impl Plugin for InitResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.8)));
    }
}
