use bevy::app::{FixedUpdate, Update};
use bevy::asset::{AssetApp, Assets, AssetServer, Handle, LoadState};
use bevy::log::info;
use bevy::prelude::{
    App, DefaultPlugins, ImagePlugin, in_state, IntoSystemConfigs, NextState, OnEnter, OnExit,
    PluginGroup, Query, Res, ResMut, Window, WindowPlugin,
};
use bevy::window::WindowResolution;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tracy_client::Client;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::resources::animation::{
    AnimationAssets, AnimationResource, EnvironmentEntityAnimationAssets,
    PlayerEntityAnimationAssets,
};
use yakuzaishi::resources::audio::AudioAssets;
use yakuzaishi::resources::tiled::{TiledLoader, TiledMap, TiledMapAssets};
use yakuzaishi::states::GameState;
use yakuzaishi::systems::animation_loadtime::attach_animations::{
    attach_animations_to_environment_entities, attach_animations_to_individual_tile_entities,
    attach_animations_to_player_entities, attach_base_textures_to_player_entities,
};
use yakuzaishi::systems::animation_loadtime::insert_animations::{
    insert_overlay_animation_resources_into_world, insert_tile_animation_resources_into_world,
};
use yakuzaishi::systems::animation_runtime::overlay_animations::animate_env_entity_animations;
use yakuzaishi::systems::animation_runtime::tile_animations::{
    animate_overlapped_tiles_event_based, handle_overlap_event, TileAnimationEvent,
};
use yakuzaishi::systems::audio_runtime::background_music::start_background_audio;
use yakuzaishi::systems::entity_loadtime::camera::init_camera;
use yakuzaishi::systems::entity_loadtime::environment_entity::spawn_environment_entity;
use yakuzaishi::systems::entity_loadtime::player_entity::spawn_player_entity;
use yakuzaishi::systems::entity_loadtime::tiled_map::{process_tiled_maps, spawn_tiled_map_entity};
use yakuzaishi::systems::entity_runtime::camera::track_camera;
use yakuzaishi::systems::entity_runtime::player_entity::control_player_entity;

fn main() {
    let _tracy_client = Client::start();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            NINTENDO_DS_SCREEN_WIDTH * 1.5,
                            NINTENDO_DS_SCREEN_HEIGHT * 1.5,
                        ),
                        resizable: false,
                        title: "Yakuzaishi".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TilemapPlugin)
        .init_asset::<TiledMap>()
        .register_asset_loader(TiledLoader)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetProcessing)
                .load_collection::<AudioAssets>()
                .load_collection::<TiledMapAssets>()
                .load_collection::<PlayerEntityAnimationAssets>()
                .load_collection::<AnimationAssets>()
                .load_collection::<EnvironmentEntityAnimationAssets>(),
        )
        .add_systems(
            OnEnter(GameState::AssetProcessing),
            (
                start_background_audio,
                spawn_tiled_map_entity, // TODO: I don't like the name of this because its spawning an asset dependant thing which i feel like should be called Load
                insert_tile_animation_resources_into_world,
                insert_overlay_animation_resources_into_world,
                spawn_player_entity,
                spawn_environment_entity,
                init_camera,
                transition_to_run_state, // TODO: even though transition_to_run_state might execute before spawn_tiled_map_entity completes, the state change to GameState::Run (and the application of Commands) won't happen until after all OnEnter(GameState::AssetProcessing) systems have run.
            ),
        )
        .add_systems(
            OnExit(GameState::AssetProcessing),
            (
                process_tiled_maps,
                attach_animations_to_individual_tile_entities,
                attach_base_textures_to_player_entities,
                attach_animations_to_player_entities, // TODO: REQUIRED OnExit
                attach_animations_to_environment_entities, // TODO: REQUIRED OnExit
            ),
        )
        .add_event::<TileAnimationEvent>()
        .add_systems(
            FixedUpdate,
            control_player_entity.run_if(in_state(GameState::Run)),
        )
        .add_systems(
            Update,
            (
                track_camera.run_if(in_state(GameState::Run)),
                animate_overlapped_tiles_event_based.run_if(in_state(GameState::Run)),
                handle_overlap_event.run_if(in_state(GameState::Run)),
                // TODO: BIG TODO!!! this next line actually doesnt make sense
                //  animate_env_entity_animations causes the player entity overlay to work and
                //  in fact the animate_overlay_animations breaks the original animations...
                // animate_overlay_animations.run_if(in_state(GameState::Run)),
                animate_env_entity_animations.run_if(in_state(GameState::Run)),
            ),
        )
        .run();
}

// TODO: this is whack, i dont like it
pub fn transition_to_run_state(mut next_state: ResMut<NextState<GameState>>) {
    info!("Transitioning to GameState::Run");
    next_state.set(GameState::Run);
}
