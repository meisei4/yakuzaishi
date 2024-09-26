use bevy::prelude::{Query, Res, TextureAtlas, Time, With};

use crate::anime::anime_component::{AnimationComponent, AnimationTimer};
use crate::kinetic_entity::EnvironmentEntityTag;

pub fn animate_env_entity_animations(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationTimer, &AnimationComponent, &mut TextureAtlas),
        With<EnvironmentEntityTag>,
    >,
) {
    for (mut timer, animation, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let current_index = overlay_atlas.index;

            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            overlay_atlas.index = next_index;
        }
    }
}
