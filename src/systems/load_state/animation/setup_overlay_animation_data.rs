use bevy::asset::{Assets, AssetServer, Handle};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, GlobalTransform, InheritedVisibility, Query, Res, ResMut, SpriteBundle,
    SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, Visibility, With,
};
use bevy::time::{Timer, TimerMode};
use log::info;

use crate::{TILE_SIZE, VEHICLE_TEXTURE_FILE_PATH, WAKE_ANIMATION};
use crate::components::animation_timer::AnimationTimer;
use crate::components::flying_entity_components::FlyingEntityComponents;
use crate::components::overlay_animation::{OverlayAnimation, OverlayAnimationTextureAtlas};
use crate::resources::animation_resources::OverlayAnimationData;

pub fn load_and_setup_overlay_animation_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Log the asset paths being used
    info!("Loading WAKE_ANIMATION from path: {}", WAKE_ANIMATION);

    let overlay_animation_image_handle = asset_server.load(WAKE_ANIMATION);

    // Log the handles to see if assets are being loaded
    info!("Wake texture handle: {:?}", overlay_animation_image_handle);

    let overlay_animation_texture_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        20,
        1,
        None,
        None,
    ));

    commands.insert_resource(OverlayAnimationData {
        wake_animation: OverlayAnimation {
            start_idx: 0,
            end_idx: 19,
            speed: 0.05,
            z_index: 2.0,
        },
        overlay_animation_image_handle,
        overlay_animation_texture_atlas: OverlayAnimationTextureAtlas {
            texture_atlas_layout: overlay_animation_texture_atlas,
            index: 0,
        },
    });

    info!("OverlayAnimationData resource has been set up.");
}

pub fn attach_overlay_animations_to_flying_entities(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<FlyingEntityComponents>>,
    overlay_animation_data: Res<OverlayAnimationData>,
) {
    for (entity, transform) in query.iter() {
        let animation = overlay_animation_data.wake_animation; // Assume wake animation for now

        commands
            .entity(entity)
            // TODO: This next line is not right, you are messing up Component vs Resource, you need to only insert components
            .insert(animation)
            .with_children(|parent| {
                parent
                    .spawn(SpriteSheetBundle {
                        texture: overlay_animation_data
                            .overlay_animation_image_handle
                            .clone(),
                        atlas: TextureAtlas {
                            layout: overlay_animation_data
                                .overlay_animation_texture_atlas
                                .texture_atlas_layout
                                .clone(),
                            index: overlay_animation_data.overlay_animation_texture_atlas.index,
                            //TODO: go back to every z-index and starting index for a sprite sheet and
                            // make constants in lib.rs also remove index from overlay_animation_texture_atlas
                        },
                        transform: Transform::default(), // Relative to the parent
                        ..Default::default()
                    })
                    //TODO: THis should have always beena Texture atlas??
                    .insert(OverlayAnimationTextureAtlas {
                        texture_atlas_layout: overlay_animation_data
                            .overlay_animation_texture_atlas
                            .texture_atlas_layout
                            .clone(),
                        index: 0,
                    })
                    .insert(AnimationTimer(Timer::from_seconds(
                        animation.speed,
                        TimerMode::Repeating,
                    )))
                    .insert(Name::new("OverlayAnimation"))
                    .insert(GlobalTransform::default())
                    .insert(Visibility::default())
                    .insert(InheritedVisibility::default());
            });

        info!(
            "Attached overlay animation to Flying Entity {:?}: start_idx {}, end_idx {}, speed {}",
            entity, animation.start_idx, animation.end_idx, animation.speed
        );
    }
}
