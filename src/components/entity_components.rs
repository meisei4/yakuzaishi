use crate::components::pedestrian_components::PedestrianComponents;
use crate::components::vehicle_components::VehicleComponents;
use amethyst::ecs::{Component, DenseVecStorage};

// TODO: at some point this is supposed to allow for centralized behavior in
// the controller systems, but i really cant figure out how to mimic pseudo-inheritance in rust
pub enum EntityComponentEnum {
    Vehicle(VehicleComponents),
    Pedestrian(PedestrianComponents),
}

impl Component for EntityComponentEnum {
    type Storage = DenseVecStorage<Self>;
}
