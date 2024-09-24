use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::prelude::{OnEnter, OnExit, States};

use crate::systems::animation_loadtime::attach_animations::{
    attach_animations_to_environment_entities, attach_animations_to_individual_tile_entities,
    attach_animations_to_player_entities, attach_base_textures_to_player_entities,
};
use crate::systems::animation_loadtime::insert_animations::{
    insert_overlay_animation_resources_into_world, insert_tile_animation_resources_into_world,
};
use crate::systems::animation_runtime::overlay_animations::animate_env_entity_animations;
use crate::systems::animation_runtime::tile_animations::{
    animate_overlapped_tiles_event_based, handle_overlap_event, TileAnimationEvent,
};
use crate::systems::audio_runtime::background_music::start_background_audio;
use crate::systems::entity_loadtime::camera::init_camera;
use crate::systems::entity_loadtime::environment_entity::spawn_environment_entity;
use crate::systems::entity_loadtime::player_entity::spawn_player_entity;
use crate::systems::entity_loadtime::tiled_map::{process_tiled_maps, spawn_tiled_map_entity};
use crate::systems::entity_runtime::camera::track_camera;
use crate::systems::entity_runtime::player_entity::control_player_entity;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    AssetProcessing,
    Load,
    Run,
}

pub struct LoadStatePlugin;

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Load),
            (
                spawn_tiled_map_entity, // TODO: I don't like the name of this because its spawning an asset dependant thing which i feel like should be called Load
                insert_tile_animation_resources_into_world,
                insert_overlay_animation_resources_into_world,
                spawn_player_entity,
                spawn_environment_entity,
                init_camera,
            ),
        )
        //TODO: check_assets_loaded needs to be improved to actually automatically check all generic asset_server
        // loading OnExit systems rely on
        .add_systems(
            OnExit(GameState::Load),
            (
                start_background_audio,
                process_tiled_maps, //TODO: REQUIRED OnExit
                attach_animations_to_individual_tile_entities, //TODO: REQUIRED OnExit
                attach_base_textures_to_player_entities, // TODO: REQUIRED OnExit
                attach_animations_to_player_entities, // TODO: REQUIRED OnExit
                attach_animations_to_environment_entities, // TODO: REQUIRED OnExit
            ),
        );
    }
}

pub struct RunStatePlugin;

impl Plugin for RunStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileAnimationEvent>()
            .add_systems(FixedUpdate, control_player_entity)
            .add_systems(
                Update,
                (
                    track_camera,
                    animate_overlapped_tiles_event_based,
                    handle_overlap_event,
                    // TODO: BIG TODO!!! this next line actually doesnt make sense
                    //  animate_env_entity_animations causes the player entity overlay to work and
                    //  in fact the animate_overlay_animations breaks the original animations...
                    //animate_overlay_animations,
                    animate_env_entity_animations,
                ),
            );
    }
}
