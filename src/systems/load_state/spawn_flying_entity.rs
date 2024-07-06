use std::fs;

use bevy::asset::ron;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{
    Assets, AssetServer, Commands, Rect, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasLayout, Transform, Vec2,
};
use serde::Deserialize;

use crate::{TILE_SIZE, VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH};
use crate::components::entity_movement_states::{CurrentMovementState, PreviousMovementState};
use crate::components::flying_entity_components::FlyingEntityComponents;

#[derive(Deserialize)]
struct SpriteSheetSpec {
    texture_width: f32,
    texture_height: f32,
    sprites: Vec<SpriteSpec>,
}

#[derive(Deserialize)]
struct SpriteSpec {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    // offsets: (f32, f32), TODO: not sure why these exist in the sprites .ron file
}

pub fn spawn_vehicle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sprite_sheet_spec = load_sprite_sheet_spec_from_file(VEHICLE_SPRITE_SHEET_FILE_PATH);
    let vehicle_texture_atlas_layout = create_texture_atlas(sprite_sheet_spec);

    let texture_atlas_layout_handle = texture_atlas_layouts.add(vehicle_texture_atlas_layout);

    let tile_spawn_coordinates = Vec2 { x: 0.0, y: 0.0 }; // TODO: figure out some logic to choose spawn?
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
            FlyingEntityComponents::new(tile_spawn_coordinates),
            SpriteSheetBundle {
                texture: asset_server.load(VEHICLE_TEXTURE_FILE_PATH),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index: 0,
                },
                transform,
                ..Default::default()
            },
            current_motion,
            old_motion,
        ))
        .insert(Name::new("Flying Entity"));
}

fn load_sprite_sheet_spec_from_file(file_path: &str) -> SpriteSheetSpec {
    let ron_data = fs::read_to_string(file_path).expect("Failed to read RON file");
    ron::from_str(&ron_data).expect("Failed to deserialize RON data")
}

fn create_texture_atlas(sprite_sheet_spec: SpriteSheetSpec) -> TextureAtlasLayout {
    let mut texture_atlas_layout = TextureAtlasLayout::new_empty(Vec2 {
        x: sprite_sheet_spec.texture_width,
        y: sprite_sheet_spec.texture_height,
    });
    for sprite in &sprite_sheet_spec.sprites {
        let rect = Rect {
            min: Vec2 {
                x: sprite.x,
                y: sprite.y,
            },
            max: Vec2 {
                x: sprite.x + sprite.width,
                y: sprite.y + sprite.height,
            },
        };
        texture_atlas_layout.add_texture(rect);
    }
    texture_atlas_layout
}
