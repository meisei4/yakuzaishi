use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Time, Transform},
};
use bevy::math::Vec3;
use bevy::prelude::Fixed;

use crate::components::controllable_entity_components::{
    PositionComponent, PreviousPositionComponent, VelocityVectorComponents,
};
use crate::DEFAULT_SPEED;

pub fn control_entity_position_smooth(
    fixed_time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut VelocityVectorComponents,
        &mut PositionComponent,
        &mut PreviousPositionComponent,
    )>,
) {
    for (mut transform, mut controllable_entity, mut position, mut prev_position) in
        query.iter_mut()
    {
        let a = fixed_time.overstep_fraction();
        transform.translation = prev_position.position.lerp(position.position, a);
        process_input(&keyboard_input, &mut controllable_entity);
        prev_position.position = position.position;
        let position_displacement = Vec3 {
            x: controllable_entity.x_axis_strafe_speed,
            y: controllable_entity.y_axis_speed,
            z: 0.0,
        };
        position.position += position_displacement * fixed_time.delta_seconds();
    }
}

fn process_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    controllable_entity: &mut VelocityVectorComponents,
) {
    handle_y_axis_movement(keyboard_input, controllable_entity);
    handle_x_axis_movement(keyboard_input, controllable_entity);
}

fn handle_y_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut VelocityVectorComponents,
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
    vehicle_component: &mut VelocityVectorComponents,
) {
    let strafe_right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let strafe_left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let strafe_direction = (strafe_right - strafe_left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    vehicle_component.x_axis_strafe_speed = DEFAULT_SPEED * strafe_direction;
}
