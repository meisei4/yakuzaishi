use std::f32::consts::PI;

use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, TextureAtlas, Time, Transform, Vec2},
};
use bevy::math::Vec3;
use bevy::prelude::Fixed;

use crate::{DEFAULT_SPEED, TILE_SIZE};
use crate::components::controlled_entity_components::ControlledEntityComponents;
use crate::components::entity_movement_states::{CurrentMovementState, PreviousMovementState};

pub fn apply_entity_movement_states_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut ControlledEntityComponents,
        &mut Transform,
        Option<&mut TextureAtlas>,
        &CurrentMovementState,
        &PreviousMovementState,
    )>,
) {
    for (mut vehicle, mut transform, player_entity_texture_atlas, state, old_state) in
        query.iter_mut()
    {
        let a = fixed_time.overstep_fraction();
        transform.translation = old_state.position.lerp(state.position, a);
        vehicle.world_coordinate_position.x = transform.translation.x;
        vehicle.world_coordinate_position.y = transform.translation.y;

        update_tile_position(&mut vehicle);
        update_sprite_index(&mut vehicle);
        if let Some(mut player_entity_texture_atlas) = player_entity_texture_atlas {
            update_sprite_index(&mut vehicle);
            player_entity_texture_atlas.index = vehicle.current_sprite_index;
        }
    }
}

pub fn update_entity_movement_states_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut ControlledEntityComponents,
        &mut CurrentMovementState,
        &mut PreviousMovementState,
    )>,
) {
    for (mut vehicle, mut state, mut prev_state) in query.iter_mut() {
        process_input_flying_entity(
            &keyboard_input,
            &mut vehicle,
            &mut state,
            &mut prev_state,
            &time,
        );
    }
}

fn process_input_flying_entity(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle: &mut ControlledEntityComponents,
    state: &mut CurrentMovementState,
    prev_state: &mut PreviousMovementState,
    time: &Time,
) {
    handle_y_axis_movement(keyboard_input, vehicle);
    handle_x_axis_strafing(keyboard_input, vehicle);
    let state = &mut *state;
    // Update motion states
    prev_state.position = state.position;
    state.movement = Vec3 {
        x: vehicle.x_axis_strafe_speed,
        y: vehicle.y_axis_speed,
        z: 0.0,
    };
    state.position += state.movement * time.delta_seconds();
}

fn handle_y_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut ControlledEntityComponents,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        vehicle_component.y_axis_speed = DEFAULT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        vehicle_component.y_axis_speed = -DEFAULT_SPEED;
    } else {
        vehicle_component.y_axis_speed = 0.0;
    }
}

fn handle_x_axis_strafing(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut ControlledEntityComponents,
) {
    let strafe_right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let strafe_left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let strafe_direction = (strafe_right - strafe_left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    vehicle_component.x_axis_strafe_speed = DEFAULT_SPEED * strafe_direction;
}

fn update_tile_position(vehicle: &mut ControlledEntityComponents) {
    //TODO: the TILE_SIZE centering and just coordinate system in general needs to be fixed i think,
    // seems too hacky
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

fn update_sprite_index(vehicle: &mut ControlledEntityComponents) {
    let angle = direction_angle_fly(vehicle);
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

fn direction_angle_fly(vehicle: &ControlledEntityComponents) -> f32 {
    vehicle.direction.y.atan2(vehicle.direction.x)
}
