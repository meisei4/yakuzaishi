use bevy::core::Name;
use bevy::prelude::{Commands, GlobalTransform, InheritedVisibility, Transform, Visibility};

use crate::{
    ENVIRONMENT_ENTITY_SPAWN_X, ENVIRONMENT_ENTITY_SPAWN_Y, ENVIRONMENT_ENTITY_Z_LEVEL, TILE_SIZE,
};
use crate::kinetic_entity::{EnvironmentEntityTag, KineticEntityComponents};

pub fn spawn_environment_entity(mut commands: Commands) {
    let transform = Transform::from_xyz(
        ENVIRONMENT_ENTITY_SPAWN_X * TILE_SIZE,
        ENVIRONMENT_ENTITY_SPAWN_Y * TILE_SIZE,
        ENVIRONMENT_ENTITY_Z_LEVEL,
    );

    let env_entity = KineticEntityComponents {
        y_axis_displacement: 0.0,
        x_axis_displacement: 0.0,
        position: transform.translation,
        prev_position: transform.translation,
    };
    commands
        .spawn((env_entity, transform))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(EnvironmentEntityTag)
        .insert(Name::new("Environmental_Entity"));
}
