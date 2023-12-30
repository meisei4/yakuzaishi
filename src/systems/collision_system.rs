use crate::components::vehicle_components::VehicleComponents;
use crate::resources::game_map_resource::GameMapResource;
use crate::util::is_drivable_tile;
use crate::TILE_SIZE;
use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        Transform,
    },
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, VehicleComponents>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, GameMapResource>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut vehicles, transforms, game_map, time): Self::SystemData) {
        let mut collision_updates: Vec<(amethyst::ecs::Entity, Vector2<f32>, f32)> = Vec::new();

        for (entity, vehicle, transform) in (&entities, &mut vehicles, &transforms).join() {
            let movement = Vector3::new(vehicle.direction.x, vehicle.direction.y, 0.0)
                * vehicle.base.speed
                * time.delta_seconds();
            let next_position = transform.translation() + movement;
            let tile_x = (next_position.x / TILE_SIZE).floor() as usize;
            let tile_y = (next_position.y / TILE_SIZE).floor() as usize;

            // Use the tile_components to check for drivability
            let is_drivable = game_map
                .tile_components
                .get(&(tile_x as u32, tile_y as u32))
                .map_or(false, |tile| tile.is_drivable);

            if !is_drivable {
                // Manually negate the x and y components of the direction vector
                let new_direction = Vector2::new(-vehicle.direction.x, -vehicle.direction.y);
                let new_speed = vehicle.base.speed * 0.5;
                collision_updates.push((entity, new_direction, new_speed));
            }
        }

        for (entity, new_direction, new_speed) in collision_updates {
            if let Some(vehicle) = vehicles.get_mut(entity) {
                vehicle.direction = new_direction;
                vehicle.base.speed = new_speed;
            }
        }

        for vehicle in (&mut vehicles).join() {
            if vehicle.base.speed > 0.0 {
                vehicle.base.speed -= vehicle.deceleration * time.delta_seconds();
                if vehicle.base.speed < 0.0 {
                    vehicle.base.speed = 0.0;
                }
            }
        }
    }
}
