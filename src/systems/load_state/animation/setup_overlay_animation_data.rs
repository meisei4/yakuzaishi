use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, Image, Query, Res, ResMut, SpriteBundle, TextureAtlas, TextureAtlasLayout,
    Transform, With,
};
use bevy::time::{Timer, TimerMode};
use log::info;

use crate::components::animation_timer::AnimationTimer;
use crate::components::flying_entity_components::FlyingEntityComponents;
use crate::components::overlay_animation::OverlayAnimation;
use crate::resources::animation_resources::OverlayAnimationData;
use crate::WAKE_ANIMATION;

pub fn load_and_setup_overlay_animation_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Log the asset paths being used
    info!("Loading WAKE_ANIMATION from path: {}", WAKE_ANIMATION);

    let wake_texture_handle = asset_server.load(WAKE_ANIMATION);

    // Log the handles to see if assets are being loaded
    info!("Wake texture handle: {:?}", wake_texture_handle);

    let wake_texture_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(16.0),
        20,
        1,
        None,
        None,
    ));

    info!("Wake texture layout handle: {:?}", wake_texture_layout);

    commands.insert_resource(OverlayAnimationData {
        wake_animation: OverlayAnimation {
            start_idx: 0,
            end_idx: 19,
            speed: 0.5,
            z_index: 2.0,
        },
        wake_texture_handle,
        wake_texture_layout,
    });

    info!("OverlayAnimationData resource has been set up.");
}

pub fn attach_overlay_animations_to_flying_entities(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Handle<Image>), With<FlyingEntityComponents>>,
    overlay_animation_data: Res<OverlayAnimationData>,
) {
    for (entity, transform, texture) in query.iter() {
        let animation = overlay_animation_data.wake_animation; // Assume wake animation for now
        commands
            .entity(entity)
            .insert(animation)
            .insert(AnimationTimer(Timer::from_seconds(
                animation.speed,
                TimerMode::Repeating,
            )))
            .insert(SpriteBundle {
                texture: texture.clone(),
                transform: *transform, // Use the existing transform of the entity
                ..Default::default()
            })
            .insert(TextureAtlas {
                layout: overlay_animation_data.wake_texture_layout.clone(),
                index: overlay_animation_data.wake_animation.start_idx as usize,
            });

        info!(
            "Attached overlay animation to Flying Entity {:?}: start_idx {}, end_idx {}, speed {}",
            entity, animation.start_idx, animation.end_idx, animation.speed
        );
    }
}
