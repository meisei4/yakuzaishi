use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::components::animated_tile::AnimatedTile;

#[derive(Resource)]
pub struct AnimationData {
    pub(crate) animations: HashMap<u32, AnimatedTile>,
}
