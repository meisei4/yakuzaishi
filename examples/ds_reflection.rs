use bevy::{
    app::{App, FixedUpdate, PluginGroup, Update},
    math::Vec2,
    prelude::{
        in_state, AppExtStates, Commands, IntoSystemConfigs, OnEnter, OnExit, ParamSet, Query,
        Rectangle, Res, ResMut, States, Transform, Window, WindowPlugin, With,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    utils::default,
    window::WindowResolution,
    DefaultPlugins,
};
use bevy_asset::Assets;
use bevy_asset_loader::{
    loading_state::{LoadingState, LoadingStateAppExt},
    prelude::ConfigureLoadingState,
};
use bevy_render::{mesh::Mesh, prelude::ImagePlugin};
use yakuzaishi::{
    anime::anime_res::PlayerEntityAnimationAssets,
    camera::{
        camera_2d_sys::{bottom_camera, top_camera},
        camera_components::BottomCameraTag,
    },
    environment::moon::{place_moon, MoonAsset, MoonLightSource, MoonTag},
    kinetic_components::PlayerEntityTag,
    materials::reflections::ReflectionMaterial,
    player::player_sys::{control_player_entity, spawn_player_entity},
    NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            NINTENDO_DS_SCREEN_WIDTH,
                            NINTENDO_DS_SCREEN_HEIGHT,
                        ),
                        resizable: false,
                        title: "MICHAEL DOUGLAS".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            // WorldInspectorPlugin::new(),
            Material2dPlugin::<ReflectionMaterial>::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .continue_to_state(GameState::Run)
                .load_collection::<PlayerEntityAnimationAssets>()
                .load_collection::<MoonAsset>(),
        )
        .add_systems(
            OnExit(GameState::Load),
            (spawn_player_entity, setup_reflection_material, place_moon),
        )
        .add_systems(OnEnter(GameState::Run), (top_camera, bottom_camera))
        .add_systems(
            FixedUpdate,
            control_player_entity.run_if(in_state(GameState::Run)),
        )
        .add_systems(Update, track_camera_ds.run_if(in_state(GameState::Run)))
        .run();
}

fn setup_reflection_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ReflectionMaterial>>,
    moon_asset: Res<MoonAsset>,
    query: Query<&MoonLightSource, With<MoonTag>>,
) {
    if let Some(moonlight) = query.iter().next() {
        let moon_image = moon_asset.background.clone();
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(100.0, 100.0)).into(),
            material: materials.add(ReflectionMaterial {
                light_position: moonlight.position,
                light_intensity: moonlight.intensity,
                light_color: moonlight.color,
                texture: moon_image,
            }),
            transform: Transform::from_xyz(0.0, NINTENDO_DS_SCREEN_HEIGHT, 0.0),
            ..default()
        });
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Load,
    Run,
}

pub fn track_camera_ds(
    mut param_set: ParamSet<(
        Query<&Transform, With<PlayerEntityTag>>,
        Query<&mut Transform, With<BottomCameraTag>>,
    )>,
) {
    let mut player_position = Vec2::ZERO;
    if let Some(player_transform) = param_set.p0().iter().next() {
        player_position.x = player_transform.translation.x;
        player_position.y = player_transform.translation.y;
    }
    if let Some(mut camera_transform) = param_set.p1().iter_mut().next() {
        camera_transform.translation.x = player_position.x;
        camera_transform.translation.y = player_position.y;
    }
}
