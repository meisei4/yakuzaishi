use amethyst::core::math::Vector2;
use log::info;
use rand::{seq::SliceRandom, thread_rng};

use crate::{components::vehicle_components::VehicleComponents, resources::game_map_resource::GameMapResource, TILE_SIZE, util};
use crate::command_buffer::command_buffer::{CommandBuffer, EntityCreationCommand};
use crate::resources::vehicle_resource::VehicleResource;
use crate::util::create_transform;

pub fn spawn_vehicle(vehicle_sprite_sheet: &VehicleResource, game_map: &GameMapResource, command_buffer: &mut CommandBuffer) {
    let drivable_tiles = get_drivable_tiles(game_map);
    if let Some(tile_coordinates) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        // TODO figure out when and how to formalize this x,y tile-coordinates vs x,y real 2D space coordinates issue
        //  here is the location where the issue of the:
        //  "tiles-coordinates" -> "world-coordinates" is introduced (perhaps not where it should be solved though)
        let world_spawn_coordinates = Vector2::new(
            (tile_coordinates.x as f32 + 0.5) * TILE_SIZE, // Adjust for the center
            (tile_coordinates.y as f32 + 0.5) * TILE_SIZE, // Adjust for the center
        );
        spawn_vehicle_at_position(vehicle_sprite_sheet, command_buffer, world_spawn_coordinates);
    }
}

fn spawn_vehicle_at_position(vehicle_sprite_sheet: &VehicleResource, command_buffer: &mut CommandBuffer, spawn_position: Vector2<f32>) {
    let transform = create_transform(spawn_position.x, spawn_position.y);
    let sprite_render = util::create_sprite_render(0, &vehicle_sprite_sheet.sprite_sheet_handle);
    let vehicle_components = VehicleComponents::new(spawn_position.x, spawn_position.y);

    let spawn_command = EntityCreationCommand::new()
        .with_transform(transform)
        .with_sprite_render(sprite_render)
        .with_vehicle_component(vehicle_components);

    command_buffer.add_command(spawn_command);
}

// Add other helper functions as needed...
fn get_drivable_tiles(game_map: &GameMapResource) -> Vec<Vector2<f32>> {
    let drivable_tiles = game_map
        .tile_components
        .iter()

        .filter_map(|((x, y), tile_component)| {
            if tile_component.is_drivable {
                Some(Vector2::new(*x as f32, *y as f32))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    drivable_tiles
}

fn select_random_tile_from_list_of_tiles(tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !tiles.is_empty() {
        let mut rng = thread_rng();
        let selected_tile = tiles.choose(&mut rng).copied();
        selected_tile
    } else {
        info!("No drivable tiles available for vehicle spawning");
        None
    }
}
