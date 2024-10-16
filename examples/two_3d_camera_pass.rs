//! Renders two 3d passes to the same window from different perspectives.

use bevy::{
    app::App,
    color::Color,
    core::Name,
    math::{UVec2, Vec2, Vec3},
    pbr::{AmbientLight, PbrBundle, StandardMaterial},
    prelude::{
        AppExtStates, Camera2dBundle, Camera3dBundle, Commands, Cuboid, OnEnter, Plane3d, Res,
        ResMut, SpriteBundle, States, TextureAtlas, TextureAtlasLayout, Transform,
    },
    utils::default,
    DefaultPlugins,
};
use bevy_asset::Assets;
use bevy_asset_loader::{
    loading_state::{LoadingState, LoadingStateAppExt},
    prelude::ConfigureLoadingState,
};
use bevy_render::{
    camera::{Camera, ClearColorConfig},
    mesh::Mesh,
};
use yakuzaishi::{
    anime::anime_res::PlayerEntityAnimationAssets,
    bundles::PlayerBundle,
    kinetic_components::{KineticEntityComponents, PlayerEntityTag},
    PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX, PLAYER_ENTITY_SPAWN_X, PLAYER_ENTITY_SPAWN_Y,
    PLAYER_ENTITY_Z_LEVEL, TILE_SIZE,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .continue_to_state(GameState::Run)
                .load_collection::<PlayerEntityAnimationAssets>(),
        )
        .add_systems(OnEnter(GameState::Run), setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_assets: Res<PlayerEntityAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // plane3d
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5.0, 5.0))),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cuboid
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(1.0),
        })),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     camera: Camera {
    //         // clear_color: ClearColorConfig::None, // TODO: this is the important thing that doesnt erase the previous render
    //         order: 1,
    //         ..default()
    //     },
    //     ..default()
    // });

    let vehicle_animation_image_handle = player_assets.image_handle.clone();

    let vehicle_texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE as u32),
        1,
        1,
        None,
        None,
    ));

    let texture_atlas = TextureAtlas {
        layout: vehicle_texture_atlas_layout.clone(),
        index: PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
    };

    let transform = Transform::from_xyz(
        (PLAYER_ENTITY_SPAWN_X * TILE_SIZE) as f32,
        (PLAYER_ENTITY_SPAWN_Y * TILE_SIZE) as f32,
        PLAYER_ENTITY_Z_LEVEL,
    );

    let sprite_sheet = SpriteBundle {
        texture: vehicle_animation_image_handle.clone(),
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
            sprite_sheet,
            texture_atlas,
        })
        .insert(PlayerEntityTag);

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Run,
    Load,
}
