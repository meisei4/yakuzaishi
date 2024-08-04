use bevy::app::{App, FixedUpdate, Plugin, Update};

use crate::events::tile_animation::TileAnimationEvent;
use crate::systems::run_state::animation::tile_animation_update_system::{
    animate_overlapped_tile_event_based, handle_overlap_event,
};
use crate::systems::run_state::camera_tracking_system::camera_tracking_system;
use crate::systems::run_state::controllable_entity_input_system::control_entity_position_smooth;
use crate::systems::run_state::environmental_entity_behavior_system::animate_env_entity_animations;

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileAnimationEvent>()
            .add_systems(FixedUpdate, control_entity_position_smooth)
            .add_systems(
                Update,
                (
                    camera_tracking_system,
                    animate_overlapped_tile_event_based,
                    handle_overlap_event,
                    //animate_overlay_animations,
                    animate_env_entity_animations,
                ),
            );
    }
}
