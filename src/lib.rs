pub mod components;
pub mod enums;
pub mod resources;
pub mod state;
pub mod systems;
pub mod util;
pub mod command_buffer;

// asset file locations

pub const VEHICLE_SPRITE_SHEET_FILE_PATH: &'static str = "sprite_data/vehicle_spritesheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &'static str = "sprite_data/vehicle_spritesheet.png";

pub const PEDESTRIAN_SPRITE_SHEET_FILE_PATH: &'static str =
    "sprite_data/racoon_walk_spritesheet.ron";
pub const PEDESTRIAN_TEXTURE_FILE_PATH: &'static str = "sprite_data/racoon_walk_spritesheet.png";

pub const MAP_FILE_PATH: &'static str = "assets/map_data/road_tilemap_1.tmx";
pub const TILESET_FILE_PATH: &'static str = "assets/map_data/road_tileset.tsx";

pub const FONT_PATH: &'static str = "font_data/saturn.ttf";

pub const TILESET_TEXTURE_FILE_PATH: &'static str = "map_data/tileset_texture.png";

pub const DISPLAY_CONFIG_FILENAME: &'static str = "display_config.ron";
pub const VEHICLE_BINDINGS_CONFIG_FILENAME: &'static str = "key_bindings/vehicle_bindings.ron";
pub const PEDESTRIAN_BINDINGS_CONFIG_FILENAME: &'static str = "key_bindings/vehicle_bindings.ron";
pub const MENU_BINDINGS_CONFIG_FILENAME: &'static str = "key_bindings/menu_bindings.ron";

// GEOMETRY constants
pub const TILE_SIZE: f32 = 64.0;
pub const MAP_HEIGHT: f32 = 10.0;
// unit is tiles
pub const MAP_WIDTH: f32 = 10.0;
pub const CAMERA_WIDTH: f32 = 200.0;
pub const CAMERA_HEIGHT: f32 = 200.0;
