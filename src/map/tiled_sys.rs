use bevy::color::{Color, Srgba};
use bevy::color::Color::LinearRgba;
use bevy::core::Name;
use bevy::log::info;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Commands, Entity, Query, Res, ResMut, Transform};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy_asset::{Assets, AssetServer, Handle};
use bevy_ecs_tilemap::{MaterialTilemapBundle, TilemapBundle};
use bevy_ecs_tilemap::map::{
    TilemapGridSize, TilemapId, TilemapSize, TilemapSpacing, TilemapTileSize, TilemapType,
};
use bevy_ecs_tilemap::prelude::{get_tilemap_center_transform, TilePos, TileStorage};
use bevy_ecs_tilemap::tiles::{TileBundle, TileTextureIndex};
use bevy_render::texture::Image;
use tiled::{LayerType, TileLayer};

use crate::{
    TILE_ANIMATION_SPEED, TILE_ANIMATION_TEXTURE_END_IDX, TILE_ANIMATION_TEXTURE_START_IDX,
    TILE_SIZE,
};
use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::map::fog_material::FogMaterial;
use crate::map::tiled_components::TileEntityTag;
use crate::map::tiled_res::{TiledMap, TiledMapAssets};

pub fn spawn_tiled_map(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMap>>,
    tiled_asset: Res<TiledMapAssets>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    info!("process_tiled_maps: Starting");
    let map_handle: Handle<TiledMap> = tiled_asset.tiled_map.clone();

    if let Some(tiled_map) = map_assets.get(&map_handle) {
        for tileset_index in 0..tiled_map.map.tilesets().len() {
            process_tileset(&mut commands, tiled_map, tileset_index, &mut materials);
        }
    }
    info!("process_tiled_maps: ENDING");
}

fn process_tileset(
    commands: &mut Commands,
    tiled_map: &TiledMap,
    tileset_index: usize,
    materials: &mut Assets<FogMaterial>,
) {
    let tileset = &tiled_map.map.tilesets()[tileset_index];
    let tilemap_texture = &tiled_map.tilemap_textures[&tileset_index];

    let fog_material_handle = materials.add(FogMaterial {
        time: 0.0,
        density: 0.5,
        fog_color: Vec3::new(0.8, 0.8, 0.8),
        ..default()
    });

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
            let tile_storage =
                process_tile_layer(commands, tile_layer, map_size, TilemapId(layer_entity));

            commands
                .entity(layer_entity)
                .insert(MaterialTilemapBundle {
                    grid_size,
                    size: map_size,
                    storage: tile_storage,
                    texture: tilemap_texture.clone(),
                    tile_size: TilemapTileSize::new(TILE_SIZE, TILE_SIZE),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    spacing: tile_spacing,
                    material: fog_material_handle.clone(),
                    ..Default::default()
                })
                .insert(Name::new("TiledMap With Fog Entity"));
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
                let tile_entity = create_tile_entity(commands, tile_pos, tilemap_id, texture_index);
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
) -> Entity {
    let mut entity_builder = commands.spawn(TileBundle {
        position: tile_pos,
        tilemap_id,
        texture_index: TileTextureIndex(texture_index),
        // TODO: there may be some logic regarding flipping here that needs to be done, otherwise
        //  figure out how to get TOPLEFT coordinates to work
        ..Default::default()
    });

    if texture_index == TILE_ANIMATION_TEXTURE_START_IDX {
        entity_builder
            .insert(AnimationComponent {
                start_idx: TILE_ANIMATION_TEXTURE_START_IDX,
                end_idx: TILE_ANIMATION_TEXTURE_END_IDX,
                speed: TILE_ANIMATION_SPEED,
            })
            .insert(AnimationTimer(Timer::from_seconds(
                TILE_ANIMATION_SPEED,
                TimerMode::Repeating,
            )))
            .insert(Name::new("AnimatedTile"));
    }
    entity_builder.insert(TileEntityTag);
    entity_builder.id()
}

// SHADER STUFF:

pub fn update_time_on_shader(time: Res<Time>, mut materials: ResMut<Assets<FogMaterial>>) {
    for (_, material) in materials.iter_mut() {
        material.time += time.delta_seconds();
    }
}
