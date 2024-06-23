use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, AssetServer, Handle, LoadState};
use bevy::prelude::{IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut};

use crate::states::state_enums::GameState;
use crate::systems::load_state::{
    initialize_camera, load_animations, load_map, process_tiled_maps, spawn_flying_entity,
};
use crate::systems::load_state::process_tiled_maps::TiledMap;

pub struct LoadStatePlugin;

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Load), load_map::load_map)
            .add_systems(Update, check_assets_loaded)
            .add_systems(
                OnExit(GameState::Load),
                (
                    process_tiled_maps::process_tiled_maps,
                    load_animations::setup_map_animation_data
                        .after(process_tiled_maps::process_tiled_maps),
                    load_animations::attach_animations_to_map
                        .after(load_animations::setup_map_animation_data),
                    spawn_flying_entity::spawn_vehicle
                        .after(load_animations::setup_map_animation_data),
                    initialize_camera::init_camera.after(spawn_flying_entity::spawn_vehicle),
                ),
            );
    }
}

fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    map_assets: Res<Assets<TiledMap>>,
    map_query: Query<&Handle<TiledMap>>,
) {
    if let Some(map_handle) = map_query.iter().next() {
        if asset_server.get_load_state(map_handle.id()) == Some(LoadState::Loaded) {
            if map_assets.get(map_handle).is_some() {
                next_state.set(GameState::Run);
            }
        }
    }
}
