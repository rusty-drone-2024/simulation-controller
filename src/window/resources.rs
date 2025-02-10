use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct MainUiState {
    pub new_pdr: Option<String>,
    pub nbhg_1: Option<String>,
    pub nbhg_2: Option<String>,
}

#[derive(Resource, Debug)]
pub struct SelectedUiState {
    pub pdr: Option<String>,
    pub node_to_add: Option<String>,
    pub node_to_rmv: Option<String>,
}
