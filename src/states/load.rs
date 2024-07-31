use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, AssetServer, Handle, LoadState};
use bevy::prelude::{IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut};

use crate::resources::animation_resources::OverlayAnimationResource;
use crate::resources::tiled_resources::TiledMap;
use crate::states::state_enums::GameState;
use crate::systems::load_state::animation_asset_prep::overlay_animation_prep::{
    attach_overlay_animations_to_controllable_entities,
    insert_overlay_animation_resources_into_gameworld,
};
use crate::systems::load_state::animation_asset_prep::tile_animation_prep::{
    attach_animations_to_individual_tile_entities, insert_tile_animation_resources_into_gameworld,
};
use crate::systems::load_state::init_camera::init_camera;
use crate::systems::load_state::process_tiled_maps::process_tiled_maps;
use crate::systems::load_state::spawn_controllable_entity::spawn_controllable_entity;
use crate::systems::load_state::spawn_tiled_map_entity::spawn_tiled_map_entity;

pub struct LoadStatePlugin;

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Load),
            (
                spawn_tiled_map_entity,
                insert_overlay_animation_resources_into_gameworld,
                insert_tile_animation_resources_into_gameworld,
            ),
        )
        .add_systems(Update, check_assets_loaded)
        .add_systems(
            OnExit(GameState::Load),
            (
                process_tiled_maps,
                insert_tile_animation_resources_into_gameworld.after(process_tiled_maps),
                attach_animations_to_individual_tile_entities
                    .after(insert_tile_animation_resources_into_gameworld),
                spawn_controllable_entity.after(insert_tile_animation_resources_into_gameworld),
                // COMMENT THIS OUT IF YOU WANT TO TURN OFF PLAYER ENTITY SPRITE
                // attach_sprite_to_flying_entity.after(spawn_vehicle),
                attach_overlay_animations_to_controllable_entities.after(spawn_controllable_entity),
                init_camera.after(spawn_controllable_entity),
            ),
        );
    }
}

fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    map_assets: Res<Assets<TiledMap>>,
    map_query: Query<&Handle<TiledMap>>,
    overlay_animation_data: Option<Res<OverlayAnimationResource>>,
) {
    if let Some(map_handle) = map_query.iter().next() {
        if asset_server.get_load_state(map_handle.id()) == Some(LoadState::Loaded) {
            if map_assets.get(map_handle).is_some() {
                if let Some(data) = overlay_animation_data {
                    if asset_server.get_load_state(data.overlay_animation_image_handle.id())
                        == Some(LoadState::Loaded)
                    {
                        next_state.set(GameState::Run);
                        // TODO: somehow this system still keeps running when moved into Run state??
                        // info!(
                        //     "All assets and animation_asset_prep data loaded, transitioning to GameState::Run"
                        // );
                        return;
                    }
                }
            }
        }
    }
}
