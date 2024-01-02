use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct GameMapTileComponents {
    pub is_drivable: bool,
    // pub x: i32, y: i32, (use as a hashmap instead during usage)
}

impl GameMapTileComponents {
    pub fn new(is_drivable: bool) -> Self {
        GameMapTileComponents { is_drivable }
    }
}

impl Component for GameMapTileComponents {
    type Storage = DenseVecStorage<Self>;
}
