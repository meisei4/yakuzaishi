use std::collections::HashMap;
use bevy::prelude::{Component, Entity};

#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}