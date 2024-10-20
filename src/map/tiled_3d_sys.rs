use bevy::{
    color::Color,
    log::info,
    math::{Vec2, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{Commands, Plane3d, Res, ResMut, Transform},
    utils::default,
};
use bevy_asset::{Assets, Handle};
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_render::{alpha::AlphaMode, mesh::Mesh};
use rand::Rng;
use tiled::LayerType;

use crate::{
    map::tiled_res::{TiledMapAssets, TiledMapSource},
    TILE_SIZE,
};

pub fn spawn_tiled_map_3d(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMapSource>>,
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

    let map_handle: Handle<TiledMapSource> = tiled_asset.tiled_map.clone();

    if let Some(tiled_map) = map_assets.get(&map_handle) {
        for tileset_index in 0..tiled_map.rs_tiled_map.tilesets().len() {
            process_tileset_3d(
                &mut commands,
                tiled_map,
                tileset_index,
                &mut meshes,
                &mut materials,
            );
        }
    }
}

fn process_tileset_3d(
    commands: &mut Commands,
    tiled_map: &TiledMapSource,
    _tileset_index: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //TODO: figure out how to get the individual images from the tilemap
    //  for now just use a random color
    //let tileset = &tiled_map.rs_tiled_map.tilesets()[tileset_index];
    //let tilemap_texture = &tiled_map.bevy_ecs_tilemap_textures;

    for layer in tiled_map.rs_tiled_map.layers() {
        // TODO: this is not yet ready, still need to fix how the tileset gets recognized as individual textures inside
        if let LayerType::Tiles(_tile_layer) = layer.layer_type() {
            process_tile_layer(commands, materials, meshes);
        } else {
            info!(
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
            let tile_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(TILE_SIZE, TILE_SIZE)));

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
