use amethyst::{
    core::{Time, Transform, TransformBundle},
    ecs::{storage::MaskedStorage, Builder, Entity, Storage, WorldExt},
    input::{Axis, Button, InputBundle, InputHandler, StringBindings},
    renderer::SpriteRender,
    shred::Fetch,
    window::ScreenDimensions,
    winit::VirtualKeyCode,
    Error, GameData, StateEvent, StateEventReader,
};
use amethyst_test::prelude::*;
use yakuzaishi::{
    components::vehicle_components::VehicleComponents,
    resources::vehicle_resource::VehicleResource,
    systems::vehicle_controller_system::VehicleControllerSystem, SPRITE_SHEET_FILE_PATH,
    VEHICLE_TEXTURE_FILE_PATH,
};

//TODO These tests are much more difficult than aticipated, might make more sense later

#[test]
fn test_vehicle_controller_system_template_method() -> Result<(), Error> {
    AmethystApplication::blank()
        .with_system(VehicleControllerSystem, "vehicle_controller_system", &[])
        .with_effect(|_world| {
            // Setup your specific test scenario here
            // For example, creating a VehicleComponent and attaching it to an entity.
        })
        .with_assertion(|_world| {
            // Assertions to verify the behavior of your system.
            // For example, checking if the VehicleComponent has been updated correctly.
        })
        .run()
}

#[test]
fn test_vehicle_controller_system_forward_movement() -> Result<(), Error> {
    let application: AmethystApplication<GameData<'_, '_>, StateEvent, StateEventReader> =
        AmethystApplication::blank()
            .with_effect(|_world| {
                _world.insert(ScreenDimensions::new(800, 600, 1.0)); // Use your desired screen dimensions here

                let vehicle_sprite_sheet: VehicleResource =
                    VehicleResource::new(_world, VEHICLE_TEXTURE_FILE_PATH, SPRITE_SHEET_FILE_PATH);

                // Create an entity with VehicleComponents, Transform, and SpriteRender
                let entity: amethyst::ecs::prelude::Entity = _world
                    .create_entity()
                    .with(VehicleComponents::new(0.0, 0.0)) // Assuming a new function for instantiation
                    .with(Transform::default())
                    .with(SpriteRender::new(
                        vehicle_sprite_sheet.sprite_sheet_handle,
                        1,
                    ))
                    .build();
                _world.insert(EffectReturn(entity));

                let mut input: InputHandler<StringBindings> = InputHandler::<StringBindings>::new();

                // Create an axis and bind it to an ID
                let axis: Axis = Axis::Emulated {
                    pos: Button::Key(VirtualKeyCode::W), // The key representing the positive direction
                    neg: Button::Key(VirtualKeyCode::S), // The key representing the negative direction
                };

                //TODO what in the hell is "_"
                let _ = input.bindings.insert_axis("vehicle_forward", axis);
                // Here we're setting the value directly rather than sending an event.

                _world.insert(input);

                // Mock the time to control the delta_time
                let time = Time::default();
                _world.insert(time);
            })
            .with_bundle(TransformBundle::new())
            .with_bundle(InputBundle::<StringBindings>::new())
            .with_system(VehicleControllerSystem, "vehicle_controller_system", &[])
            .with_assertion(|_world| {
                let entity: Entity = _world.read_resource::<EffectReturn<Entity>>().0.clone();
                let vehicle_components: Storage<
                    '_,
                    VehicleComponents,
                    Fetch<'_, MaskedStorage<VehicleComponents>>,
                > = _world.read_storage::<VehicleComponents>();
                let transform: Storage<'_, Transform, Fetch<'_, MaskedStorage<Transform>>> =
                    _world.read_storage::<Transform>();

                let vehicle_component: &VehicleComponents = vehicle_components.get(entity).unwrap();
                let transform_component: &Transform = transform.get(entity).unwrap();

                // Assert that the vehicle has moved forward
                assert!(vehicle_component.speed > 0.0);

                // Assert that the position has been updated
                assert!(transform_component.translation().y > 0.0);
            });

    application.run()
}
