use bevy::prelude::*;
use rand::prelude::*;

const TREE_PATH: &str = "test/pine.png";
const SHEEP_PATH: &str = "test/sheep.png";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 300.0, 0.0)
            // .looking_at(Vec3::ZERO, Vec3::Y)
            // .with_translation(Vec3::new(0.0, -10.0, -150.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    //green plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(200.0, 200.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.5, 0.0),
            ..default()
        }),
        ..default()
    });

    //spawn sun
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_xyz(10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         color: Color::WHITE,
    //         illuminance: 50000.0,
    //         ..default()
    //     },
    //     ..default()
    // });

    //ambient ligjt
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
    });

    let square = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0).into()));
    let sheep_texture: Handle<Image> = asset_server.load(SHEEP_PATH);
    let tree_texture: Handle<Image> = asset_server.load(TREE_PATH);

    let sheep_material = materials.add(StandardMaterial {
        base_color_texture: Some(sheep_texture),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    //spawn sheeps
    let r = 50.0;
    let mut rng = rand::thread_rng();
    let sheep_count = 200;

    for _ in 0..sheep_count {
        let x = rng.gen_range(-r..r);
        let z = rng.gen_range(-r..r);
        //let y = z;
        let pos = Vec3::new(x, 0.1, z);
        if pos.length() > r {
            continue;
        }

        commands.spawn(PbrBundle {
            mesh: square.clone(),
            material: sheep_material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z + 2.0)
                //.with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, PI, 0.0))
                .with_scale(Vec3::splat(5.0)),
            ..default()
        });
    }

    //spawn trees
    let tree_count = 500;
    let tree_material = materials.add(StandardMaterial {
        base_color_texture: Some(tree_texture),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.1,
        ..default()
    });

    let tree_r = 150.0;
    let cut_r = r + 20.0;

    for _ in 0..tree_count {
        let x = rng.gen_range(-tree_r..tree_r);
        let z = rng.gen_range(-tree_r..tree_r);
        // let y = z;
        let pos = Vec3::new(x, 0.1, z);
        if pos.length() < cut_r {
            continue;
        }

        commands.spawn(PbrBundle {
            mesh: square.clone(),
            material: tree_material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z + 4.0)
                //.with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, PI, 0.0))
                .with_scale(Vec3::new(5.0, 5.0, 10.0)),
            ..default()
        });
    }
}
