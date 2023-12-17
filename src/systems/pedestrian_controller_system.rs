use crate::components::pedestrian_components::{PedestrianComponents, WalkingDirection};
use crate::systems::controller_system_util::update_transform;
use amethyst::{
    core::{
        math::{ArrayStorage, Matrix, Vector2, U1, U2},
        timing::Time,
        Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct PedestrianControllerSystem;

impl<'s> System<'s> for PedestrianControllerSystem {
    type SystemData = (
        WriteStorage<'s, PedestrianComponents>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut pedestrian_components, mut transforms, mut sprite_renders, input, time): Self::SystemData,
    ) {
        let delta_time: f32 = time.delta_seconds();

        for (pedestrian_component, transform, sprite_render) in (
            &mut pedestrian_components,
            &mut transforms,
            &mut sprite_renders,
        )
            .join()
        {
            process_input(&input, pedestrian_component);
            update_position(pedestrian_component, delta_time);
            update_transform(&pedestrian_component.base, transform);
            sprite_render.sprite_number = update_sprite_index(pedestrian_component);
        }
    }
}

fn process_input(
    input: &Read<InputHandler<StringBindings>>,
    pedestrian: &mut PedestrianComponents,
) {
    let walk_north: bool = input.action_is_down("walk_north").unwrap_or(false);
    let walk_east: bool = input.action_is_down("walk_east").unwrap_or(false);
    let walk_south: bool = input.action_is_down("walk_south").unwrap_or(false);
    let walk_west: bool = input.action_is_down("walk_west").unwrap_or(false);

    let current_direction: WalkingDirection = pedestrian.direction.clone();
    pedestrian.direction = match (walk_north, walk_east, walk_south, walk_west) {
        (true, false, false, true) => WalkingDirection::Northwest,
        (true, true, false, false) => WalkingDirection::Northeast,
        (false, true, true, false) => WalkingDirection::Southeast,
        (false, false, true, true) => WalkingDirection::Southwest,
        (true, false, false, false) => WalkingDirection::North,
        (false, true, false, false) => WalkingDirection::East,
        (false, false, true, false) => WalkingDirection::South,
        (false, false, false, true) => WalkingDirection::West,
        _ => current_direction, // No change if no keys are pressed or if there's an ambiguous combination
    };
}

pub fn update_position(pedestrian_components: &mut PedestrianComponents, delta_time: f32) {
    let movement: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> =
        match pedestrian_components.direction {
            WalkingDirection::North => Vector2::new(0.0, 1.0),
            WalkingDirection::Northeast => Vector2::new(1.0, 1.0),
            WalkingDirection::East => Vector2::new(1.0, 0.0),
            WalkingDirection::Southeast => Vector2::new(1.0, -1.0),
            WalkingDirection::South => Vector2::new(0.0, -1.0),
            WalkingDirection::Southwest => Vector2::new(-1.0, -1.0),
            WalkingDirection::West => Vector2::new(-1.0, 0.0),
            WalkingDirection::Northwest => Vector2::new(-1.0, 1.0),
        };
    pedestrian_components.base.position +=
        movement.normalize() * pedestrian_components.base.speed * delta_time;
}

// Update the sprite index based on the direction
pub fn update_sprite_index(pedestrian_components: &PedestrianComponents) -> usize {
    // Assuming a sprite sheet where each direction has a corresponding sprite
    match pedestrian_components.direction {
        WalkingDirection::North => 2,
        WalkingDirection::Northeast => 2,
        WalkingDirection::East => 1,
        WalkingDirection::Southeast => 1,
        WalkingDirection::South => 0,
        WalkingDirection::Southwest => 0,
        WalkingDirection::West => 3,
        WalkingDirection::Northwest => 3,
    }
}
