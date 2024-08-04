use bevy::core::Name;
use bevy::prelude::{
    Assets, AssetServer, Commands, GlobalTransform, InheritedVisibility, Res, ResMut,
    TextureAtlasLayout, Transform, Vec2, Visibility,
};

use crate::{
    CONTROLLABLE_ENTITY_Z_LEVEL, DEFAULT_SPAWN_TILE_X, DEFAULT_SPAWN_TILE_Y, TILE_SIZE,
    VEHICLE_TEXTURE_FILE_PATH,
};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::resources::animation_resources::ControlledAnimationResource;

pub fn spawn_controllable_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let vehicle_animation_image_handle = asset_server.load(VEHICLE_TEXTURE_FILE_PATH);

    let vehicle_texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        1,
        1,
        None,
        None,
    ));

    commands.insert_resource(ControlledAnimationResource {
        image_handle: vehicle_animation_image_handle,
        texture_atlas: vehicle_texture_atlas_layout,
    });

    // TODO: 0,0 should not be bottom left
    //  anymore it should be top left
    //  review CRT scanline order and latin writing conventions (japan didn't invent the computer)

    let transform = Transform::from_xyz(
        DEFAULT_SPAWN_TILE_X * TILE_SIZE,
        DEFAULT_SPAWN_TILE_Y * TILE_SIZE,
        CONTROLLABLE_ENTITY_Z_LEVEL,
    );

    let controllable_entity = ControllableEntityComponents {
        x_axis_displacement: 0.0,
        position: transform.translation,
        y_axis_displacement: 0.0,
        prev_position: transform.translation,
    };

    commands
        .spawn((controllable_entity, transform))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(Name::new("Controllable Entity"));
}
