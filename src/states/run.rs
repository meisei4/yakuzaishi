use bevy::app::{App, FixedUpdate, Plugin, Update};

use crate::events::tile_animation::TileAnimationEvent;
use crate::systems::run_state::animation::overlay_animation_update_system::animate_overlay_animations;
use crate::systems::run_state::animation::tile_animation_update_system::{
    animate_overlapped_tile_event_based, handle_overlap_event,
};
use crate::systems::run_state::camera_tracking_system::camera_tracking_system;
use crate::systems::run_state::controllable_entity_input_system::{
    apply_entity_movement_states_system, update_entity_movement_states_system,
};

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileAnimationEvent>()
            .add_systems(
                FixedUpdate,
                (
                    update_entity_movement_states_system,
                    apply_entity_movement_states_system,
                ),
            )
            .add_systems(
                Update,
                (
                    camera_tracking_system,
                    apply_entity_movement_states_system,
                    // For continuous:
                    // animate_overlapped_tile_continuous,
                    // for event based:
                    animate_overlapped_tile_event_based,
                    handle_overlap_event,
                    animate_overlay_animations,
                ),
            );
    }
}
