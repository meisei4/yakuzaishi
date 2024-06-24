use bevy::asset::AssetApp;
use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, Window, WindowPlugin};
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::states::load::LoadStatePlugin;
use yakuzaishi::states::run::RunStatePlugin;
use yakuzaishi::states::state_enums::GameState;
use yakuzaishi::systems::load_state::process_tiled_maps::TiledMap;
use yakuzaishi::systems::load_state::tiled_loader::TiledLoader;

fn main() {
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
        .add_plugins(TilemapPlugin)
        .init_asset::<TiledMap>()
        .register_asset_loader(TiledLoader)
        .add_plugins(LoadStatePlugin)
        .add_plugins(RunStatePlugin)
        .insert_state(GameState::Load)
        .run();
}
