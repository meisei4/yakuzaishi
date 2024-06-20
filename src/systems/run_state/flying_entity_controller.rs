use std::f32::consts::PI;

use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, TextureAtlas, Time, Transform, Vec2},
};

use crate::{DEFAULT_SPEED, TILE_SIZE};
use crate::components::flying_entity_components::FlyingEntityComponents;

pub fn vehicle_controller_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut FlyingEntityComponents,
        &mut Transform,
        &mut TextureAtlas,
    )>,
) {
    let delta_time = time.delta_seconds();

    for (mut vehicle, mut transform, mut player_entity_texture_atlas) in query.iter_mut() {
        process_input(&keyboard_input, &mut vehicle, delta_time);
        transform.translation.x = vehicle.world_coordinate_position.x;
        transform.translation.y = vehicle.world_coordinate_position.y;
        update_tile_position(&mut vehicle);
        update_sprite_index(&mut vehicle);
        player_entity_texture_atlas.index = vehicle.current_sprite_index;
    }
}

fn process_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle: &mut FlyingEntityComponents,
    delta_time: f32,
) {
    handle_y_axis_movement(keyboard_input, vehicle, delta_time);
    handle_x_axis_strafing(keyboard_input, vehicle, delta_time);
}

fn handle_y_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut FlyingEntityComponents,
    delta_time: f32,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        vehicle_component.y_axis_speed = DEFAULT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        vehicle_component.y_axis_speed = -DEFAULT_SPEED;
    } else {
        vehicle_component.y_axis_speed = 0.0;
    }
    let forward_movement_amount = vehicle_component.y_axis_speed * delta_time;
    vehicle_component.world_coordinate_position.y += forward_movement_amount;
}

fn handle_x_axis_strafing(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut FlyingEntityComponents,
    delta_time: f32,
) {
    let strafe_right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let strafe_left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let strafe_direction = (strafe_right - strafe_left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    let strafe_amount = vehicle_component.x_axis_strafe_speed * delta_time * strafe_direction;
    vehicle_component.world_coordinate_position.x += strafe_amount;
}

fn update_tile_position(vehicle: &mut FlyingEntityComponents) {
    let new_tile_x = (vehicle.world_coordinate_position.x / TILE_SIZE).floor();
    let new_tile_y = (vehicle.world_coordinate_position.y / TILE_SIZE).floor();
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

fn update_sprite_index(vehicle: &mut FlyingEntityComponents) {
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

fn direction_angle(vehicle: &FlyingEntityComponents) -> f32 {
    vehicle.direction.y.atan2(vehicle.direction.x)
}
