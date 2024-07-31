use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{
    Assets, AssetServer, Commands, Entity, GlobalTransform, InheritedVisibility, Query, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, Vec2, Visibility, With,
};

use crate::{OVERLAY_ANIMATION_TEXTURE_START_IDX, TILE_SIZE, VEHICLE_TEXTURE_FILE_PATH};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::components::entity_movement_states::{CurrentMovementState, PreviousMovementState};
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
    let tile_spawn_coordinates = Vec2 { x: 0.0, y: 0.0 };
    let world_spawn_coordinates = Vec2 {
        x: tile_spawn_coordinates.x * TILE_SIZE,
        y: tile_spawn_coordinates.y * TILE_SIZE,
    };

    let transform = Transform::from_xyz(world_spawn_coordinates.x, world_spawn_coordinates.y, 1.0);
    let current_motion = CurrentMovementState {
        position: Vec3 {
            x: world_spawn_coordinates.x,
            y: world_spawn_coordinates.y,
            z: 1.0,
        },

        movement: Default::default(),
    };
    let old_motion = PreviousMovementState {
        position: Default::default(),
    };
    commands
        .spawn((
            ControllableEntityComponents::new(tile_spawn_coordinates),
            transform,
            current_motion,
            old_motion,
        ))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(InheritedVisibility::default())
        .insert(Name::new("Flying Entity"));
}

pub fn attach_controlled_animations_to_controllable_entities(
    mut commands: Commands,
    vehicle_animation_resource: Res<ControlledAnimationResource>,
    query: Query<Entity, With<ControllableEntityComponents>>,
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
                index: OVERLAY_ANIMATION_TEXTURE_START_IDX,
            },
            ..Default::default()
        });
    }
}
