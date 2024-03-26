use amethyst::ecs::prelude::{Component, DenseVecStorage};

// Define the TileType enumeration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Normal,
    Grass,
    Wall,
}

pub struct GameMapTileComponents {
    pub is_drivable: bool,
    pub tile_type: TileType, // Add the TileType field to your component
}

impl GameMapTileComponents {
    // Adjust the constructor to accept a TileType parameter
    pub fn new(is_drivable: bool, tile_type: TileType) -> Self {
        GameMapTileComponents {
            is_drivable,
            tile_type,
        }
    }
}

impl Component for GameMapTileComponents {
    type Storage = DenseVecStorage<Self>;
}
