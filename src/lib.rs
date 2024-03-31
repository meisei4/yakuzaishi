pub mod components;
pub mod startup;
pub mod systems;

// asset file locations

pub const VEHICLE_SPRITE_SHEET_FILE_PATH: &'static str = "assets/sprite_data/vehicle_spritesheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &'static str = "sprite_data/vehicle_spritesheet.png";

pub const MAP_FILE_PATH: &'static str = "assets/map_data/road_tilemap_1.tmx";
pub const TILESET_FILE_PATH: &'static str = "assets/map_data/road_tileset.tsx";

pub const FONT_PATH: &'static str = "font_data/saturn.ttf";

pub const TILESET_TEXTURE_FILE_PATH: &'static str = "map_data/tileset_texture.png";

// GEOMETRY constants
pub const TILE_SIZE: f32 = 64.0;
pub const MAP_HEIGHT: f32 = 10.0;
// unit is tiles
pub const MAP_WIDTH: f32 = 10.0;
pub const CAMERA_WIDTH: f32 = 200.0;
pub const CAMERA_HEIGHT: f32 = 200.0;

pub const VEHICLE_MAX_WIDTH: f32 = 8.0;
// Max width from the sprites (check vehicle_spritesheet.ron)
pub const VEHICLE_MAX_HEIGHT: f32 = 15.0; // Max height from the sprites
