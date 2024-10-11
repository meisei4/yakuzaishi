use bevy::app::{FixedUpdate, Update};
use bevy::asset::AssetApp;
use bevy::log::info;
use bevy::prelude::{
    App, AppExtStates, DefaultPlugins, ImagePlugin, in_state, IntoSystemConfigs, NextState,
    OnEnter, PluginGroup, ResMut, States, Window, WindowPlugin,
};
use bevy::sprite::Material2dPlugin;
use bevy::window::WindowResolution;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_ecs_tilemap::prelude::MaterialTilemapPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tracy_client::Client;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::anime::anime_res::{
    EnvironmentEntityAnimationAssets, OverlayAnimationAssets, PlayerEntityAnimationAssets,
};
use yakuzaishi::anime::environment_anime_sys::animate_env_entity_animations;
use yakuzaishi::anime::map_anime_sys::{
    animate_overlapped_tiles_event_based, handle_overlap_event, TileAnimationEvent,
};
use yakuzaishi::anime::overlay_anime_sys::{
    animate_overlay_animations, attach_overlay_animation_to_player_entity,
};
use yakuzaishi::audio::audio_res::AudioAssets;
use yakuzaishi::audio::audio_sys::start_background_audio;
use yakuzaishi::camera::camera_2d_sys::{bottom_camera, init_camera, top_camera, track_camera};
use yakuzaishi::environment::environment_sys::spawn_environment_entity;
use yakuzaishi::environment::moon::{MoonAsset, place_moon};
use yakuzaishi::map::tiled_res::{TiledLoader, TiledMapAssets, TiledMapSource};
use yakuzaishi::map::tiled_sys::{spawn_tiled_map, update_time_on_shader};
use yakuzaishi::materials::fog::FogMaterial;
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
                            NINTENDO_DS_SCREEN_WIDTH,
                            NINTENDO_DS_SCREEN_HEIGHT,
                        ),
                        resizable: false,
                        title: "Yakuzaishi".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins((
            WorldInspectorPlugin::new(),
            TilemapPlugin,
            MaterialTilemapPlugin::<FogMaterial>::default(),
        ))
        .init_asset::<TiledMapSource>()
        .register_asset_loader(TiledLoader)
        .add_event::<TileAnimationEvent>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetProcessing)
                .load_collection::<AudioAssets>()
                .load_collection::<TiledMapAssets>()
                .load_collection::<PlayerEntityAnimationAssets>()
                .load_collection::<OverlayAnimationAssets>()
                .load_collection::<EnvironmentEntityAnimationAssets>()
                .load_collection::<MoonAsset>(),
        )
        .add_systems(
            OnEnter(GameState::AssetProcessing),
            (
                //start_background_audio,
                // TODO: this does not work at all yet, still learning how 3D meshes and materials works
                //spawn_tiled_map_3d,
                spawn_tiled_map,
                spawn_player_entity,
                spawn_environment_entity,
                place_moon,
                top_camera,
                bottom_camera,
                // TODO: even though transition_to_run_state might execute before
                //  spawn_tiled_map_entity completes, the state change to GameState::Run
                //  (and the application of Commands) won't happen until after all
                //  OnEnter(GameState::AssetProcessing) systems have run.
                transition_to_run_state,
            ),
        )
        .add_systems(
            OnEnter(GameState::Run),
            attach_overlay_animation_to_player_entity,
        )
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
                // TODO: sometimes when I have the overlay animations on after like several
                //  environment entity animation loop cycles the sprite breaks
                animate_overlay_animations.run_if(in_state(GameState::Run)),
                animate_env_entity_animations.run_if(in_state(GameState::Run)),
                update_time_on_shader.run_if(in_state(GameState::Run)),
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
