use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// TODO fix IDE to not presume prelude? (in order to learn where modules truly reside
use bevy::prelude::*;
use bevy::{
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResolution,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
    prelude::ConfigureLoadingState,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use yakuzaishi::{
    materials::mode7::Mode7Material, NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH,
};

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
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(500.0, 500.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            //WorldInspectorPlugin::new(),
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
        //TODO: This mesh is very confusing why does it matter
        mesh: meshes.add(Rectangle::new(10.0, 10.0)).into(),
        transform: Transform::default(),
        material: materials.add(Mode7Material {
            //TODO: see line 62, these affect eachother too much
            scaling: Vec2::new(100.0, 100.0),
            // TODO: only this x-tilt allows for the y-axis rotation to feel like its occuring around the player/camera
            fov: PI / 4.0,
            frustrum_x_rotation: PI / 4.0,
            y_axis_rotation: PI,
            translation: Vec2::new(-9.0, 7.0), // mario_circuit starting zone
            altitude: 400.0, //TODO: this is ridiculous in how it results in bending the whole plane when lower
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
    let y_rotate_speed = PI; // Radians per second
    let x_rotate_speed = FRAC_PI_2; // Radians per second
    let fov_speed = PI / 15.0; // Units per second
    let y_altitude_speed = 2.0;

    let delta = time.delta_seconds();

    for (_, material) in materials.iter_mut() {
        // Rotation
        if keyboard_input.pressed(KeyCode::KeyD) {
            material.y_axis_rotation += y_rotate_speed * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            material.y_axis_rotation -= y_rotate_speed * delta;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            material.frustrum_x_rotation += x_rotate_speed * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            material.frustrum_x_rotation -= x_rotate_speed * delta;
        }

        // Altitude/Y-Axis angle?
        if keyboard_input.pressed(KeyCode::KeyU) {
            material.fov = material.fov + fov_speed * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyJ) {
            material.fov = material.fov - fov_speed * delta;
        }

        if keyboard_input.pressed(KeyCode::KeyI) {
            material.altitude += y_altitude_speed;
        }
        if keyboard_input.pressed(KeyCode::KeyK) {
            material.altitude -= y_altitude_speed;
        }

        // Move Forward
        if keyboard_input.pressed(KeyCode::Space) {
            let dx = move_speed * delta * material.y_axis_rotation.sin();
            let dy = move_speed * delta * material.y_axis_rotation.cos();
            material.translation.x += dx;
            material.translation.y += dy;
        }

        if keyboard_input.pressed(KeyCode::KeyB) {
            let dx = move_speed * delta * material.y_axis_rotation.sin();
            let dy = move_speed * delta * material.y_axis_rotation.cos();
            material.translation.x -= dx;
            material.translation.y -= dy;
        }

        info!(
            "Player Position - X: {:.2}, Z: {:.2}",
            material.translation.x, material.translation.y
        );
        info!("fov: {:.2}", material.fov);
        info!("Rotation Y (Yaw): {:.2} radians", material.y_axis_rotation);
        info!(
            "Rotation X (Pitch): {:.2} radians",
            material.frustrum_x_rotation
        );
    }
}
