use bevy::core::Name;
use bevy::prelude::{AssetServer, Bundle, Commands, Handle, Res};

use crate::OCEAN_MAP_FILE_PATH;
use crate::resources::tiled_resources::TiledMap;

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
}

pub fn spawn_tiled_map_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load(OCEAN_MAP_FILE_PATH);

    //TODO: If I dont spawn something here, then nothing shows up. But I still need to decouple.
    // I dont like the word spawn, i just want to insert a map resource into the gameworld to be honest
    commands
        .spawn(TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        })
        .insert(Name::new("TiledMap Bundle Entity"));
}
