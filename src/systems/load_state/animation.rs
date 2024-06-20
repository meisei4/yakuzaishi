use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Query, Res, Resource};
use bevy_ecs_tilemap::tiles::{AnimatedTile, TileTextureIndex};

#[derive(Resource, Debug)]
pub struct AnimationData {
    animations: HashMap<u32, AnimatedTile>,
}

pub fn setup_map_animation_data(mut commands: Commands) {
    let mut animations = HashMap::new();

    // Hard-code the animation for texture index 40
    animations.insert(40, AnimatedTile {
        start: 40,
        end: 55,
        speed: 0.75,
    });

    let animation_data = AnimationData { animations };
    commands.insert_resource(animation_data);
}

pub fn attach_animations_to_map(
    mut commands: Commands,
    query: Query<(Entity, &TileTextureIndex)>,
    animation_data: Res<AnimationData>,
) {
    for (entity, texture_index) in query.iter() {
        if let Some(animation) = animation_data.animations.get(&{ texture_index.0 }) {
            commands.entity(entity).insert(*animation);
        }
    }
}
