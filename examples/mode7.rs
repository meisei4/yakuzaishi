// main.rs or your main setup file

use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use yakuzaishi::materials::mode7::Mode7Material;

#[derive(Component, Default)]
pub struct TopCameraTag;

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
        .add_systems(
            Update,
            update_time_on_shader.run_if(in_state(GameState::Run)),
        )
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
            scaling: Vec2::new(100.0, 150.0),
            rotation: 0.0,
            // rotation: std::f32::consts::FRAC_PI_4, // Rotate by 45 degrees,
            translation: Vec2::ZERO, // Ensure translation starts at (0, 0)
            altitude: 0.001,
            time: 0.0,
            _padding: Vec3::ZERO, // Can be set to zero
            map_texture: map_image,
        }),
        ..default()
    });
}
pub fn update_time_on_shader(time: Res<Time>, mut materials: ResMut<Assets<Mode7Material>>) {
    for (_, material) in materials.iter_mut() {
        material.time += time.delta_seconds();
        material.rotation = f32::sin(material.time * 0.3);
    }
}
