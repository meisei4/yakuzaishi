use amethyst::core::math::Vector2;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::vehicle_components::VehicleComponents;
use crate::resources::game_map_resource::GameMapResource;
use crate::resources::system_active_flag::SystemActive;
use crate::resources::vehicle_resource::VehicleResource;
use crate::TILE_SIZE;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, VehicleComponents>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, GameMapResource>,
        ReadExpect<'s, VehicleResource>,
        Read<'s, SystemActive>,
    );

    fn run(&mut self, (mut vehicles, transforms, game_map, vehicle_resource, system_active): Self::SystemData) {
        if !system_active.is_active {
            return;
        }
        for (vehicle, transform) in (&mut vehicles, &transforms).join() {
            let has_collision_occured = check_future_collision(vehicle, transform, &game_map, &vehicle_resource);
            if has_collision_occured {
                handle_collision(vehicle);
            }
        }
    }
}

// TODO: Currently this does not actually cause 100% stop, because the vehicle_controller_system
//  still allows a split second of motion/displacement before it checks for collision again.
fn handle_collision(vehicle_component: &mut VehicleComponents) {
    vehicle_component.base.speed = 0.0;
}

fn check_future_collision(
    vehicle: &VehicleComponents,
    transform: &Transform,
    game_map: &GameMapResource,
    vehicle_resource: &VehicleResource,
) -> bool {
    let current_hitbox = &vehicle_resource.hitboxes[vehicle.current_hitbox_index];

    // Calculate the future position of the vehicle based on its current direction and speed
    let future_position = Vector2::new(
        transform.translation().x + vehicle.direction.x * vehicle.base.speed,
        transform.translation().y + vehicle.direction.y * vehicle.base.speed,
    );

    // Adjust the hitbox corners and midpoints based on the future position of the vehicle
    let adjusted_corners = current_hitbox.corners.map(|corner| {
        Vector2::new(corner.x + future_position.x, corner.y + future_position.y)
    });
    let adjusted_midpoints = current_hitbox.midpoints.map(|midpoint| {
        Vector2::new(midpoint.x + future_position.x, midpoint.y + future_position.y)
    });

    // Combine corners and midpoints for collision checking
    let collision_points = [adjusted_corners, adjusted_midpoints].concat();

    // Check all relevant points for drivability
    for point in collision_points.iter() {
        let tile_x = (point.x / TILE_SIZE).floor() as u32;
        let tile_y = (point.y / TILE_SIZE).floor() as u32;

        if let Some(tile) = game_map.tile_components.get(&(tile_x, tile_y)) {
            if !tile.is_drivable {
                log::info!("Collision Detected at Vehicle Point: {:?}, with Tile: ({}, {}), TileType: {:?}", point, tile_x, tile_y, tile.tile_type);
                return true;
            }
        }
    }
    false // No collision detected
}



