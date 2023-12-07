use amethyst::{
    core::{Transform, timing::Time},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings}, renderer::SpriteRender,
};
use log::{info, warn};
use crate::components::vehicle_component::VehicleComponents;

#[derive(SystemDesc)]
pub struct VehicleControllerSystem;

impl<'s> System<'s> for VehicleControllerSystem {
    type SystemData = (
        WriteStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut vehicle_components, mut transforms, mut sprite_renders, input, time): Self::SystemData) {
        let delta_time: f32 = time.delta_seconds();

        for (vehicle_component, transform, sprite_render) in (&mut vehicle_components, &mut transforms, &mut sprite_renders).join() {
            process_input(&input, vehicle_component, delta_time);
            update_vehicle_sprite(vehicle_component, sprite_render);
            update_vehicle_transform(vehicle_component, transform, delta_time);
        }
    }
}
    fn process_input(input: &Read<InputHandler<StringBindings>>, vehicle_component: &mut VehicleComponents, delta_time: f32) {
        handle_forward_movement(input, vehicle_component, delta_time);
        handle_turning(input, vehicle_component, delta_time);
    }

    fn handle_forward_movement(input: &Read<InputHandler<StringBindings>>, vehicle_component: &mut VehicleComponents, delta_time: f32) {
        if let Some(forward_movement) = input.axis_value("vehicle_forward") {
            if forward_movement != 0.0 {
                vehicle_component.adjust_speed(forward_movement, delta_time);
            }
        }
    }

    fn handle_turning(input: &Read<InputHandler<StringBindings>>, vehicle_component: &mut VehicleComponents, delta_time: f32) {
        if let Some(turn_movement) = input.axis_value("vehicle_turn") {
            if turn_movement != 0.0 {
                vehicle_component.adjust_direction(turn_movement, delta_time);
            }
        }
    }

    fn update_vehicle_sprite(vehicle_component: &mut VehicleComponents, sprite_render: &mut SpriteRender) {
        vehicle_component.update_sprite_index(); //TODO perhaps bad practice not sure if should return new sprite instead?
        sprite_render.sprite_number = vehicle_component.current_sprite_index; 
    }

    fn update_vehicle_transform(vehicle_component: &mut VehicleComponents, transform: &mut Transform, delta_time: f32) {
        vehicle_component.update_position(delta_time);
        transform.set_translation_x(vehicle_component.position.x);
        transform.set_translation_y(vehicle_component.position.y);
        transform.set_rotation_2d(vehicle_component.direction_angle()- std::f32::consts::PI / 2.0);
    }

