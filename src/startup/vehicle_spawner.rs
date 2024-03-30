use std::collections::HashMap;

use bevy::asset::Assets;
use bevy::prelude::{AssetServer, Commands, Res, ResMut, Sprite, SpriteSheetBundle, TextureAtlasLayout, Transform, Vec2};
use bevy::sprite::TextureAtlas;
use bevy::utils::petgraph::visit::Walker;
use rand::{seq::SliceRandom, thread_rng};

use crate::{components::vehicle_components::VehicleComponents, TILE_SIZE, TILESET_TEXTURE_FILE_PATH};
use crate::components::game_map_tile_components::GameMapTileComponents;

pub fn spawn_vehicle(command_buffer: &mut Commands,
                     asset_server: &Res<AssetServer>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
                     texture_atlas_layout: TextureAtlasLayout,
                     tile_components: HashMap<(u32, u32), GameMapTileComponents>,
) {
    let drivable_tiles = get_drivable_tiles(tile_components);
    if let Some(tile_coordinates) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        let world_spawn_coordinates = Vec2::new(
            (tile_coordinates.x + 0.5) * TILE_SIZE, // Adjust for the center
            (tile_coordinates.y + 0.5) * TILE_SIZE, // Adjust for the center
        );
        queue_vehicle_spawn_command(command_buffer, asset_server, &mut texture_atlas_layouts, texture_atlas_layout, world_spawn_coordinates);
    }
}

fn queue_vehicle_spawn_command(command_buffer: &mut Commands,
                               asset_server: &Res<AssetServer>,
                               texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
                               texture_atlas_layout: TextureAtlasLayout,
                               spawn_position: Vec2,
) {
    let transform = Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0);
    let vehicle_components = VehicleComponents::new(spawn_position.x, spawn_position.y);

    log::info!("Spawning entity for vehicle at ({}, {})", transform.local_x(), transform.local_y());

    let layout = texture_atlas_layouts.add(texture_atlas_layout);
    command_buffer.spawn(())
        .insert(SpriteSheetBundle {
            texture: asset_server.load(TILESET_TEXTURE_FILE_PATH),
            atlas: TextureAtlas { layout, index: 0 },
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
