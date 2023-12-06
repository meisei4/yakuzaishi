use amethyst::{
    core::{Transform, timing::Time, math::{ArrayStorage, U1, U2, Matrix}},
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
            let direction_before: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> = vehicle.direction;

            if let Some(forward_movement) = input.axis_value("vehicle_forward") {
                if forward_movement > 0.0 {
                    vehicle.accelerate(delta_time * forward_movement);
                    //info!("Vehicle accelerating with speed: {}", vehicle.speed);
                } else if forward_movement < 0.0 {
                    vehicle.decelerate(delta_time * -forward_movement);
                    //info!("Vehicle decelerating with speed: {}", vehicle.speed);
                }
            }
            
            if let Some(turn_movement) = input.axis_value("vehicle_turn") {
                if turn_movement > 0.0 {
                    vehicle.turn_right(delta_time);
                } else if turn_movement < 0.0 {
                    vehicle.turn_left(delta_time);
                }
            }
            vehicle.update_position(delta_time);
            //info!("Vehicle updated position: {:?}", vehicle.position);

            if direction_before != vehicle.direction {
                //info!("Vehicle updated direction: {:?}", vehicle.direction);
            }

            transform.set_translation_x(vehicle.position.x);
            transform.set_translation_y(vehicle.position.y);
            // something about sprite orientation in amethyst requires rotation here
            transform.set_rotation_2d(vehicle.direction_angle() - std::f32::consts::PI / 2.0);

        }
    }
}
