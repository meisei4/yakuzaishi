use bevy::math::Quat;
use bevy::prelude::{Camera3dBundle, Commands, default, Transform};

pub fn init_camera_3d(mut command_buffer: Commands) {
    command_buffer.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2000.0, 0.0)
            // .looking_at(Vec3::ZERO, Vec3::Y)
            // .with_translation(Vec3::new(0.0, -10.0, -150.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}
