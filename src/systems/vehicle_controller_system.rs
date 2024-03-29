use std::f32::consts::PI;

use bevy::{input::keyboard::KeyboardInput, prelude::{KeyCode, Query, Res, Sprite, Time, Transform, UVec2, Vec2}};

use crate::{components::vehicle_components::VehicleComponents, TILE_SIZE};

pub fn vehicle_controller_system(time: Res<Time>,
                                 keyboard_input: Res<KeyboardInput>,
                                 mut query: Query<(&mut VehicleComponents,
                                                   &mut Transform,
                                                   &mut Sprite)>,
) {
    let delta_time = time.delta_seconds();

    for (mut vehicle, mut transform, sprite) in query.iter_mut() {
        process_input(&keyboard_input, &mut vehicle, delta_time);
        update_position_and_transform(&mut vehicle, delta_time, &mut transform);
        update_sprite_index_and_hitbox_index(&mut vehicle);
    }
}


fn process_input(
    keyboard_input: &Res<KeyboardInput>,
    vehicle: &mut VehicleComponents,
    delta_time: f32,
) {
    handle_forward_movement(keyboard_input, vehicle, delta_time);
    handle_turning(keyboard_input, vehicle, delta_time);
}

fn handle_forward_movement(
    keyboard_input: &Res<KeyboardInput>,
    vehicle_component: &mut VehicleComponents,
    delta_time: f32,
) {
    //TODO; Cool ass algebraic solution from gpt vs the if forward -> 1, if backward -> -1, else nothing -> 0 grossness
    let forward = keyboard_input.pressed(KeyCode::KeyW) as i32;
    let backward = keyboard_input.pressed(KeyCode::KeyS) as i32;
    let forward_movement = (forward - backward) as f32; // 1 if W is pressed, -1 if S is pressed, 0 otherwise

    adjust_speed(vehicle_component, forward_movement, delta_time);
}

fn handle_turning(
    keyboard_input: &Res<KeyboardInput>,
    vehicle_component: &mut VehicleComponents,
    delta_time: f32,
) {
    let right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let turn_movement = (right - left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    adjust_direction(vehicle_component, turn_movement, delta_time);
}

fn update_position_and_transform(vehicle: &mut VehicleComponents, delta_time: f32, transform: &mut Transform) {
    let displacement = Vec2::new(
        vehicle.direction.x * vehicle.base.speed,
        vehicle.direction.y * vehicle.base.speed,
    ) * delta_time;
    vehicle.base.position.x += displacement.x;
    vehicle.base.position.y += displacement.y;

    // TODO: stop having to convert back and forth between world coordinates and tile coordinates
    let new_tile_x = (vehicle.base.position.x / TILE_SIZE).floor();
    let new_tile_y = (vehicle.base.position.y / TILE_SIZE).floor();
    let new_tile = UVec2::new(new_tile_x as u32, new_tile_y as u32);
    if new_tile != vehicle.current_tile {
        log::info!(
            "Vehicle has moved to a new tile: {:?} from old tile {:?}",
            new_tile,
            vehicle.current_tile
        );
        vehicle.current_tile = new_tile;
    }
    transform.translation.x = vehicle.base.position.x;
    transform.translation.y = vehicle.base.position.y;
}

fn adjust_speed(
    vehicle: &mut VehicleComponents,
    forward_movement: f32,
    delta_time: f32,
) {
    if forward_movement > 0.0 {
        accelerate(vehicle, delta_time * forward_movement);
    } else if forward_movement < 0.0 {
        decelerate(vehicle, delta_time * -forward_movement);
    }
}

fn accelerate(vehicle: &mut VehicleComponents, delta_time: f32) {
    vehicle.base.speed += vehicle.acceleration * delta_time;
    if vehicle.base.speed > vehicle.max_speed {
        vehicle.base.speed = vehicle.max_speed;
    }
}

fn decelerate(vehicle: &mut VehicleComponents, delta_time: f32) {
    vehicle.base.speed -= vehicle.deceleration * delta_time;
    if vehicle.base.speed < 0.0 {
        vehicle.base.speed = 0.0;
    }
}

fn adjust_direction(
    vehicle: &mut VehicleComponents,
    turn_movement: f32,
    delta_time: f32,
) {
    if turn_movement > 0.0 {
        turn_right(vehicle, delta_time);
    } else if turn_movement < 0.0 {
        turn_left(vehicle, delta_time);
    }
}

fn turn_left(vehicle: &mut VehicleComponents, delta_time: f32) {
    let rotation_amount = vehicle.rotation_speed * delta_time;
    let new_direction_angle = direction_angle(&vehicle) + rotation_amount;
    vehicle.direction =
        Vec2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn turn_right(vehicle: &mut VehicleComponents, delta_time: f32) {
    let rotation_amount = vehicle.rotation_speed * delta_time;
    let new_direction_angle = direction_angle(&vehicle) - rotation_amount;
    vehicle.direction =
        Vec2::new(new_direction_angle.cos(), new_direction_angle.sin());
}

fn direction_angle(vehicle: &VehicleComponents) -> f32 {
    vehicle
        .direction
        .y
        .atan2(vehicle.direction.x)
}

fn update_sprite_index_and_hitbox_index(vehicle: &mut VehicleComponents) {
    let angle = direction_angle(vehicle);
    let normalized_angle = (angle + 2.0 * PI) % (2.0 * PI);
    let north_sprite_index = 36; // Index of North-facing sprite
    let total_sprites = 48;
    let radians_per_sprite = 2.0 * PI / total_sprites as f32;

    let index_offset = ((normalized_angle - PI / 2.0) / radians_per_sprite).round() as isize;

    let updated_sprite_index =
        (north_sprite_index as isize - index_offset).rem_euclid(total_sprites as isize) as usize;

    if updated_sprite_index != vehicle.base.current_sprite_index {
        vehicle.base.current_sprite_index = updated_sprite_index;
        log::info!("Raw direction vector: {:?}", vehicle.direction);
        log::info!("Normalized direction angle: {} radians", normalized_angle);
        log::info!(
            "Updating sprite index: {} -> {}",
            vehicle.base.current_sprite_index,
            updated_sprite_index
        );
    }
    vehicle.current_hitbox_index = updated_sprite_index
}
