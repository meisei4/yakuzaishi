use bevy::color::{Color, Srgba};
use bevy::color::Color::LinearRgba;
use bevy::core::Name;
use bevy::log::info;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Bundle, Commands, Entity, Plane3d, Query, Res, ResMut, Transform};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy_asset::{Assets, AssetServer, Handle};
use bevy_ecs_tilemap::{MaterialTilemapBundle, TilemapBundle};
use bevy_ecs_tilemap::map::{
    TilemapGridSize, TilemapId, TilemapSize, TilemapSpacing, TilemapTexture, TilemapTileSize,
    TilemapType,
};
use bevy_ecs_tilemap::prelude::{get_tilemap_center_transform, TilePos, TileStorage};
use bevy_ecs_tilemap::tiles::{TileBundle, TileFlip, TileTextureIndex};
use bevy_render::alpha::AlphaMode;
use bevy_render::mesh::Mesh;
use bevy_render::texture::Image;
use rand::Rng;
use tiled::{LayerType, TileLayer};

use crate::{
    TILE_ANIMATION_SPEED, TILE_ANIMATION_TEXTURE_END_IDX, TILE_ANIMATION_TEXTURE_START_IDX,
    TILE_SIZE,
};
use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::map::fog_material::FogMaterial;
use crate::map::tiled_components::TileEntityTag;
use crate::map::tiled_res::{TiledMap, TiledMapAssets};

pub fn spawn_tiled_map_3d(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMap>>,
    tiled_asset: Res<TiledMapAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the base plane test
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(
            Vec3::Y,
            Vec2::new(TILE_SIZE * 10.0, TILE_SIZE * 10.0),
        )),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.5, 0.0),
            ..default()
        }),
        ..default()
    });

    let map_handle: Handle<TiledMap> = tiled_asset.tiled_map.clone();

    if let Some(tiled_map) = map_assets.get(&map_handle) {
        for tileset_index in 0..tiled_map.map.tilesets().len() {
            process_tileset_3d(
                &mut commands,
                &tiled_map,
                tileset_index,
                &mut meshes,
                &mut materials,
            );
        }
    }
}

fn process_tileset_3d(
    commands: &mut Commands,
    tiled_map: &TiledMap,
    tileset_index: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //TODO: figure out how to get the individual images from the tilemap
    //  for now just use a random color
    let tileset = &tiled_map.map.tilesets()[tileset_index];
    let tilemap_texture = &tiled_map.tilemap_textures[&tileset_index];

    for layer in tiled_map.map.layers() {
        // TODO: this is not yet ready, still need to fix how the tileset gets recognized as individual textures inside
        if let LayerType::Tiles(tile_layer) = layer.layer_type() {
            process_tile_layer(commands, materials, meshes);
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
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let map_size = TilemapSize { x: 10, y: 10 };
    let mut rng = rand::thread_rng();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_mesh = meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::new(TILE_SIZE, TILE_SIZE).into(),
            ));

            let tile_material = materials.add(StandardMaterial {
                base_color: Color::srgb(rng.gen(), rng.gen(), rng.gen()),
                //TODO: figure out how to get the individual images from the tilemap AND OR VERTEX SHADER!!
                //  for now just use a random color
                //base_color_texture: Some(tilemap_texture.clone()),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            });

            commands.spawn(PbrBundle {
                mesh: tile_mesh.clone(),
                material: tile_material.clone(),
                transform: Transform::from_xyz(
                    x as f32 * 2.0 * TILE_SIZE,
                    0.1,
                    y as f32 * 2.0 * TILE_SIZE,
                ),
                //.with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, PI, 0.0))
                //.with_scale(Vec3::splat(10.0)),
                ..default()
            });
        }
    }
}
