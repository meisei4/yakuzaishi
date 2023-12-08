use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, U1, U2},
        Transform,
    },
    ecs::{prelude::*, Entities},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};
use log::info;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    components::pedestrian_components::PedestrianComponents,
    resources::pedestrian_resource::PedestrianResource, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE,
};

pub struct PedestrianSpawnerSystem {
    pedestrian_spawned: bool,
}

impl PedestrianSpawnerSystem {
    pub fn new() -> Self {
        PedestrianSpawnerSystem {
            pedestrian_spawned: false,
        }
    }
}

impl<'s> System<'s> for PedestrianSpawnerSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, PedestrianResource>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, PedestrianComponents>,
    );

    fn run(
        &mut self,
        (
            entities,
            pedestrian_sprite_sheet,
            mut transforms,
            mut sprite_renders,
            mut pedestrian_components,
        ): Self::SystemData,
    ) {
        if self.pedestrian_spawned {
            return;
        }
        let spawn_position: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> = select_random_tile(); // No longer needs to be passed a slice of tiles.
        spawn_pedestrian(
            &entities,
            &pedestrian_sprite_sheet.sprite_sheet_handle,
            &mut transforms,
            &mut sprite_renders,
            &mut pedestrian_components,
            spawn_position,
        );
        self.pedestrian_spawned = true;
        info!("pedestrian spawned at position: {:?}", spawn_position);
    }
}

fn select_random_tile() -> Vector2<f32> {
    let mut rng: ThreadRng = thread_rng();
    let x: f32 = rng.gen_range(0.0..MAP_WIDTH);
    let y: f32 = rng.gen_range(0.0..MAP_HEIGHT);
    Vector2::new(x, y)
}

fn spawn_pedestrian(
    entities: &Entities,
    sprite_sheet_handle: &SpriteSheetHandle,
    transforms: &mut WriteStorage<Transform>,
    sprite_renders: &mut WriteStorage<SpriteRender>,
    pedestrian_components: &mut WriteStorage<PedestrianComponents>,
    spawn_position: Vector2<f32>,
) {
    let world_x: f32 = spawn_position.x * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y: f32 = spawn_position.y * TILE_SIZE + TILE_SIZE / 2.0;

    let transform: Transform = create_transform_for_sprite(world_x, world_y);
    let sprite_render: SpriteRender = create_sprite_render_for_pedestrian(sprite_sheet_handle);

    entities
        .build_entity()
        .with(sprite_render, sprite_renders)
        .with(transform, transforms)
        .with(
            PedestrianComponents::new(world_x, world_y),
            pedestrian_components,
        )
        .build();
}

//TODO same as the map rendering system, copy pasted code, fix it.
fn create_transform_for_sprite(x: f32, y: f32) -> Transform {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(x * TILE_SIZE, y * TILE_SIZE, 0.0);
    transform
}

// Adapted from `create_sprite_render_for_tile` from map_rendering_system
fn create_sprite_render_for_pedestrian(sprite_sheet_handle: &SpriteSheetHandle) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    }
}
