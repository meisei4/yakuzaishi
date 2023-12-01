use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData, WriteStorage, Join},
    ecs::prelude::{ReadStorage, Write},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Vehicle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut vehicles, input): Self::SystemData) {
        for vehicle in (&mut vehicles).join() {
            let acceleration = input.axis_value("accelerate");
            let steering = input.axis_value("steer");

            if let Some(accel_value) = acceleration {
                if accel_value > 0.0 {
                    vehicle.accelerate(accel_value as f32);
                } else {
                    vehicle.decelerate((-accel_value) as f32);
                }
            }

            if let Some(steer_value) = steering {
                if steer_value > 0.0 {
                    vehicle.turn_right(steer_value as f32);
                } else if steer_value < 0.0 {
                    vehicle.turn_left((-steer_value) as f32);
                }
            }
        }
    }
}
