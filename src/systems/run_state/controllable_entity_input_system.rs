use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Time, Transform},
};
use bevy::math::Vec3;
use bevy::prelude::Fixed;

use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::components::entity_movement_states::{CurrentMovementState, PreviousMovementState};
use crate::DEFAULT_SPEED;

// TODO: the naming for every method in this module is horrible. fix it.
pub fn apply_entity_movement_states_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &CurrentMovementState,
        &PreviousMovementState,
    )>,
) {
    for (mut transform, state, old_state) in query.iter_mut() {
        let a = fixed_time.overstep_fraction();
        transform.translation = old_state.position.lerp(state.position, a);
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
