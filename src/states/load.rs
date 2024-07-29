use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, AssetServer, Handle, LoadState};
use bevy::prelude::{IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut};

use crate::resources::animation_resources::OverlayAnimationData;
use crate::states::state_enums::GameState;
use crate::systems::load_state::animation::setup_map_animation_data::{
    attach_animations_to_map, setup_map_animation_data,
};
use crate::systems::load_state::animation::setup_overlay_animation_data::{
    attach_overlay_animations_to_flying_entities, load_and_setup_overlay_animation_data,
};
use crate::systems::load_state::initialize_camera::init_camera;
use crate::systems::load_state::load_map;
use crate::systems::load_state::process_tiled_maps::{process_tiled_maps, TiledMap};
use crate::systems::load_state::spawn_flying_entity::spawn_vehicle;

pub struct LoadStatePlugin;

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Load),
            (load_map::load_map, load_and_setup_overlay_animation_data),
        )
        .add_systems(Update, check_assets_loaded)
        .add_systems(
            OnExit(GameState::Load),
            (
                //cleanup_check_assets_loaded,
                process_tiled_maps,
                setup_map_animation_data.after(process_tiled_maps),
                attach_animations_to_map.after(setup_map_animation_data),
                spawn_vehicle.after(setup_map_animation_data),
                attach_overlay_animations_to_flying_entities.after(spawn_vehicle),
                init_camera.after(spawn_vehicle),
            ),
        );
    }
}

fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    map_assets: Res<Assets<TiledMap>>,
    map_query: Query<&Handle<TiledMap>>,
    overlay_animation_data: Option<Res<OverlayAnimationData>>,
) {
    if let Some(map_handle) = map_query.iter().next() {
        if asset_server.get_load_state(map_handle.id()) == Some(LoadState::Loaded) {
            if map_assets.get(map_handle).is_some() {
                if let Some(data) = overlay_animation_data {
                    if asset_server.get_load_state(data.wake_texture_handle.id())
                        == Some(LoadState::Loaded)
                    {
                        next_state.set(GameState::Run);
                        // TODO: somehow this system still keeps running when moved into Run state??
                        // info!(
                        //     "All assets and animation data loaded, transitioning to GameState::Run"
                        // );
                        return;
                    }
                }
            }
        }
    }
}
//
// fn cleanup_check_assets_loaded(mut commands: Commands) {
//     commands.remove_system(check_assets_loaded);
// }
