use bevy::prelude::{Query, Res, TextureAtlas, Time};

use crate::components::animation_components::AnimationTimer;
use crate::resources::animation_resources::OverlayAnimationResource;

pub fn animate_overlay_animations(
    time: Res<Time>,
    overlay_animation_data: Res<OverlayAnimationResource>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            //TODO: Is there some sort of implied Copy or Clone happening here? When
            // AnimationComponent doesnt derive copy or clone this next line doesnt work
            let animation = overlay_animation_data.wake_animation;

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
