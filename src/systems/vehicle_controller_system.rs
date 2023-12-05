use amethyst::{
    core::{Transform, timing::Time},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};
use log::{info, warn};
use crate::components::vehicle_component::VehicleComponents;

#[derive(SystemDesc)]
pub struct VehicleControllerSystem;

impl<'s> System<'s> for VehicleControllerSystem {
    type SystemData = (
        WriteStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut vehicles, mut transforms, input, time): Self::SystemData) {
        let delta_time = time.delta_seconds();

        for (vehicle, transform) in (&mut vehicles, &mut transforms).join() {
            let direction_before = vehicle.direction;

            // Process forward movement input
            if let Some(forward_movement) = input.axis_value("vehicle_forward") {
                if forward_movement > 0.0 {
                    vehicle.accelerate(delta_time * forward_movement);
                    info!("Vehicle accelerating with speed: {}", vehicle.speed);
                } else if forward_movement < 0.0 {
                    vehicle.decelerate(delta_time * -forward_movement);
                    info!("Vehicle decelerating with speed: {}", vehicle.speed);
                }
            } else {
                warn!("No forward movement input detected.");
            }

            // Process turn movement input
            if let Some(turn_movement) = input.axis_value("vehicle_turn") {
                if turn_movement > 0.0 {
                    vehicle.turn_right(delta_time);
                    info!("Vehicle turning right");
                } else if turn_movement < 0.0 {
                    vehicle.turn_left(delta_time);
                    info!("Vehicle turning left");
                }
            } else {
                warn!("No turn movement input detected.");
            }

            // Update the vehicle's position
            vehicle.update_position(delta_time);
            info!("Vehicle updated position: {:?}", vehicle.position);

            // Log direction changes
            if direction_before != vehicle.direction {
                info!("Vehicle updated direction: {:?}", vehicle.direction);
            }

            // Update the Transform component with the new vehicle position
            transform.set_translation_x(vehicle.position.x);
            transform.set_translation_y(vehicle.position.y);
            //TODO figure out how this is the inversion of what the sprite rotation is demonstrating
            transform.set_rotation_2d(vehicle.direction_angle());
        }
    }
}
