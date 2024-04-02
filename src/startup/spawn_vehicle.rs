use std::collections::HashMap;
use std::fs;
use std::path::Path;

use bevy::asset::ron;
use bevy::prelude::{
    AssetServer, Assets, Commands, Rect, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasLayout, Transform, Vec2,
};
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use tiled::{FiniteTileLayer, Loader, Map, TileLayer};

use crate::components::{
    map_tile_component::MapTileComponent, vehicle_components::VehicleComponents,
};
use crate::{
    MAP_FILE_PATH, OCEAN_MAP_FILE_PATH, TILE_SIZE, VEHICLE_SPRITE_SHEET_FILE_PATH,
    VEHICLE_TEXTURE_FILE_PATH,
};

#[derive(Deserialize)]
struct SpriteSheetSpec {
    texture_width: f32,
    texture_height: f32,
    sprites: Vec<SpriteSpec>,
}

#[derive(Deserialize)]
struct SpriteSpec {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    // offsets: (f32, f32), TODO: not sure why these exist in the sprites .ron file
}

pub fn spawn_vehicle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tiled_map = Loader::new()
        .load_tmx_map(Path::new(OCEAN_MAP_FILE_PATH))
        .expect("Failed to load tilemap");

    let sprite_sheet_spec = load_sprite_sheet_spec_from_file(VEHICLE_SPRITE_SHEET_FILE_PATH);
    let vehicle_texture_atlas_layout = create_texture_atlas(sprite_sheet_spec);
    let tile_components_map = build_tile_components(&tiled_map);

    let drivable_tiles = get_drivable_tiles_from_map(tile_components_map);

    if let Some(tile_spawn_coordinates) = select_random_drivable_tile(&drivable_tiles) {
        let texture_atlas_layout_handle = texture_atlas_layouts.add(vehicle_texture_atlas_layout);

        let world_spawn_coordinates = Vec2 {
            x: tile_spawn_coordinates.x * TILE_SIZE,
            y: tile_spawn_coordinates.y * TILE_SIZE,
        };

        let transform =
            Transform::from_xyz(world_spawn_coordinates.x, world_spawn_coordinates.y, 1.0);

        commands.spawn((
            VehicleComponents::new(tile_spawn_coordinates),
            SpriteSheetBundle {
                texture: asset_server.load(VEHICLE_TEXTURE_FILE_PATH),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index: 0,
                },
                transform,
                ..Default::default()
            },
        ));
    } else {
        log::info!("No drivable tiles available for vehicle spawning");
    }
}

fn load_sprite_sheet_spec_from_file(file_path: &str) -> SpriteSheetSpec {
    let ron_data = fs::read_to_string(file_path).expect("Failed to read RON file");
    ron::from_str(&ron_data).expect("Failed to deserialize RON data")
}

fn create_texture_atlas(sprite_sheet_spec: SpriteSheetSpec) -> TextureAtlasLayout {
    let mut texture_atlas_layout = TextureAtlasLayout::new_empty(Vec2 {
        x: sprite_sheet_spec.texture_width,
        y: sprite_sheet_spec.texture_height,
    });
    for sprite in &sprite_sheet_spec.sprites {
        let rect = Rect {
            min: Vec2 {
                x: sprite.x,
                y: sprite.y,
            },
            max: Vec2 {
                x: sprite.x + sprite.width,
                y: sprite.y + sprite.height,
            },
        };
        texture_atlas_layout.add_texture(rect);
    }
    texture_atlas_layout
}

fn get_drivable_tiles_from_map(
    tile_components: HashMap<(i32, i32), MapTileComponent>,
) -> Vec<Vec2> {
    let drivable_tiles = tile_components
        .iter()
        .filter_map(|((x, y), tile_component)| {
            if tile_component.is_drivable {
                Some(Vec2 {
                    x: *x as f32,
                    y: *y as f32,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    drivable_tiles
}

fn build_tile_components(tiled_map: &Map) -> HashMap<(i32, i32), MapTileComponent> {
    let mut tile_components = HashMap::new();
    for layer in tiled_map.layers() {
        if let Some(TileLayer::Finite(finite_layer)) = layer.as_tile_layer() {
            let layer_components = process_layer(finite_layer);
            tile_components.extend(layer_components);
        }
    }
    tile_components
}

fn process_layer(finite_layer: FiniteTileLayer) -> HashMap<(i32, i32), MapTileComponent> {
    let mut tile_components = HashMap::new();
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                tile_components.insert(
                    (x as i32, y as i32),
                    MapTileComponent {
                        is_drivable: tile.id() != 17,
                    },
                );
            }
        }
    }
    tile_components
}

fn select_random_drivable_tile(drivable_tiles: &[Vec2]) -> Option<Vec2> {
    drivable_tiles.choose(&mut thread_rng()).copied()
}
