use bevy::core::Name;
use bevy::prelude::{
    Assets, Commands, GlobalTransform, InheritedVisibility, Res, ResMut, TextureAtlasLayout,
    Transform, Vec2, Visibility,
};

use crate::{PLAYER_ENTITY_SPAWN_X, PLAYER_ENTITY_SPAWN_Y, PLAYER_ENTITY_Z_LEVEL, TILE_SIZE};
use crate::components::kinetic_entity::{KineticEntityComponents, PlayerEntityTag};
use crate::resources::animation::{PlayerEntityAnimationAssets, PlayerEntityAnimationResource};

pub fn spawn_player_entity(
    mut commands: Commands,
    player_assets: Res<PlayerEntityAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let vehicle_animation_image_handle = player_assets.image_handle.clone();

    let vehicle_texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        1,
        1,
        None,
        None,
    ));

    commands.insert_resource(PlayerEntityAnimationResource {
        image_handle: vehicle_animation_image_handle,
        texture_atlas: vehicle_texture_atlas_layout,
    });

    // TODO: 0,0 should not be bottom left
    //  anymore it should be top left
    //  review CRT scanline order and latin writing conventions (japan didn't invent the computer)

    let transform = Transform::from_xyz(
        PLAYER_ENTITY_SPAWN_X * TILE_SIZE,
        PLAYER_ENTITY_SPAWN_Y * TILE_SIZE,
        PLAYER_ENTITY_Z_LEVEL,
    );

    let player_entity = KineticEntityComponents {
        x_axis_displacement: 0.0,
        y_axis_displacement: 0.0,
        position: transform.translation,
        prev_position: transform.translation,
    };

    commands
        .spawn((player_entity, transform))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(PlayerEntityTag)
        .insert(Name::new("Player Entity"));
}
