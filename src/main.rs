use bevy::app::{FixedUpdate, Update};
use bevy::asset::{AssetApp, Assets, AssetServer, Handle, LoadState};
use bevy::log::info;
use bevy::prelude::{
    App, DefaultPlugins, ImagePlugin, in_state, IntoSystemConfigs, NextState, OnEnter, OnExit,
    PluginGroup, Query, Res, ResMut, States, Window, WindowPlugin,
};
use bevy::window::WindowResolution;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tracy_client::Client;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::anime::anime_res::{
    AnimationAssets, EnvironmentEntityAnimationAssets, PlayerEntityAnimationAssets,
};
use yakuzaishi::anime::character_anime_sys::{
    animate_env_entity_animations, animate_overlay_animations,
    attach_animations_to_player_entities, insert_overlay_animation_resources_into_world,
};
use yakuzaishi::anime::map_anime_sys::{
    animate_overlapped_tiles_event_based, handle_overlap_event, TileAnimationEvent,
};
use yakuzaishi::audio::audio_res::AudioAssets;
use yakuzaishi::audio::audio_sys::start_background_audio;
use yakuzaishi::camera::camera_sys::{init_camera, track_camera};
use yakuzaishi::environment::environment_sys::spawn_environment_entity;
use yakuzaishi::map::tiled_res::{TiledLoader, TiledMap, TiledMapAssets};
use yakuzaishi::map::tiled_sys::{process_tiled_maps, spawn_tiled_map_entity};
use yakuzaishi::player::player_sys::{control_player_entity, spawn_player_entity};

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
                insert_overlay_animation_resources_into_world,
                start_background_audio,
                spawn_tiled_map_entity, // TODO: I don't like the name of this because its spawning an asset dependant thing which i feel like should be called Load
                spawn_player_entity,
                spawn_environment_entity,
                init_camera,
                transition_to_run_state, // TODO: even though transition_to_run_state might execute before spawn_tiled_map_entity completes, the state change to GameState::Run (and the application of Commands) won't happen until after all OnEnter(GameState::AssetProcessing) systems have run.
            ),
        )
        .add_systems(
            OnExit(GameState::AssetProcessing),
            (process_tiled_maps, attach_animations_to_player_entities),
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
                // TODO: When I have the overlay animations on after like several environment entity animation loop cycles the sprite breaks
                //animate_overlay_animations.run_if(in_state(GameState::Run)),
                animate_env_entity_animations.run_if(in_state(GameState::Run)),
            ),
        )
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    AssetProcessing,
    Load,
    Run,
}

// TODO: this is whack, i dont like it
pub fn transition_to_run_state(mut next_state: ResMut<NextState<GameState>>) {
    info!("Transitioning to GameState::Run");
    next_state.set(GameState::Run);
}
