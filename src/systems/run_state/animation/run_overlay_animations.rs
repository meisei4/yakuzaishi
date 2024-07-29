use bevy::prelude::{Query, Res, TextureAtlas, Time};

use crate::components::animation_timer::AnimationTimer;
use crate::components::overlay_animation::OverlayAnimation;

pub fn animate_overlay_continuous(
    time: Res<Time>,
    //TODO: This is grabbing the TextureAtlas that corresponds to the Car sprite
    // -- I think there is a huge issue with how I am wrapping up core Components?
    // -- to the point that they cant effectively be queried
    // It also seems like a Resource issue.
    // Look at the world Inspector and how you cant view the custom Components
    // that I try to add (ones that contain core things like TextureAtlas
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &OverlayAnimation)>,
) {
    for (mut timer, mut atlas, animation) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                atlas.index + 1
            };
        }
    }
}
