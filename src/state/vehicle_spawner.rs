use amethyst::core::math::Vector2;
use amethyst::{core::Transform, ecs::prelude::*, renderer::SpriteRender};
use log::info;
use rand::{seq::SliceRandom, thread_rng};

use crate::util::{create_sprite_render, create_transform};
use crate::{
    components::vehicle_components::VehicleComponents,
    resources::{game_map_resource::GameMapResource, vehicle_resource::VehicleResource},
    TILE_SIZE,
};

pub fn spawn_vehicle(world: &mut World) {
    let game_map = world.read_resource::<GameMapResource>();
    let vehicle_sprite_sheet = world.read_resource::<VehicleResource>();

    let transforms = &mut world.write_storage::<Transform>();
    let sprite_renders = &mut world.write_storage::<SpriteRender>();
    let vehicle_components = &mut world.write_storage::<VehicleComponents>();

    let drivable_tiles = get_drivable_tiles(&game_map);
    if let Some(spawn_position) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        // TODO this is where we convert tile coordinates to world coordinates, but there has to be a more clear way to handle this tile <-> cartesian stuff
        let world_x = spawn_position.x * TILE_SIZE;
        let world_y = spawn_position.y * TILE_SIZE;

        let transform = create_transform(world_x, world_y);
        let sprite_render = create_sprite_render(0, &vehicle_sprite_sheet.sprite_sheet_handle);

        world
            .entities()
            .build_entity()
            .with(sprite_render, sprite_renders)
            .with(transform, transforms)
            .with(VehicleComponents::new(world_x, world_y), vehicle_components)
            .build();
        info!("Vehicle spawned at position: {:?}", spawn_position);
    }
}

fn get_drivable_tiles(game_map: &GameMapResource) -> Vec<Vector2<f32>> {
    game_map
        .tile_components
        .iter()
        .filter_map(|((x, y), tile_component)| {
            if tile_component.is_drivable {
                // Assuming x and y are &u32, they're dereferenced and converted to f32 here
                Some(Vector2::new(*x as f32, *y as f32))
            } else {
                None
            }
        })
        .collect()
}

fn select_random_tile_from_list_of_tiles(tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !tiles.is_empty() {
        let mut rng = thread_rng();
        tiles.choose(&mut rng).copied()
    } else {
        None
    }
}
