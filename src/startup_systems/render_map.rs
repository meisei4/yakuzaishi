use bevy::prelude::{AssetServer, Bundle, Commands, GlobalTransform, Handle, Res, Transform};
use bevy_ecs_tilemap::prelude::TilemapRenderSettings;

use crate::OCEAN_MAP_FILE_PATH;
use crate::startup_systems::process_tiled_maps::TiledMap;

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub render_settings: TilemapRenderSettings,
}

pub fn render_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load(OCEAN_MAP_FILE_PATH);

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
