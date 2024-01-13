use amethyst::core::math::Vector2;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::SpriteRender;
use log::info;
use rand::{seq::SliceRandom, thread_rng};

use crate::resources::vehicle_resource::VehicleResource;
use crate::util::create_transform;
use crate::{
    components::vehicle_components::VehicleComponents,
    resources::game_map_resource::GameMapResource, util,
};

pub fn spawn_vehicle(world: &mut World) {
    let drivable_tiles = get_drivable_tiles(world);
    if let Some(spawn_position) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        spawn_vehicle_at_position(world, spawn_position);
    }
}

fn spawn_vehicle_at_position(world: &mut World, spawn_position: Vector2<f32>) {
    let transform = create_transform(spawn_position.x, spawn_position.y);
    let sprite_render = create_sprite_render(world, 0);

    let transforms = &mut world.write_storage::<Transform>();
    let sprite_renders = &mut world.write_storage::<SpriteRender>();
    let vehicle_components = &mut world.write_storage::<VehicleComponents>();

    world
        .entities()
        .build_entity()
        .with(sprite_render, sprite_renders)
        .with(transform, transforms)
        .with(
            VehicleComponents::new(spawn_position.x, spawn_position.y),
            vehicle_components,
        )
        .build();

    info!("Vehicle spawned at position: {:?}", spawn_position);
}

// Add other helper functions as needed...

fn get_drivable_tiles(world: &World) -> Vec<Vector2<f32>> {
    let game_map = world.read_resource::<GameMapResource>();
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

fn create_sprite_render(world: &World, sprite_index: usize) -> SpriteRender {
    let vehicle_sprite_sheet = world.read_resource::<VehicleResource>();
    util::create_sprite_render(sprite_index, &vehicle_sprite_sheet.sprite_sheet_handle)
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
