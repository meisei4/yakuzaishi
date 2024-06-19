use bevy::app::{App, Plugin, Update};

use crate::update_systems::camera_tracker::camera_tracking_system;
use crate::update_systems::flying_entity_controller::vehicle_controller_system;

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_tracking_system, vehicle_controller_system));
    }
}
