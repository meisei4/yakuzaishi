use bevy::prelude::{Camera2dBundle, Commands, Transform};

pub fn init_camera(mut command_buffer: Commands) {
    command_buffer.spawn(Camera2dBundle {
        // TODO: The transform is only there to serve z purpose (being above the z=0.0 map and z=1.0 car)
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..Default::default()
    });
}
