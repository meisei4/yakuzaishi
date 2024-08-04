use bevy::core::Name;
use bevy::prelude::{Commands, GlobalTransform, InheritedVisibility, Transform, Visibility};

use crate::{
    ENVIRONMENT_ENTITY_Z_LEVEL, ENVIRONMENTAL_ENTITY_SPAWN_X, ENVIRONMENTAL_ENTITY_SPAWN_Y,
    TILE_SIZE,
};
use crate::components::behavior_patterns::FloatInCircle;
use crate::components::environmental_entity::EnvironmentalEntityComponents;

pub fn spawn_environmental_entity(mut commands: Commands) {
    let transform = Transform::from_xyz(
        ENVIRONMENTAL_ENTITY_SPAWN_X * TILE_SIZE,
        ENVIRONMENTAL_ENTITY_SPAWN_Y * TILE_SIZE,
        ENVIRONMENT_ENTITY_Z_LEVEL,
    );

    let env_entity = EnvironmentalEntityComponents {
        // behavior: BehaviorPattern::FloatInCircle,
        y_axis_displacement: 0.0,
        x_axis_displacement: 0.0,
        position: transform.translation,
        prev_position: transform.translation,
    };
    commands
        .spawn((env_entity, transform))
        .insert(FloatInCircle {
            radius: 50.0,
            speed: 1.0,
            angle: 0.0,
        })
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(Name::new("Environmental_Entity"));
}
