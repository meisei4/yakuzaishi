use bevy::core::Name;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{
    Assets, Commands, Fixed, KeyCode, Query, Res, ResMut, TextureAtlasLayout, Time, Transform,
    Vec2, With,
};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};

use crate::{
    DEFAULT_SPEED, PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX, PLAYER_ENTITY_SPAWN_X,
    PLAYER_ENTITY_SPAWN_Y, PLAYER_ENTITY_Z_LEVEL, TILE_SIZE,
};
use crate::anime::anime_res::PlayerEntityAnimationAssets;
use crate::bundles::PlayerBundle;
use crate::kinetic_components::{KineticEntityComponents, PlayerEntityTag};

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

    let texture_atlas = TextureAtlas {
        layout: vehicle_texture_atlas_layout.clone(),
        index: PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
    };

    // TODO: 0,0 should not be bottom left
    //  anymore it should be top left
    //  review CRT scanline order and latin writing conventions (japan didn't invent the computer)

    let transform = Transform::from_xyz(
        PLAYER_ENTITY_SPAWN_X * TILE_SIZE,
        PLAYER_ENTITY_SPAWN_Y * TILE_SIZE,
        PLAYER_ENTITY_Z_LEVEL,
    );

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture: vehicle_animation_image_handle.clone(),
        atlas: texture_atlas,
        transform,
        ..Default::default()
    };

    let player_kinetics = KineticEntityComponents {
        x_axis_displacement: 0.0,
        y_axis_displacement: 0.0,
        position: transform.translation,
        prev_position: transform.translation,
    };

    commands
        .spawn(PlayerBundle {
            name: Name::new("Player Entity"),
            kinetics: player_kinetics,
            sprite_sheet: sprite_sheet_bundle,
        })
        .insert(PlayerEntityTag);
}

pub fn control_player_entity(
    fixed_time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut KineticEntityComponents), With<PlayerEntityTag>>,
) {
    for (mut transform, mut player_entity) in query.iter_mut() {
        let a = fixed_time.overstep_fraction();
        transform.translation = player_entity.prev_position.lerp(player_entity.position, a);
        process_input(&keyboard_input, &mut player_entity);
        player_entity.prev_position = player_entity.position;
        let position_displacement = Vec3 {
            x: player_entity.x_axis_displacement,
            y: player_entity.y_axis_displacement,
            z: 0.0, // TODO: Ugly, but only because the Vec3 is required to fit with the implementation
        };
        player_entity.position += position_displacement * fixed_time.delta_seconds();
    }
}

fn process_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    player_entity: &mut KineticEntityComponents,
) {
    handle_y_axis_movement(keyboard_input, player_entity);
    handle_x_axis_movement(keyboard_input, player_entity);
}

fn handle_y_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut KineticEntityComponents,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        vehicle_component.y_axis_displacement = DEFAULT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        vehicle_component.y_axis_displacement = -DEFAULT_SPEED;
    } else {
        vehicle_component.y_axis_displacement = 0.0;
    }
}

fn handle_x_axis_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    vehicle_component: &mut KineticEntityComponents,
) {
    let strafe_right = keyboard_input.pressed(KeyCode::KeyD) as i32;
    let strafe_left = keyboard_input.pressed(KeyCode::KeyA) as i32;
    let strafe_direction = (strafe_right - strafe_left) as f32; // 1 if D is pressed, -1 if A is pressed, 0 otherwise

    vehicle_component.x_axis_displacement = DEFAULT_SPEED * strafe_direction;
}
