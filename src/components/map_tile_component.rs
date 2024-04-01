use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct MapTileComponent {
    pub is_drivable: bool,
}
