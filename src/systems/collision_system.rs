use amethyst::core::math::Vector2;
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::game_map_tile_components::TileType;
use crate::components::vehicle_components::VehicleComponents;
use crate::resources::game_map_resource::GameMapResource;
use crate::TILE_SIZE;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, VehicleComponents>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, GameMapResource>,
    );

    fn run(&mut self, (mut vehicles, transforms, game_map): Self::SystemData) {
        for (vehicle, transform) in (&mut vehicles, &transforms).join() {
            let (collision, tile_type) = check_future_collision(vehicle, transform, &game_map);
            if collision {
                handle_collision(vehicle);
            } else {
                adjust_speed_for_tile(vehicle, tile_type);
            }
        }
    }
}

// TODO: Currently this does not actually cause 100% stop, because the vehicle_controller_system
//  still allows a split second of motion/displacement before it checks for collision again.
fn handle_collision(vehicle_component: &mut VehicleComponents) {
    vehicle_component.base.speed = 0.0;
}

fn adjust_speed_for_tile(vehicle_component: &mut VehicleComponents, tile_type: TileType) {
    match tile_type {
        TileType::Grass => vehicle_component.base.speed *= 0.5,
        TileType::Wall => vehicle_component.base.speed = 0.0,
        _ => {}
    }
}

fn check_future_collision(vehicle: &VehicleComponents, transform: &Transform, game_map: &GameMapResource) -> (bool, TileType) {
    let next_x = transform.translation().x + vehicle.direction.x * vehicle.base.speed;
    let next_y = transform.translation().y + vehicle.direction.y * vehicle.base.speed;

    let next_position = Vector2::new(next_x, next_y);

    let corner_positions = calculate_vehicle_corners(next_position, vehicle.size);

    // Check all relevant tiles for drivability
    for corner_pos in corner_positions {
        let tile_x = (corner_pos.x / TILE_SIZE).floor() as u32;
        let tile_y = (corner_pos.y / TILE_SIZE).floor() as u32;

        if let Some(tile) = game_map.tile_components.get(&(tile_x, tile_y)) {
            if !tile.is_drivable {
                log::info!("Collision Detected:");
                log::info!("Vehicle Speed: {}, Direction: {:?}", vehicle.base.speed, vehicle.direction);
                log::info!("Collision at Vehicle Corner: {:?}, Tile: ({}, {}), TileType: {:?}", corner_pos, tile_x, tile_y, tile.tile_type);
                return (true, tile.tile_type);
            }
        }
    }

    (false, TileType::Normal)
}

fn calculate_vehicle_corners(position: Vector2<f32>, dimensions: Vector2<f32>) -> Vec<Vector2<f32>> {
    let half_width = dimensions.x / 2.0;
    let half_height = dimensions.y / 2.0;

    vec![
        Vector2::new(position.x - half_width, position.y - half_height), // Bottom left
        Vector2::new(position.x + half_width, position.y - half_height), // Bottom right
        Vector2::new(position.x - half_width, position.y + half_height), // Top left
        Vector2::new(position.x + half_width, position.y + half_height), // Top right
    ]
}


