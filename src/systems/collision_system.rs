use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::{Entities, Join, Read, ReadExpect, System, WriteStorage},
};

use crate::{
    components::vehicle_components::VehicleComponents,
    resources::game_map_resource::GameMapResource, TILE_SIZE,
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, GameMapResource>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut vehicle_components, mut transforms, game_map, time): Self::SystemData,
    ) {
        for (_entity, vehicle_component, transform) in (&entities, &mut vehicle_components, &mut transforms).join() {
            if self.is_vehicle_colliding(transform, &game_map) {
                vehicle_component.base.speed = 0.0;
            } else {
                self.apply_movement(vehicle_component, transform, time.delta_seconds());
            }
        }
    }
}

impl CollisionSystem {
    fn is_vehicle_colliding(&self, transform: &Transform, game_map: &GameMapResource) -> bool {
        let position = transform.translation();
        let tile_x = (position.x / TILE_SIZE).floor() as u32;
        let tile_y = (position.y / TILE_SIZE).floor() as u32;

        !game_map.tile_components.get(&(tile_x, tile_y)).map_or(false, |tile| tile.is_drivable)
    }

    fn apply_movement(&self, vehicle: &VehicleComponents, transform: &mut Transform, delta_time: f32) {
        let displacement = Vector3::new(
            vehicle.direction.x * vehicle.base.speed * delta_time,
            vehicle.direction.y * vehicle.base.speed * delta_time,
            0.0,
        );
        transform.prepend_translation(displacement);
    }
}
