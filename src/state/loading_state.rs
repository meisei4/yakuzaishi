use bevy::asset::{Assets, AssetServer};
use bevy::prelude::{ColorMaterial, Commands, Res, ResMut, State};

use crate::{VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH};
use crate::state::bevy_game_state::GameState;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load(VEHICLE_TEXTURE_FILE_PATH);
    let sprite_sheet_handle = asset_server.load(VEHICLE_SPRITE_SHEET_FILE_PATH);

    // Example of how you might track asset loading progress.
    // Bevy doesn't use a progress counter like Amethyst; instead, you check if assets are ready.
    //TODO use check assets ready? for generating hitbox from the textureatlas thing


    commands.insert_resource(Some(texture_handle.clone()));

    // Transition to the main state could be based on an event or an asset loading check in another system.
    // For simplicity and direct translation, we'll assume immediate transition for demonstration purposes.
    state.set(GameState::Main).unwrap();
}

fn check_assets_ready(
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    texture_handle: Res<Option<HandleUntyped>>,
) {
    if let Some(handle) = texture_handle.as_ref() {
        if asset_server.get_load_state(handle.id) == LoadState::Loaded {
            // All assets are ready, transition to the main game state
            state.set(GameState::Main).unwrap();
        }
    }
}
