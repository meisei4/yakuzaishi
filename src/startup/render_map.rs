use bevy::prelude::{AssetServer, Commands, Handle, Res};

use crate::helpers_hack::tiled_hack::{TiledMap, TiledMapBundle};
use crate::MAP_FILE_PATH_ASSET;

pub fn render_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load(MAP_FILE_PATH_ASSET);

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
