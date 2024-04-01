use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Normal,
}


#[derive(Component, Clone)]
pub struct GameMapTileComponents {
    pub is_drivable: bool,
    pub tile_type: TileType,
}

impl GameMapTileComponents {
    pub fn new(is_drivable: bool, tile_type: TileType) -> Self {
        GameMapTileComponents {
            is_drivable,
            tile_type,
        }
    }
}
