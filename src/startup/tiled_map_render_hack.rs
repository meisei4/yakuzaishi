use bevy::prelude::{AssetServer, Commands, Handle, Res};

use crate::{helpers, MAP_FILE_PATH_ASSET};

pub fn render_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::tiled_hack::TiledMap> = asset_server.load(MAP_FILE_PATH_ASSET);

    commands.spawn(helpers::tiled_hack::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}