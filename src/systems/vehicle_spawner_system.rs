use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, U1, U2},
        Transform,
    },
    ecs::{prelude::*, Entities},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};
use log::info;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tiled::{FiniteTileLayer, Map};

use crate::{
    components::vehicle_components::VehicleComponents,
    resources::{game_map_resource::GameMapResource, vehicle_resource::VehicleResource},
    TILE_SIZE,
};

pub struct VehicleSpawnerSystem {
    vehicle_spawned: bool,
}

impl VehicleSpawnerSystem {
    pub fn new() -> Self {
        VehicleSpawnerSystem {
            vehicle_spawned: false,
        }
    }
}

impl<'s> System<'s> for VehicleSpawnerSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, GameMapResource>,
        ReadExpect<'s, VehicleResource>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, VehicleComponents>,
    );

    fn run(
        &mut self,
        (entities, game_map, vehicle_sprite_sheet, mut transforms, mut sprite_renders, mut vehicle): Self::SystemData,
    ) {
        if self.vehicle_spawned {
            return;
        }
        let drivable_tiles: Vec<Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>>> =
            get_drivable_tiles(&game_map.tiled_map);
        if let Some(spawn_position) = select_random_tile(&drivable_tiles) {
            spawn_vehicle(
                &entities,
                &vehicle_sprite_sheet.sprite_sheet_handle,
                &mut transforms,
                &mut sprite_renders,
                &mut vehicle,
                spawn_position,
            );
            self.vehicle_spawned = true;
            info!("Vehicle spawned at position: {:?}", spawn_position);
        }
    }
}

fn get_drivable_tiles(tiled_map: &Map) -> Vec<Vector2<f32>> {
    let mut drivable_tiles: Vec<Vector2<f32>> = Vec::new();

    //TODO: this still has to loop through layers,
    for layer in tiled_map.layers() {
        match layer.layer_type() {
            tiled::LayerType::Tiles(layer) => match layer {
                tiled::TileLayer::Finite(tile_layer) => {
                    fill_up_drivable_tiles(&tile_layer, &mut drivable_tiles);
                }
                tiled::TileLayer::Infinite(_) => todo!(),
            },
            tiled::LayerType::Objects(_) => todo!(),
            tiled::LayerType::Image(_) => todo!(),
            tiled::LayerType::Group(_) => todo!(),
        }
    }
    drivable_tiles
}

fn fill_up_drivable_tiles(tiles: &FiniteTileLayer, drivable_tiles: &mut Vec<Vector2<f32>>) {
    for y in 0..tiles.height() {
        for x in 0..tiles.width() {
            if let Some(tile) = tiles.get_tile(x as i32, y as i32) {
                //TODO: figure out how to retrieve the metadata in the tsx file
                if tile.id() != 17 {
                    drivable_tiles.push(Vector2::new(x as f32, y as f32));
                }
            }
        }
    }
}

fn select_random_tile(tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !tiles.is_empty() {
        let mut rng: ThreadRng = thread_rng();
        tiles.choose(&mut rng).copied()
    } else {
        None
    }
}

fn spawn_vehicle(
    entities: &Entities,
    sprite_sheet_handle: &SpriteSheetHandle,
    transforms: &mut WriteStorage<Transform>,
    sprite_renders: &mut WriteStorage<SpriteRender>,
    vehicle: &mut WriteStorage<VehicleComponents>,
    spawn_position: Vector2<f32>,
) {
    // TODO this is where we convert tile coordinates to world coordinates, but there has to be a more clear way to handle this tile <-> cartesian stuff
    let world_x: f32 = spawn_position.x * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y: f32 = spawn_position.y * TILE_SIZE + TILE_SIZE / 2.0;

    let transform: Transform = create_transform_for_sprite(world_x, world_y);
    let sprite_render: SpriteRender = create_sprite_render_for_vehicle(sprite_sheet_handle);

    entities
        .build_entity()
        .with(sprite_render, sprite_renders)
        .with(transform, transforms)
        .with(VehicleComponents::new(world_x, world_y), vehicle)
        .build();
}

//TODO same as the map rendering system, copy pasted code, fix it.
fn create_transform_for_sprite(x: f32, y: f32) -> Transform {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(x * TILE_SIZE, y * TILE_SIZE, 0.0);
    transform
}

// Adapted from `create_sprite_render_for_tile` from map_rendering_system
fn create_sprite_render_for_vehicle(sprite_sheet_handle: &SpriteSheetHandle) -> SpriteRender {
    // TODO this is pretty much identical to the map_rendering_system.rs helper method, but the vehicle sprite sheet only has 1 sprite..
    SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    }
}
