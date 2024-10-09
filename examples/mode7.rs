// main.rs or your main setup file

use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use yakuzaishi::materials::mode7::Mode7Material;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Load,
    Run,
}

#[derive(AssetCollection, Resource)]
pub struct Mode7Asset {
    #[asset(path = "map_data/mariocircuit-1.png")]
    pub image: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            Material2dPlugin::<Mode7Material>::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .continue_to_state(GameState::Run)
                .load_collection::<Mode7Asset>(),
        )
        .add_systems(OnEnter(GameState::Run), setup)
        .add_systems(Update, process_input.run_if(in_state(GameState::Run)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Mode7Material>>,
    mode7_asset: Res<Mode7Asset>,
) {
    commands.spawn(Camera2dBundle::default());

    let map_image = mode7_asset.image.clone();

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(1024.0, 1024.0)).into(),
        transform: Transform::default(),
        material: materials.add(Mode7Material {
            scaling: Vec2::new(100.0, 100.0),
            rotation: 0.0,
            translation: Vec2::ZERO, // Ensure translation starts at (0, 0)
            altitude: 0.0,
            _padding: Vec3::ZERO, // Can be set to zero
            map_texture: map_image,
        }),
        ..default()
    });
}

fn process_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<Mode7Material>>,
    time: Res<Time>,
) {
    let move_speed = 2.5; // Units per second
    let rotate_speed = std::f32::consts::PI; // Radians per second
    let altitude_speed = 1000.0; // Units per second

    let delta = time.delta_seconds();

    for (_, material) in materials.iter_mut() {
        // Rotation
        if keyboard_input.pressed(KeyCode::KeyD) {
            material.rotation += rotate_speed * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            material.rotation -= rotate_speed * delta;
        }

        // Altitude/Y-Axis angle?
        if keyboard_input.pressed(KeyCode::KeyW) {
            material.altitude += altitude_speed * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            material.altitude -= altitude_speed * delta;
        }

        // Move Forward
        if keyboard_input.pressed(KeyCode::Space) {
            let dx = move_speed * delta * material.rotation.sin();
            let dy = move_speed * delta * material.rotation.cos();
            material.translation.x += dx;
            material.translation.y += dy;
        }
    }
}
