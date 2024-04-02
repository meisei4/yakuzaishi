use bevy::prelude::{App, AssetApp, Plugin, Update};
use crate::helpers_hack::tiled_hack::{process_loaded_maps, TiledMap};
use crate::helpers_hack::tiled_loader::TiledLoader;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TiledMap>()
            .register_asset_loader(TiledLoader)
            .add_systems(Update, process_loaded_maps);
    }
}