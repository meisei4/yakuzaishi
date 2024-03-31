use bevy::prelude::{AssetServer, Camera2dBundle, Commands, Handle, Res};

use crate::{helpers, MAP_FILE_PATH_ASSET};

pub fn render_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load(MAP_FILE_PATH_ASSET);

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}