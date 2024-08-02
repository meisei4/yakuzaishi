use bevy::core::Name;
use bevy::prelude::{
    Assets, AssetServer, Commands, Entity, GlobalTransform, InheritedVisibility, Query, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, Vec2, Visibility, With,
};

use crate::{
    CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX, CONTROLLABLE_ENTITY_Z_LEVEL,
    DEFAULT_SPAWN_TILE_X, DEFAULT_SPAWN_TILE_Y, TILE_SIZE, VEHICLE_TEXTURE_FILE_PATH,
};
use crate::components::controllable_entity_components::{
    PositionComponent, PreviousPositionComponent, VelocityVectorComponents,
};
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
        controlled_animation_image_handle: vehicle_animation_image_handle,
        controlled_animation_texture_atlas: vehicle_texture_atlas_layout,
    });
    // TODO: 0,0 should not be bottom left
    //  anymore it should be top left
    //  review CRT scanline order and latin writing conventions (japan didn't invent the computer)
    let transform = Transform::from_xyz(
        DEFAULT_SPAWN_TILE_X,
        DEFAULT_SPAWN_TILE_Y,
        CONTROLLABLE_ENTITY_Z_LEVEL,
    );
    let current_motion = PositionComponent {
        position: transform.translation,
    };
    let old_motion = PreviousPositionComponent {
        position: transform.translation,
    };
    commands
        .spawn((
            VelocityVectorComponents {
                x_axis_displacement: 0.0,
                y_axis_displacement: 0.0,
            },
            transform,
            current_motion,
            old_motion,
        ))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(Name::new("Controllable Entity"));
}

pub fn attach_controlled_animations_to_controllable_entities(
    mut commands: Commands,
    vehicle_animation_resource: Res<ControlledAnimationResource>,
    query: Query<Entity, With<VelocityVectorComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture: vehicle_animation_resource
                .controlled_animation_image_handle
                .clone(),
            atlas: TextureAtlas {
                layout: vehicle_animation_resource
                    .controlled_animation_texture_atlas
                    .clone(),
                index: CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX,
            },
            ..Default::default()
        });
    }
}
