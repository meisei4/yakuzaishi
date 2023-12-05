use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};
use crate::components::vehicle_component::Vehicle;

#[derive(SystemDesc)]
pub struct VehicleControllerSystem;

impl<'s> System<'s> for VehicleControllerSystem {

    type SystemData = (
        WriteStorage<'s, Vehicle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut vehicles, input, time): Self::SystemData) {
        let delta_time = time.delta_seconds();

        let forward_movement = input.axis_value("vehicle_forward").unwrap_or(0.0);
        let turn_movement = input.axis_value("vehicle_turn").unwrap_or(0.0);

        for vehicle in (&mut vehicles).join() {
            if forward_movement != 0.0 {
                vehicle.accelerate(delta_time * forward_movement);
            } else if forward_movement < 0.0 {
                vehicle.decelerate(delta_time * -forward_movement);
            }

            if turn_movement != 0.0 {
                if turn_movement > 0.0 {
                    vehicle.turn_right(delta_time);
                } else {
                    vehicle.turn_left(delta_time);
                }
            }
            vehicle.update_position(delta_time);
        }
    }

}
