use bevy::prelude::*;

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