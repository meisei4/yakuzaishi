use bevy::asset::Assets;
use bevy::core::Name;
use bevy::log::info;
use bevy::prelude::{Bundle, Commands, Entity, Handle, Query, Res};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_tilemap::map::{
    TilemapGridSize, TilemapId, TilemapSize, TilemapSpacing, TilemapTileSize, TilemapType,
};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage};
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::tiles::{TileBundle, TileTextureIndex};
use tiled::{LayerType, TileLayer};

use crate::{
    TILE_ANIMATION_SPEED, TILE_ANIMATION_TEXTURE_END_IDX, TILE_ANIMATION_TEXTURE_START_IDX,
    TILE_SIZE,
};
use crate::anime::anime_component::{AnimationComponent, AnimationTimer};
use crate::anime::anime_res::TileAnimationResource;
use crate::map::tiled_components::TileEntityTag;
use crate::map::tiled_res::{TiledMap, TiledMapAssets};

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
}

pub fn spawn_tiled_map_entity(mut commands: Commands, tiled_asset: Res<TiledMapAssets>) {
    let map_handle: Handle<TiledMap> = tiled_asset.tiled_map.clone();
    commands
        .spawn(TiledMapBundle {
            tiled_map: map_handle,
        })
        .insert(Name::new("TiledMap Bundle Entity"));

    //TODO: look into how to better take advantage of tiled bevy crate for
    // parsing the animation data in the .tmx asset file
    let animation = AnimationComponent {
        start_idx: TILE_ANIMATION_TEXTURE_START_IDX,
        end_idx: TILE_ANIMATION_TEXTURE_END_IDX,
        speed: TILE_ANIMATION_SPEED,
    };
    let animation_data = TileAnimationResource { animation };
    commands.insert_resource(animation_data);
}

//TODO: this is alot from the custom solution from bevy_ecs_tiled, so this needs to be looked at later

pub fn process_tiled_maps(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMap>>, //TODO: learn about all the plural Assets (including TextureAtlasLayouts etc)
    animation_data: Res<TileAnimationResource>,
    mut map_query: Query<&Handle<TiledMap>>, // TODO: why is this an &
) {
    info!("process_tiled_maps: Starting");
    if let Some(map_handle) = map_query.iter_mut().next() {
        if let Some(tiled_map) = map_assets.get(map_handle) {
            for tileset_index in 0..tiled_map.map.tilesets().len() {
                process_tileset(&mut commands, tiled_map, tileset_index, &animation_data);
            }
        }
    }
    info!("process_tiled_maps: ENDING");
}

fn process_tileset(
    commands: &mut Commands,
    tiled_map: &TiledMap,
    tileset_index: usize,
    animation_data: &TileAnimationResource,
) {
    let tileset = &tiled_map.map.tilesets()[tileset_index];
    let tilemap_texture = &tiled_map.tilemap_textures[&tileset_index];

    let tile_spacing = TilemapSpacing {
        x: tileset.spacing as f32,
        y: tileset.spacing as f32,
    };

    for layer in tiled_map.map.layers() {
        let layer_entity = commands.spawn_empty().id();
        if let LayerType::Tiles(tile_layer) = layer.layer_type() {
            let map_size = TilemapSize {
                x: tiled_map.map.width,
                y: tiled_map.map.height,
            };
            let grid_size = TilemapGridSize {
                x: tiled_map.map.tile_width as f32,
                y: tiled_map.map.tile_height as f32,
            };
            let tile_storage = process_tile_layer(
                commands,
                tile_layer,
                map_size,
                TilemapId(layer_entity),
                animation_data,
            );

            commands
                .entity(layer_entity)
                .insert(TilemapBundle {
                    grid_size,
                    size: map_size,
                    storage: tile_storage,
                    texture: tilemap_texture.clone(),
                    tile_size: TilemapTileSize::new(TILE_SIZE, TILE_SIZE),
                    spacing: tile_spacing,
                    map_type: TilemapType::Square,
                    ..Default::default()
                })
                .insert(Name::new("TiledMap Tiles Entity"));
        } else {
            log::info!(
                "Skipping layer {} because only tile layers are supported.",
                layer.id()
            );
        }
    }
}

fn process_tile_layer(
    commands: &mut Commands,
    tile_layer: TileLayer,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    animation_data: &TileAnimationResource,
) -> TileStorage {
    let TileLayer::Finite(layer_data) = tile_layer else {
        log::info!("Skipping layer because only finite layers are supported.");
        return TileStorage::empty(map_size);
    };

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            if let Some(layer_tile) = layer_data.get_tile(x as i32, y as i32) {
                let texture_index = layer_tile.id();
                let tile_pos = TilePos { x, y };
                let tile_entity = create_tile_entity(
                    commands,
                    tile_pos,
                    tilemap_id,
                    texture_index,
                    animation_data,
                );
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    }

    tile_storage
}

fn create_tile_entity(
    commands: &mut Commands,
    tile_pos: TilePos,
    tilemap_id: TilemapId,
    texture_index: u32,
    animation_data: &TileAnimationResource,
) -> Entity {
    //TODO: this seems to be making redundant/duplicate entities just to have AnimatedTiles?
    let mut entity_builder = commands.spawn(TileBundle {
        position: tile_pos,
        tilemap_id,
        texture_index: TileTextureIndex(texture_index),
        // TODO: there may be some logic regarding flipping here that needs to be done, otherwise
        //  figure out how to get TOPLEFT coordinates to work
        ..Default::default()
    });

    if texture_index == animation_data.animation.start_idx {
        entity_builder
            .insert(animation_data.animation.clone())
            .insert(AnimationTimer(Timer::from_seconds(
                animation_data.animation.speed,
                TimerMode::Repeating,
            )))
            .insert(TileEntityTag)
            .insert(Name::new("TileAnimation"));
    }
    entity_builder.id()
}
