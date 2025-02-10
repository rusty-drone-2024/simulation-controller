use bevy::{prelude::*, winit::WinitSettings};
pub struct InitResourcesPlugin;

impl Plugin for InitResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::game())
            .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.8)))
            .insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}
