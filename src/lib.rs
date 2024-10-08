pub mod anime;
pub mod audio;
pub mod bundles;
pub mod camera;
pub mod environment;
pub mod kinetic_components;
pub mod map;
pub mod materials;
pub mod player;

//TODO: move all of these constants to their corresponding resource modules

//-----------------GAME_WORLD CONFIGS/SETTINGS-----------------
pub const TILE_SIZE: f32 = 64.0;
pub const NINTENDO_DS_SCREEN_WIDTH: f32 = 256.0 * 1.5;
pub const NINTENDO_DS_SCREEN_HEIGHT: f32 = 384.0 * 1.5;
pub const CAMERA_SCALE_MULTIPLIER: f32 = 1.0;

// TODO: for this whole entire const file figure out when to use proper types (f32 doesnt make sense for tile coordinates etc)
pub const CAMERA_Z_LEVEL: f32 = 3.0;
pub const OVERLAY_ANIMATIONS_Z_LEVEL: f32 = 2.0; // TODO: currently overlay Transform inherits from Parent Entity it is attached to, use later
pub const PLAYER_ENTITY_Z_LEVEL: f32 = 1.0;
pub const ENVIRONMENT_ENTITY_Z_LEVEL: f32 = 1.0;

//-----------------ASSET CONFIGS/SETTINGS-----------------

pub const PLAYER_ENTITY_TEXTURE_FILE_PATH: &str = "sprite_data/iruka.png";
pub const PLAYER_ENTITY__ANIMATION_TEXTURE_COLUMN_LENGTH: usize = 1;
pub const PLAYER_ENTITY__ANIMATION_TEXTURE_ROW_LENGTH: usize = 1;
pub const PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX: usize = 0;
pub const PLAYER_ENTITY_ANIMATION_TEXTURE_END_IDX: usize = 0;
pub const PLAYER_ENTITY_ANIMATION_SPEED: f32 = 0.05;

pub const WAKE_ANIMATION_FILE_PATH: &str = "sprite_data/random_test_animations.png";
pub const WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH: u32 = 20;
pub const WAKE_ANIMATION_TEXTURE_ROW_LENGTH: u32 = 1;
pub const WAKE_ANIMATION_TEXTURE_START_IDX: u32 = 0;
pub const WAKE_ANIMATION_TEXTURE_END_IDX: u32 = 19;
pub const WAKE_ANIMATION_SPEED: f32 = 0.05;

pub const TILE_MAP_FILE_PATH: &str = "map_data/water.tmx";
pub const TILE_ANIMATION_TEXTURE_START_IDX: u32 = 40;
pub const TILE_ANIMATION_TEXTURE_END_IDX: u32 = 54;
pub const TILE_ANIMATION_SPEED: f32 = 0.1;

pub const ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_FILE_PATH: &str = "sprite_data/Ikiikiiruka.png";
pub const ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH: u32 = 8;
pub const ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH: u32 = 3;
pub const ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX: usize = 0;
pub const ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX: u32 = 18;
pub const ENVIRONMENT_ENTITY_ANIMATION_SPEED: f32 = 0.2;

//-----------------ENTITY/GAME LOGIC-----------------
pub const DEFAULT_SPEED: f32 = 150.0;
pub const PLAYER_ENTITY_SPAWN_X: f32 = 0.0;
pub const PLAYER_ENTITY_SPAWN_Y: f32 = 0.0;

pub const ENVIRONMENT_ENTITY_SPAWN_X: f32 = 3.0;
pub const ENVIRONMENT_ENTITY_SPAWN_Y: f32 = 3.0;
