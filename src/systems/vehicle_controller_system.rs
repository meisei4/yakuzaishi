use std::f32::consts::PI;

use crate::components::vehicle_components::VehicleComponents;
use crate::util::update_transform;
use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, U1, U2},
        timing::Time,
        Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

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

    fn run(
        &mut self,
        (mut vehicle_components, mut transforms, mut sprite_renders, input, time): Self::SystemData,
    ) {
        let delta_time: f32 = time.delta_seconds();

        for (vehicle_component, transform, sprite_render) in (
            &mut vehicle_components,
            &mut transforms,
            &mut sprite_renders,
        )
            .join()
        {
            process_input(&input, vehicle_component, delta_time);
            update_position(vehicle_component, delta_time);
            update_transform(&vehicle_component.base, transform);
            sprite_render.sprite_number = update_sprite_index(vehicle_component);
        }
    }
}

fn process_input(
    input: &Read<InputHandler<StringBindings>>,
    vehicle_component: &mut VehicleComponents,
    delta_time: f32,
) {
    handle_forward_movement(input, vehicle_component, delta_time);
    handle_turning(input, vehicle_component, delta_time);
}

fn handle_forward_movement(
    input: &Read<InputHandler<StringBindings>>,
    vehicle_component: &mut VehicleComponents,
    delta_time: f32,
) {
    if let Some(forward_movement) = input.axis_value("vehicle_forward") {
        if forward_movement != 0.0 {
            adjust_speed(vehicle_component, forward_movement, delta_time);
        }
    }
}

fn handle_turning(
    input: &Read<InputHandler<StringBindings>>,
    vehicle_component: &mut VehicleComponents,
    delta_time: f32,
) {
    if let Some(turn_movement) = input.axis_value("vehicle_turn") {
        if turn_movement != 0.0 {
            adjust_direction(vehicle_component, turn_movement, delta_time);
        }
    }
}

fn update_position(vehicle_components: &mut VehicleComponents, delta_time: f32) {
    let displacement: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> = Vector2::new(
        vehicle_components.direction.x * vehicle_components.base.speed,
        vehicle_components.direction.y * vehicle_components.base.speed,
    ) * delta_time;
    vehicle_components.base.position.x += displacement.x;
    vehicle_components.base.position.y += displacement.y;
}

fn adjust_speed(
    vehicle_components: &mut VehicleComponents,
    forward_movement: f32,
    delta_time: f32,
) {
    if forward_movement > 0.0 {
        accelerate(vehicle_components, delta_time * forward_movement);
    } else if forward_movement < 0.0 {
        decelerate(vehicle_components, delta_time * -forward_movement);
    }
}

fn accelerate(vehicle_components: &mut VehicleComponents, delta_time: f32) {
    vehicle_components.base.speed += vehicle_components.acceleration * delta_time;
    if vehicle_components.base.speed > vehicle_components.max_speed {
        vehicle_components.base.speed = vehicle_components.max_speed;
    }
}

fn decelerate(vehicle_components: &mut VehicleComponents, delta_time: f32) {
    vehicle_components.base.speed -= vehicle_components.deceleration * delta_time;
    if vehicle_components.base.speed < 0.0 {
        vehicle_components.base.speed = 0.0;
    }
}

fn adjust_direction(
    vehicle_components: &mut VehicleComponents,
    turn_movement: f32,
    delta_time: f32,
) {
    if turn_movement > 0.0 {
        turn_right(vehicle_components, delta_time);
    } else if turn_movement < 0.0 {
        turn_left(vehicle_components, delta_time);
    }
}

fn turn_left(vehicle_components: &mut VehicleComponents, delta_time: f32) {
    let rotation_amount: f32 = vehicle_components.rotation_speed * delta_time;
    let new_direction_angle: f32 = direction_angle(vehicle_components) + rotation_amount;
    vehicle_components.direction =
        Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn turn_right(vehicle_components: &mut VehicleComponents, delta_time: f32) {
    let rotation_amount: f32 = vehicle_components.rotation_speed * delta_time;
    let new_direction_angle: f32 = direction_angle(vehicle_components) - rotation_amount;
    vehicle_components.direction =
        Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn direction_angle(vehicle_components: &mut VehicleComponents) -> f32 {
    vehicle_components
        .direction
        .y
        .atan2(vehicle_components.direction.x)
}

fn update_sprite_index(vehicle_components: &mut VehicleComponents) -> usize {
    let angle: f32 = direction_angle(vehicle_components);
    let normalized_angle: f32 = (angle + 2.0 * PI) % (2.0 * PI);
    // Calculate sprite index
    let north_sprite_index: i32 = 36; // Index of North-facing sprite
    let total_sprites: i32 = 48;
    let radians_per_sprite: f32 = 2.0 * PI / total_sprites as f32;

    // Calculate the index offset from North
    let index_offset: isize = ((normalized_angle - PI / 2.0) / radians_per_sprite).round() as isize;

    // Adjust the sprite index considering clockwise direction from North
    let updated_sprite_index: usize =
        (north_sprite_index as isize - index_offset).rem_euclid(total_sprites as isize) as usize;

    if updated_sprite_index != vehicle_components.base.current_sprite_index {
        vehicle_components.base.current_sprite_index = updated_sprite_index;
        log::debug!("Raw direction vector: {:?}", vehicle_components.direction);
        log::debug!("Normalized direction angle: {} radians", normalized_angle);
        log::debug!(
            "Updating sprite index: {} -> {}",
            vehicle_components.base.current_sprite_index,
            updated_sprite_index
        );
    }
    updated_sprite_index
}
