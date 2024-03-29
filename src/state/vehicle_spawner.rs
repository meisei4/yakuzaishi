use bevy::prelude::{Commands, Sprite, SpriteSheetBundle, Transform, Vec2};
use rand::{seq::SliceRandom, thread_rng};

use crate::{components::vehicle_components::VehicleComponents, resources::game_map_resource::GameMapResource, resources::vehicle_resource::VehicleResource, TILE_SIZE};

pub fn spawn_vehicle(vehicle_resource: &VehicleResource, game_map: &GameMapResource, command_buffer: &mut Commands) {
    let drivable_tiles = get_drivable_tiles(game_map);
    if let Some(tile_coordinates) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        let world_spawn_coordinates = Vec2::new(
            (tile_coordinates.x + 0.5) * TILE_SIZE, // Adjust for the center
            (tile_coordinates.y + 0.5) * TILE_SIZE, // Adjust for the center
        );
        queue_vehicle_spawn_command(vehicle_resource, command_buffer, world_spawn_coordinates);
    }
}

fn queue_vehicle_spawn_command(vehicle_resource: &VehicleResource, command_buffer: &mut Commands, spawn_position: Vec2) {
    let transform = Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0);
    let vehicle_components = VehicleComponents::new(spawn_position.x, spawn_position.y);

    let sprite = Sprite::new(0);
    log::info!("Spawning entity for vehicle at ({}, {}) with sprite ID {}", transform.local_x(), transform.local_y(), sprite.id());
    command_buffer.spawn(())
        .insert(SpriteSheetBundle {
            sprite,
            atlas: &vehicle_resource.sprite_sheet_handle,
            transform,
            ..Default::default()
        })
        .insert(vehicle_components);
}

fn get_drivable_tiles(game_map: &GameMapResource) -> Vec<Vec2> {
    let drivable_tiles = game_map
        .tile_components
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
