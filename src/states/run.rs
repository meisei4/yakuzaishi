use bevy::app::{App, Plugin, Update};

use crate::events::tile_animation::TileAnimationEvent;
use crate::systems::run_state::camera_tracker::camera_tracking_system;
use crate::systems::run_state::flying_entity_controller::{
    apply_motion_states_system, update_motion_states_system,
};
use crate::systems::run_state::rotation_vehicle_controller::rotation_vehicle_controller_system;
use crate::systems::run_state::run_animations::animate_overlapped_tile_continuous;

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileAnimationEvent>()
            //.add_systems(FixedUpdate, (update_motion_states_system))
            .add_systems(
                Update,
                (
                    camera_tracking_system,
                    rotation_vehicle_controller_system,
                    update_motion_states_system,
                    apply_motion_states_system,
                    // For continuous:
                    animate_overlapped_tile_continuous,
                    // for event based:
                    //animate_overlapped_tile_event_based,
                    // handle_overlap_event,
                ),
            );
    }
}
