use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::ui::event_listener::DisplayedInfo;

#[derive(Debug, Resource)]
pub struct MusicResource {
    pub entity: Option<Entity>,
    pub playing: bool,
}

#[derive(Debug, Resource)]
pub struct StateResource {
    pub unchecked: bool,
}

#[derive(Resource)]
pub struct ModeConfig {
    pub bypass_cheks: bool,
}

#[derive(Event)]
struct MusicEvent;

#[derive(Event)]
struct ModeEvent;

#[derive(Event)]
struct ResetInfosEvent;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MusicResource {
            entity: None,
            playing: true,
        });
        app.insert_resource(StateResource { unchecked: false });
        app.insert_resource(ModeConfig {
            bypass_cheks: false,
        });
        app.add_event::<MusicEvent>();
        app.add_event::<ModeEvent>();
        app.add_event::<ResetInfosEvent>();
        app.add_systems(Update, settings_window);
        app.add_systems(Update, update_music);
        app.add_systems(Update, update_unchecked);
        app.add_systems(Update, reset_infos);
    }
}

fn settings_window(
    mut contexts: EguiContexts,
    mut music_ui: ResMut<MusicResource>,
    mut state_ui: ResMut<StateResource>,
    mut ew_music: EventWriter<MusicEvent>,
    mut ew_unchecked_mode: EventWriter<ModeEvent>,
    mut ew_infos: EventWriter<ResetInfosEvent>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        if ui.checkbox(&mut music_ui.playing, "Music").clicked() {
            ew_music.send(MusicEvent);
        };
        if ui
            .checkbox(&mut state_ui.unchecked, "Unchecked mode")
            .clicked()
        {
            ew_unchecked_mode.send(ModeEvent);
        };
        if ui.button("Reset infos").clicked() {
            ew_infos.send(ResetInfosEvent);
        }
    });
}

fn update_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<MusicEvent>,
    mut music: ResMut<MusicResource>,
) {
    for _ in reader.read() {
        if music.playing {
            if music.entity.is_none() {
                let entity = commands
                    .spawn((
                        AudioPlayer::new(asset_server.load("soundtrack.mp3")),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::new(0.5),
                            ..Default::default()
                        },
                    ))
                    .id();
                music.entity = Some(entity);
            }
        } else if music.entity.is_some() {
            commands.entity(music.entity.unwrap()).despawn_recursive();
            music.entity = None;
        }
    }
}

fn update_unchecked(mut reader: EventReader<ModeEvent>, mut state: ResMut<ModeConfig>) {
    for _ in reader.read() {
        state.bypass_cheks = !state.bypass_cheks;
    }
}

fn reset_infos(mut reader: EventReader<ResetInfosEvent>, mut info: ResMut<DisplayedInfo>) {
    for _ in reader.read() {
        for (_, data) in &mut info.drone {
            data.packets_sent = 0;
            data.packets_shortcutted = 0;
            data.data_sent = 0;
            data.data_dropped = 0;
            data.faulty_packets_sent = 0;
            data.fouls = 0;
            data.neighbours.clear();
            data.latency = 0;
        }
        for (_, data) in &mut info.server {
            data.packets_sent = 0;
            data.data_sent = 0;
            data.pending_requests = 0;
            data.avg_bytes_xmessage = 0;
            data.fouls = 0;
        }
        for (_, data) in &mut info.client {
            data.packets_sent = 0;
            data.data_received = 0;
            data.pending_requests = 0;
            data.avg_bytes_xmessage = 0;
            data.fouls = 0;
        }
    }
}
