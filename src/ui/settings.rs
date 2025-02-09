use bevy::prelude::*;
use bevy_egui::*;

#[derive(Debug, Resource)]
pub struct SettingsState {
    pub music: bool,
    pub unchecked: bool,
}

#[derive(Event)]
struct UpdateMusic {
    music: bool,
}

#[derive(Event)]
struct UpdateUnchecked {
    music: bool,
}

#[derive(Event)]
struct ResetInfos;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingsState {
            unchecked: false,
            music: true,
        });
        app.add_systems(Update, settings_window);
    }
}

fn settings_window(mut contexts: EguiContexts, mut ui_state: ResMut<SettingsState>) {
    egui::Window::new("Settings").show(&contexts.ctx_mut(), |ui| {
        if ui.checkbox(&mut ui_state.music, "Music").clicked() {};
        if ui
            .checkbox(&mut ui_state.unchecked, "Unchecked mode")
            .clicked()
        {};
        if ui.button("Reset infos").clicked() {}
    });
}
