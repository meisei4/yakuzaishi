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
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::util::{create_sprite_render, create_transform};
use crate::{
    components::pedestrian_components::PedestrianComponents,
    resources::pedestrian_resource::PedestrianResource, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE,
};

pub fn spawn_pedestrian(world: &mut World) {
    let pedestrian_sprite_sheet: Fetch<'_, PedestrianResource> =
        world.read_resource::<PedestrianResource>();
    let transforms: &mut Storage<'_, Transform, FetchMut<'_, MaskedStorage<Transform>>> =
        &mut world.write_storage::<Transform>();
    let sprite_renders: &mut Storage<'_, SpriteRender, FetchMut<'_, MaskedStorage<SpriteRender>>> =
        &mut world.write_storage::<SpriteRender>();
    let pedestrian_components: &mut Storage<
        '_,
        PedestrianComponents,
        FetchMut<'_, MaskedStorage<PedestrianComponents>>,
    > = &mut world.write_storage::<PedestrianComponents>();

    let spawn_position: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> =
        select_random_tile_from_full_map();
    let world_x: f32 = spawn_position.x * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y: f32 = spawn_position.y * TILE_SIZE + TILE_SIZE / 2.0;

    let transform: Transform = create_transform(world_x, world_y);
    let sprite_render: SpriteRender =
        create_sprite_render(0, &pedestrian_sprite_sheet.sprite_sheet_handle);

    world
        .entities()
        .build_entity()
        .with(sprite_render, sprite_renders)
        .with(transform, transforms)
        .with(
            PedestrianComponents::new(world_x, world_y),
            pedestrian_components,
        )
        .build();
    info!("pedestrian spawned at position: {:?}", spawn_position);
}

fn select_random_tile_from_full_map() -> Vector2<f32> {
    let mut rng: ThreadRng = thread_rng();
    let x: f32 = rng.gen_range(0.0..MAP_WIDTH);
    let y: f32 = rng.gen_range(0.0..MAP_HEIGHT);
    Vector2::new(x, y)
}
