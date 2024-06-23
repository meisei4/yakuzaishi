use std::env;
use std::path::PathBuf;

pub mod components;
pub mod states;
pub mod systems;

lazy_static::lazy_static! {
    pub static ref ASSETS_BASE_PATH: PathBuf = env::current_dir().unwrap().join("assets");
}

pub const VEHICLE_SPRITE_SHEET_FILE_PATH: &str = "assets/sprite_data/vehicle_spritesheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &str = "sprite_data/vehicle_spritesheet.png";

pub const DEFAULT_SPEED: f32 = 128.0;
pub const DEFAULT_RATE_OF_ROTATION: f32 = 4.0;

pub const OCEAN_MAP_FILE_PATH: &str = "map_data/ocean_animated.tmx";

pub const TILE_SIZE: f32 = 64.0;

pub const NINTENDO_DS_SCREEN_WIDTH: f32 = 256.0;
pub const NINTENDO_DS_SCREEN_HEIGHT: f32 = 384.0;
