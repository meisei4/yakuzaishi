use crate::components::pedestrian_components::{PedestrianComponents, WalkingDirection};
use amethyst::{
    core::{timing::Time, Transform},
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
            update_pedestrian_sprite(pedestrian_component, sprite_render);
            pedestrian_component.update_position(delta_time);
            update_transform(pedestrian_component, transform);
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

fn update_pedestrian_sprite(
    pedestrian_component: &mut PedestrianComponents,
    sprite_render: &mut SpriteRender,
) {
    pedestrian_component.update_sprite_index(); //TODO perhaps bad practice not sure if should return new sprite instead?
    sprite_render.sprite_number = pedestrian_component.current_sprite_index;
}

fn update_transform(pedestrian: &PedestrianComponents, transform: &mut Transform) {
    transform.set_translation_x(pedestrian.position.x);
    transform.set_translation_y(pedestrian.position.y);
}
