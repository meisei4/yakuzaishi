use std::collections::HashMap;
use std::fs;
use std::path::Path;

use bevy::asset::ron;
use bevy::prelude::{Assets, AssetServer, Commands, Handle, Rect, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, Vec2};
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use tiled::{FiniteTileLayer, LayerTile, Loader, Map, TileLayer};

use crate::{components::vehicle_components::VehicleComponents, MAP_FILE_PATH, TILE_SIZE, VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH};
use crate::components::game_map_tile_components::{GameMapTileComponents, TileType};

#[derive(Deserialize)]
struct SpriteSheetSpec {
    texture_width: u32,
    texture_height: u32,
    sprites: Vec<SpriteSpec>,
}

#[derive(Deserialize)]
struct SpriteSpec {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offsets: (f32, f32),
}

pub fn spawn_vehicle(command_buffer: Commands,
                     asset_server: Res<AssetServer>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut tiled_loader = Loader::new();

    let tiled_map = tiled_loader.load_tmx_map(Path::new(MAP_FILE_PATH)).expect("Failed to load tilemap");

    let tile_components = build_tile_components(&tiled_map);

    let sprite_sheet_spec = load_sprite_sheet_spec_from_file(VEHICLE_SPRITE_SHEET_FILE_PATH);

    let vehicle_texture_atlas_layout = create_texture_atlas_vehicle(sprite_sheet_spec);

    let drivable_tiles = get_drivable_tiles(tile_components);

    if let Some(tile_coordinates) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        let world_spawn_coordinates = Vec2::new(
            (tile_coordinates.x + 0.5) * TILE_SIZE, // Adjust for the center
            (tile_coordinates.y + 0.5) * TILE_SIZE, // Adjust for the center
        );

        let texture_atlas_layout_handle = texture_atlas_layouts.add(vehicle_texture_atlas_layout);
        queue_vehicle_spawn_command(command_buffer, asset_server, texture_atlas_layout_handle, world_spawn_coordinates);
    }
}

fn queue_vehicle_spawn_command(mut command_buffer: Commands,
                               asset_server: Res<AssetServer>,
                               texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
                               spawn_position: Vec2,
) {
    let transform = Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0);
    let vehicle_components = VehicleComponents::new(spawn_position.x, spawn_position.y);

    command_buffer.spawn(())
        .insert(SpriteSheetBundle {
            texture: asset_server.load(VEHICLE_TEXTURE_FILE_PATH),
            atlas: TextureAtlas { layout: texture_atlas_layout_handle, index: 0 },
            transform,
            ..Default::default()
        })
        .insert(vehicle_components);
}

fn get_drivable_tiles(tile_components: HashMap<(u32, u32), GameMapTileComponents>) -> Vec<Vec2> {
    let drivable_tiles = tile_components
        .iter()
        .filter_map(|((x, y), tile_component)| {
            if tile_component.is_drivable {
                Some(Vec2::new(*x as f32, *y as f32))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    drivable_tiles
}

fn select_random_tile_from_list_of_tiles(tiles: &[Vec2]) -> Option<Vec2> {
    if !tiles.is_empty() {
        let mut rng = thread_rng();
        let selected_tile = tiles.choose(&mut rng).copied();
        selected_tile
    } else {
        log::info!("No drivable tiles available for vehicle spawning");
        None
    }
}

fn build_tile_components(tiled_map: &Map) -> HashMap<(u32, u32), GameMapTileComponents> {
    let mut tile_components = HashMap::new();
    for layer in tiled_map.layers() {
        if let Some(TileLayer::Finite(finite_layer)) = layer.as_tile_layer() {
            let layer_components = process_layer(finite_layer);
            tile_components.extend(layer_components);
        }
    }
    tile_components
}

fn process_layer(finite_layer: FiniteTileLayer) -> HashMap<(u32, u32), GameMapTileComponents> {
    let mut components = HashMap::new();
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                components.insert((x, y), GameMapTileComponents::new(is_drivable_tile(tile), TileType::Normal));
            }
        }
    }
    components
}

fn is_drivable_tile(tile: LayerTile) -> bool {
    tile.id() != 17
}


///VEHICLE STUFF

fn load_sprite_sheet_spec_from_file(file_path: &str) -> SpriteSheetSpec {
    let ron_data = fs::read_to_string(file_path).expect("Failed to read RON file");
    ron::from_str(&ron_data).expect("Failed to deserialize RON data")
}

fn create_texture_atlas_vehicle(
    sprite_sheet_spec: SpriteSheetSpec,
) -> TextureAtlasLayout {
    let mut texture_atlas_layout = TextureAtlasLayout::new_empty(Vec2 { x: sprite_sheet_spec.texture_width as f32, y: sprite_sheet_spec.texture_height as f32 });
    for sprite in &sprite_sheet_spec.sprites {
        let rect = Rect {
            min: Vec2::new(sprite.x as f32, sprite.y as f32),
            max: Vec2::new((sprite.x + sprite.width) as f32, (sprite.y + sprite.height) as f32),
        };
        texture_atlas_layout.add_texture(rect);
    }
    texture_atlas_layout
}
