use bevy::asset::AssetApp;
use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, Window, WindowPlugin};
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tracy_client::Client;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::resources::tiled::{TiledLoader, TiledMap};
use yakuzaishi::states::{GameState, LoadStatePlugin, RunStatePlugin};

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
        //.init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TilemapPlugin)
        .init_asset::<TiledMap>()
        .register_asset_loader(TiledLoader)
        // .add_loading_state(
        //     LoadingState::new(GameState::AudioAssetLoad)
        //         .continue_to_state(GameState::Load)
        //         .load_collection::<AudioAssets>(),
        // )
        //.add_systems(FixedUpdate, control_player_entity)
        // .add_systems(
        //     OnEnter(GameState::Run),
        //     (
        //         track_camera,
        //         animate_overlapped_tiles_event_based,
        //         handle_overlap_event,
        //         animate_env_entity_animations,
        //         start_background_audio,
        //     ),
        // )
        .add_plugins(LoadStatePlugin)
        .add_plugins(RunStatePlugin)
        .insert_state(GameState::Load)
        .run();
}
