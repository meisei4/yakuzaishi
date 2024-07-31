use bevy::log::info;
use bevy::prelude::{Query, Res, TextureAtlas, Time};

use crate::components::animation_timer::AnimationTimer;
use crate::components::overlay_animation::OverlayAnimationTextureAtlas;
use crate::resources::animation_resources::OverlayAnimationData;

pub fn animate_overlay_animations(
    time: Res<Time>,
    overlay_animation_data: Res<OverlayAnimationData>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        // Log the timer's state
        // info!("Timer: {:?}", timer);

        if timer.0.just_finished() {
            let animation = overlay_animation_data.wake_animation;

            // Log the current and next indices
            let current_index = overlay_atlas.index;
            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            info!(
                "Updating animation index from {} to {}",
                current_index, next_index
            );

            overlay_atlas.index = next_index;
        }
    }
}
