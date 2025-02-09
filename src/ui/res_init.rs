use crate::ui::resources::ModeConfig;
use bevy::{prelude::*, winit::WinitSettings};
pub struct InitResourcesPlugin;

impl Plugin for InitResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::game())
            .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.8)))
            .insert_resource(Time::<Fixed>::from_seconds(1.0))
            .insert_resource(initialize_mode_config());
    }
}

fn initialize_mode_config() -> ModeConfig {
    #[cfg(feature = "dev")]
    {
        println!("Development mode");
        ModeConfig { bypass_cheks: true }
    }

    #[cfg(not(feature = "dev"))]
    {
        println!("Production mode");
        ModeConfig {
            bypass_cheks: false,
        }
    }
}
