use crate::util::is_drivable_tile;
use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, U1, U2},
        Transform,
    },
    ecs::{prelude::*, storage::MaskedStorage},
    renderer::SpriteRender,
    shred::{Fetch, FetchMut},
};
use log::info;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tiled::{FiniteTileLayer, Map};

use crate::util::{create_sprite_render, create_transform};
use crate::{
    components::vehicle_components::VehicleComponents,
    resources::{game_map_resource::GameMapResource, vehicle_resource::VehicleResource},
    TILE_SIZE,
};

pub fn spawn_vehicle(world: &mut World) {
    let game_map: Fetch<'_, GameMapResource> = world.read_resource::<GameMapResource>();
    let vehicle_sprite_sheet: Fetch<'_, VehicleResource> = world.read_resource::<VehicleResource>();

    let transforms: &mut Storage<'_, Transform, FetchMut<'_, MaskedStorage<Transform>>> =
        &mut world.write_storage::<Transform>();
    let sprite_renders: &mut Storage<'_, SpriteRender, FetchMut<'_, MaskedStorage<SpriteRender>>> =
        &mut world.write_storage::<SpriteRender>();
    let vehicle_components: &mut Storage<
        '_,
        VehicleComponents,
        FetchMut<'_, MaskedStorage<VehicleComponents>>,
    > = &mut world.write_storage::<VehicleComponents>();

    let drivable_tiles = get_drivable_tiles(&game_map);
    if let Some(spawn_position) = select_random_tile_from_list_of_tiles(&drivable_tiles) {
        // TODO this is where we convert tile coordinates to world coordinates, but there has to be a more clear way to handle this tile <-> cartesian stuff
        let world_x: f32 = spawn_position.x * TILE_SIZE + TILE_SIZE / 2.0;
        let world_y: f32 = spawn_position.y * TILE_SIZE + TILE_SIZE / 2.0;

        let transform: Transform = create_transform(world_x, world_y);
        let sprite_render: SpriteRender =
            create_sprite_render(0, &vehicle_sprite_sheet.sprite_sheet_handle);

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
                Some(Vector2::new(x, y))
            } else {
                None
            }
        })
        .collect()
}

fn select_random_tile_from_list_of_tiles(tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !tiles.is_empty() {
        let mut rng: ThreadRng = thread_rng();
        tiles.choose(&mut rng).copied()
    } else {
        None
    }
}
