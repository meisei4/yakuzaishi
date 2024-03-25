use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::SystemDesc;
use crate::components::game_map_tile_components::TileType;
use crate::components::vehicle_components::VehicleComponents;
use crate::resources::game_map_resource::GameMapResource;
use crate::TILE_SIZE;

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, VehicleComponents>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, GameMapResource>,
    );

    fn run(&mut self, (mut vehicles, transforms, game_map): Self::SystemData) {
        for (vehicle, transform) in (&mut vehicles, &transforms).join() {
            let (collision, tile_type) = check_collision_and_tile_type(transform, &game_map);
            if collision {
                handle_collision(vehicle);
            } else {
                adjust_speed_for_tile(vehicle, tile_type);
            }
        }
    }
}

fn check_collision_and_tile_type(transform: &Transform, game_map: &GameMapResource) -> (bool, TileType) {
    let position = transform.translation();
    let tile_x = (position.x / TILE_SIZE).floor() as u32;
    let tile_y = (position.y / TILE_SIZE).floor() as u32;

    match game_map.tile_components.get(&(tile_x, tile_y)) {
        Some(tile) => (tile.is_drivable, tile.tile_type),
        None => (false, TileType::Normal), // Assuming 'Normal' is a default non-collidable tile type
    }
}

fn handle_collision(vehicle_component: &mut VehicleComponents) {
    vehicle_component.base.speed = 0.0; // Stop the vehicle upon collision
}

fn adjust_speed_for_tile(vehicle_component: &mut VehicleComponents, tile_type: TileType) {
    match tile_type {
        TileType::Grass => vehicle_component.base.speed *= 0.5, // Example: halve speed on grass
        TileType::Wall => vehicle_component.base.speed = 0.0, // Stop on wall, redundant here due to collision handling but illustrative
        _ => {}, // No adjustment for normal tiles
    }
}
