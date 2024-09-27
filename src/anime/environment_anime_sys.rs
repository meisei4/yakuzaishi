use bevy::prelude::{Query, Res, TextureAtlas, Time, With};

use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::kinetic_components::EnvironmentEntityTag;

pub fn animate_env_entity_animations(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationTimer, &AnimationComponent, &mut TextureAtlas),
        With<EnvironmentEntityTag>,
    >,
) {
    //TODO: refactor all the animation logic to work without having copy pasted code
    for (mut timer, animation, mut texture_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let current_index = texture_atlas.index;

            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            texture_atlas.index = next_index;
        }
    }
}
