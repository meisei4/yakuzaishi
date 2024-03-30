use std::collections::HashMap;
use std::fs;
use std::path::Path;

use bevy::asset::ron;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Assets, AssetServer, Commands, Res, ResMut, Resource, TextureAtlasLayout};
use serde::Deserialize;
use tiled::{FiniteTileLayer, LayerTile, Loader, Map, TileLayer, Tileset};

use crate::{enums::entity_type::EntityType, MAP_FILE_PATH, TILESET_FILE_PATH, VEHICLE_SPRITE_SHEET_FILE_PATH};
use crate::components::game_map_tile_components::{GameMapTileComponents, TileType};

use super::{camera_initializer, game_map_renderer, vehicle_spawner};

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

#[derive(Resource, Deserialize)]
pub struct Yakuzaishi {
    pub entity_type: EntityType,
}

impl Yakuzaishi {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type,
        }
    }

    // assets:
    pub fn init_game_state(
        &mut self,
        command_buffer: &mut Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let mut tiled_loader = Loader::new();

        let tile_set = tiled_loader.load_tsx_tileset(Path::new(TILESET_FILE_PATH)).expect("Failed to load tileset");

        let map_texture_atlas_layout = create_texture_atlas(&tile_set);

        let tiled_map = tiled_loader.load_tmx_map(Path::new(MAP_FILE_PATH)).expect("Failed to load tilemap");

        let tile_components = build_tile_components(&tiled_map);

        game_map_renderer::render_map(command_buffer, &asset_server,
                                      texture_atlas_layouts.clone(), // TODO: HAHAHAH AHAH this fixes the "already moved" issue, but probably not correct solution
                                      map_texture_atlas_layout, tiled_map);

        let sprite_sheet_spec = load_sprite_sheet_spec_from_file(VEHICLE_SPRITE_SHEET_FILE_PATH);
        let vehicle_texture_atlas_layout = create_texture_atlas_vehicle(sprite_sheet_spec);

        vehicle_spawner::spawn_vehicle(command_buffer, &asset_server, texture_atlas_layouts, vehicle_texture_atlas_layout, tile_components);
        camera_initializer::init_camera(command_buffer);
    }
}


///MAP STUFF

fn create_texture_atlas(
    tile_set: &Tileset,
) -> TextureAtlasLayout {
    let columns = tile_set.columns;
    let rows = (tile_set.tilecount as f32 / columns as f32).ceil() as usize;

    let mut texture_atlas_layout = TextureAtlasLayout::new_empty(Vec2 { x: columns as f32, y: rows as f32 });
    for y in 0..rows {
        for x in 0..columns {
            let sprite_rect = Rect {
                min: Vec2::new(x as f32 * tile_set.tile_width as f32, y as f32 * tile_set.tile_height as f32),
                max: Vec2::new((x + 1) as f32 * tile_set.tile_width as f32, (y + 1) as f32 * tile_set.tile_height as f32),
            };
            texture_atlas_layout.add_texture(sprite_rect);
        }
    }
    texture_atlas_layout
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

impl Default for Yakuzaishi {
    fn default() -> Self {
        Self::new(EntityType::Vehicle)
    }
}




