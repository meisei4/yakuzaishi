use std::f32::consts::PI;

use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, TextureAtlas, Time, Transform, Vec2},
};
use bevy::math::Vec3;
use bevy::prelude::Fixed;

use crate::{DEFAULT_SPEED, TILE_SIZE};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::components::entity_movement_states::{CurrentMovementState, PreviousMovementState};

// TODO: the naming for every method in this module is horrible. fix it.
pub fn apply_entity_movement_states_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut ControllableEntityComponents,
        &mut Transform,
        &CurrentMovementState,
        &PreviousMovementState,
    )>,
) {
    for (mut vehicle, mut transform, state, old_state) in query.iter_mut() {
        let a = fixed_time.overstep_fraction();
        transform.translation = old_state.position.lerp(state.position, a);
        vehicle.world_coordinate_position.x = transform.translation.x;
        vehicle.world_coordinate_position.y = transform.translation.y;

        update_tile_position(&mut vehicle);
    }
}

pub fn update_entity_movement_states_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut ControllableEntityComponents,
        &mut CurrentMovementState,
        &mut PreviousMovementState,
    )>,
) {
    for (mut vehicle, mut state, mut prev_state) in query.iter_mut() {
        process_input(
            &keyboard_input,
            &mut vehicle,
            &mut state,
            &mut prev_state,
            &time,
        );
    }
}

fn process_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle: &mut ControllableEntityComponents,
    state: &mut CurrentMovementState,
    prev_state: &mut PreviousMovementState,
    time: &Time,
) {
    handle_y_axis_movement(keyboard_input, vehicle);
    handle_x_axis_movement(keyboard_input, vehicle);
    let state = &mut *state;
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
    vehicle_component: &mut ControllableEntityComponents,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        vehicle_component.y_axis_speed = DEFAULT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        vehicle_component.y_axis_speed = -DEFAULT_SPEED;
    } else {
        vehicle_component.y_axis_speed = 0.0;
    }
}

fn handle_x_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut ControllableEntityComponents,
) {
    let strafe_right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let strafe_left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let strafe_direction = (strafe_right - strafe_left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    vehicle_component.x_axis_strafe_speed = DEFAULT_SPEED * strafe_direction;
}

fn update_tile_position(vehicle: &mut ControllableEntityComponents) {
    //TODO: the TILE_SIZE centering and just coordinate system in general needs to be fixed i think,
    // seems too hacky

    // TODO: UPDATE- figure out how to turn it all into the thing from ControllableEntityComponents TODO and motion states
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
