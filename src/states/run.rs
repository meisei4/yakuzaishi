use bevy::app::{App, Plugin, Update};

use crate::events::tile_animation::TileAnimationEvent;
use crate::systems::run_state::camera_tracker::camera_tracking_system;
use crate::systems::run_state::flying_entity_controller::flying_entity_controller_system;
use crate::systems::run_state::rotation_vehicle_controller::rotation_vehicle_controller_system;
use crate::systems::run_state::run_animations::animate_overlapped_tile_continuous;

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileAnimationEvent>().add_systems(
            Update,
            (
                camera_tracking_system,
                flying_entity_controller_system,
                rotation_vehicle_controller_system,
                // For continuous:
                animate_overlapped_tile_continuous,
                // for event based:
                //animate_overlapped_tile_event_based,
                // handle_overlap_event,
            ),
        );
    }
}
