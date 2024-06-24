use std::f32::consts::PI;

use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, TextureAtlas, Time, Transform, Vec2},
};

use crate::components::rotation_vehicle_components::RotationalVehicleComponents;
use crate::TILE_SIZE;

pub fn rotation_vehicle_controller_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut RotationalVehicleComponents,
        &mut Transform,
        &mut TextureAtlas,
    )>,
) {
    let delta_time = time.delta_seconds();

    for (mut vehicle, mut transform, mut player_entity_texture_atlas) in query.iter_mut() {
        process_input_rotational_entity(&keyboard_input, &mut vehicle, delta_time);
        update_position_and_transform(&mut vehicle, delta_time, &mut transform);
        update_tile_position(&mut vehicle);
        update_sprite_index(&mut vehicle);
        player_entity_texture_atlas.index = vehicle.current_sprite_index;
    }
}

fn update_tile_position(vehicle: &mut RotationalVehicleComponents) {
    let new_tile_x =
        ((vehicle.world_coordinate_position.x + (TILE_SIZE / 2.0)) / TILE_SIZE).floor();
    let new_tile_y =
        ((vehicle.world_coordinate_position.y + (TILE_SIZE / 2.0)) / TILE_SIZE).floor();
    let new_tile = Vec2 {
        x: new_tile_x,
        y: new_tile_y,
    };
    if new_tile != vehicle.tile_coordinate_position {
        log::info!(
            "Vehicle has moved to a new tile: {:?} from old tile {:?}",
            new_tile,
            vehicle.tile_coordinate_position
        );
        vehicle.tile_coordinate_position = new_tile;
    }
}

// ---------------------------------------------ROTATIONAL_VEHICLE----------------------------------

fn process_input_rotational_entity(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle: &mut RotationalVehicleComponents,
    delta_time: f32,
) {
    handle_forward_movement(keyboard_input, vehicle, delta_time);
    handle_turning(keyboard_input, vehicle, delta_time);
}

fn handle_forward_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut RotationalVehicleComponents,
    delta_time: f32,
) {
    // Cool ass algebraic solution from gpt vs the if forward -> 1, if backward -> -1, else nothing -> 0 grossness
    let forward = keyboard_input.pressed(KeyCode::KeyW) as i32;
    let backward = keyboard_input.pressed(KeyCode::KeyS) as i32;
    let forward_movement = (forward - backward) as f32; // 1 if W is pressed, -1 if S is pressed, 0 otherwise

    adjust_speed(vehicle_component, forward_movement, delta_time);
}

fn adjust_speed(vehicle: &mut RotationalVehicleComponents, forward_movement: f32, delta_time: f32) {
    if forward_movement > 0.0 {
        accelerate(vehicle, delta_time * forward_movement);
    } else if forward_movement < 0.0 {
        decelerate(vehicle, delta_time * -forward_movement);
    }
}

fn accelerate(vehicle: &mut RotationalVehicleComponents, delta_time: f32) {
    vehicle.speed += vehicle.acceleration * delta_time;
    if vehicle.speed > vehicle.max_speed {
        vehicle.speed = vehicle.max_speed;
    }
}

fn decelerate(vehicle: &mut RotationalVehicleComponents, delta_time: f32) {
    vehicle.speed -= vehicle.deceleration * delta_time;
    if vehicle.speed < 0.0 {
        vehicle.speed = 0.0;
    }
}

fn handle_turning(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut RotationalVehicleComponents,
    delta_time: f32,
) {
    let right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let turn_movement = (right - left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    adjust_direction(vehicle_component, turn_movement, delta_time);
}

fn adjust_direction(
    vehicle: &mut RotationalVehicleComponents,
    turn_movement: f32,
    delta_time: f32,
) {
    if turn_movement > 0.0 {
        turn_right(vehicle, delta_time);
    } else if turn_movement < 0.0 {
        turn_left(vehicle, delta_time);
    }
}

fn turn_left(vehicle: &mut RotationalVehicleComponents, delta_time: f32) {
    let rotation_amount = vehicle.rotation_speed * delta_time;
    let new_direction_angle = direction_angle(&vehicle) + rotation_amount;
    vehicle.direction = Vec2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn turn_right(vehicle: &mut RotationalVehicleComponents, delta_time: f32) {
    let rotation_amount = vehicle.rotation_speed * delta_time;
    let new_direction_angle = direction_angle(&vehicle) - rotation_amount;
    vehicle.direction = Vec2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn update_position_and_transform(
    vehicle: &mut RotationalVehicleComponents,
    delta_time: f32,
    transform: &mut Transform,
) {
    let displacement = Vec2::new(
        vehicle.direction.x * vehicle.speed,
        vehicle.direction.y * vehicle.speed,
    ) * delta_time;
    vehicle.world_coordinate_position.x += displacement.x;
    vehicle.world_coordinate_position.y += displacement.y;

    transform.translation.x = vehicle.world_coordinate_position.x;
    transform.translation.y = vehicle.world_coordinate_position.y;
}

fn update_sprite_index(vehicle: &mut RotationalVehicleComponents) {
    let angle = direction_angle(vehicle);
    let normalized_angle = (angle + 2.0 * PI) % (2.0 * PI);
    let north_sprite_index = 36; // Index of North-facing sprite
    let total_sprites = 48;
    let radians_per_sprite = 2.0 * PI / total_sprites as f32;

    let index_offset = ((normalized_angle - PI / 2.0) / radians_per_sprite).round() as isize;

    let updated_sprite_index =
        (north_sprite_index as isize - index_offset).rem_euclid(total_sprites as isize) as usize;

    if updated_sprite_index != vehicle.current_sprite_index {
        vehicle.current_sprite_index = updated_sprite_index;
    }
}

fn direction_angle(vehicle: &RotationalVehicleComponents) -> f32 {
    vehicle.direction.y.atan2(vehicle.direction.x)
}
