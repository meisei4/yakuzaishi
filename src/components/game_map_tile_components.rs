use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Normal,
    Grass,
    Wall,
}

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

impl Component for GameMapTileComponents {
    type Storage = DenseVecStorage<Self>;
}
