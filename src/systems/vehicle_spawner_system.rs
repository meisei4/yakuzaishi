use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, Vector3, U1, U3},
        Transform,
    },
    ecs::{prelude::*, Entities},
    renderer::SpriteRender,
};
use log::info;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tiled::{FiniteTileLayer, Map};

use crate::resources::{
    game_map_resource::GameMap, 
    vehicle_sprite_sheet::VehicleSpriteSheet
};
use crate::components::vehicle_component::Vehicle;

const TILE_SIZE: f32 = 64.0;
const VEHICLE_SIZE: f32 = 32.0;

pub struct VehicleSpawnerSystem;

impl<'s> System<'s> for VehicleSpawnerSystem {
    
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, GameMap>, //if only using the tiled::Map this complains, so GameMap wraps as a Resource
        ReadExpect<'s, VehicleSpriteSheet>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Vehicle>,
    );

    fn run(
        &mut self,
        (entities, 
            game_map,
            vehicle_sprite_sheet,
            mut transforms,
            mut sprite_renders,
            mut vehicles): Self::SystemData,
    ) {
        if let Some(spawn_position) = select_random_tile(&game_map.map) {
            let mut transform: Transform = Transform::default();
            //TODO: Not sure if the TILE_SIZE is needed for spawning figure out how to spawn into the middle of a tile i guess
            transform.set_translation_xyz(
                spawn_position.x * TILE_SIZE,
                spawn_position.y * TILE_SIZE,
                0.0,
            );
            let scale: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> =
                Vector3::new(VEHICLE_SIZE / TILE_SIZE, VEHICLE_SIZE / TILE_SIZE, 1.0);
            transform.set_scale(scale);

            entities
                .build_entity()
                .with(
                    SpriteRender {
                        sprite_sheet: vehicle_sprite_sheet.sprite_sheet_handle.clone(),
                        sprite_number: 0,
                    },
                    &mut sprite_renders,
                )
                .with(transform, &mut transforms)
                .with(Vehicle::new(), &mut vehicles)
                .build();

            info!("Vehicle spawned at position: {:?}", spawn_position);
        }
    }

}

fn select_random_tile(game_map: &Map) -> Option<Vector2<f32>> {
    let mut drivable_tiles: Vec<Vector2<f32>> = Vec::new();

    //TODO: this still has to loop through layers,
    for layer in game_map.layers() {
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
    select_random_tile_from_tiles(&drivable_tiles)
}

fn select_random_tile_from_tiles(drivable_tiles: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if !drivable_tiles.is_empty() {
        let mut rng: ThreadRng = thread_rng();
        drivable_tiles.choose(&mut rng).copied()
    } else {
        None
    }
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