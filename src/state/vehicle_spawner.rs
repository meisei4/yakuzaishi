use amethyst::core::math::Vector2;
use log::info;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    components::vehicle_components::VehicleComponents,
    resources::game_map_resource::GameMapResource, util,
};
use crate::command_buffer::command_buffer::{CommandBuffer, EntityCreationCommand};
use crate::resources::vehicle_resource::VehicleResource;
use crate::util::create_transform;

pub fn spawn_vehicle(vehicle_sprite_sheet: &VehicleResource, game_map: &GameMapResource, command_buffer: &mut CommandBuffer) {
    let drivable_tiles = get_drivable_tiles(game_map);
    if let Some(spawn_position) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        spawn_vehicle_at_position(vehicle_sprite_sheet, command_buffer, spawn_position);
    }
}

fn spawn_vehicle_at_position(vehicle_sprite_sheet: &VehicleResource, command_buffer: &mut CommandBuffer, spawn_position: Vector2<f32>) {
    let transform = create_transform(spawn_position.x, spawn_position.y);
    let sprite_render = util::create_sprite_render(0, &vehicle_sprite_sheet.sprite_sheet_handle);
    let vehicle_components = VehicleComponents::new(spawn_position.x, spawn_position.y);

    let spawn_command = EntityCreationCommand::new()
        .with_transform(transform)
        .with_sprite_render(sprite_render)
        .with_vehicle_component(vehicle_components); // Assuming `vehicle_components` is of type VehicleComponents

    command_buffer.add_command(spawn_command);
    info!("Vehicle spawn command queued for position: {:?}", spawn_position);
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

    info!("Found {} drivable tiles", drivable_tiles.len());
    drivable_tiles
}

fn select_random_tile_from_list_of_tiles(tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !tiles.is_empty() {
        let mut rng = thread_rng();
        let selected_tile = tiles.choose(&mut rng).copied();
        if let Some(tile) = selected_tile {
            info!("Selected random drivable tile at position: {:?}", tile);
        }
        selected_tile
    } else {
        info!("No drivable tiles available for vehicle spawning");
        None
    }
}
