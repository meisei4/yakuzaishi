use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    prelude::*,
    ui::FontAsset,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

use super::entity_type::EntityType;
use crate::{state::main_game_state::Yakuzaishi, FONT_PATH};

pub struct MenuState {
    selected_entity_type: EntityType,
    font_handle: Option<Handle<FontAsset>>,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            selected_entity_type: EntityType::Vehicle, // Default selection
            font_handle: None,
        }
    }

    fn create_ui_entities(&mut self, world: &mut World) {
        // Load the font
        self.font_handle = Some(load_font(world, FONT_PATH));

        // Create UI entities for Vehicle and Pedestrian options
        create_ui_entity(
            world,
            "Vehicle",
            -150.0,
            0.0,
            self.font_handle.clone().unwrap(),
        );
        create_ui_entity(
            world,
            "Pedestrian",
            150.0,
            0.0,
            self.font_handle.clone().unwrap(),
        );
    }

    fn handle_input(&mut self, input: &InputHandler<StringBindings>) -> Option<SimpleTrans> {
        if input.action_is_down("select_vehicle").unwrap_or(false) {
            self.selected_entity_type = EntityType::Vehicle;
        }

        if input.action_is_down("select_pedestrian").unwrap_or(false) {
            self.selected_entity_type = EntityType::Pedestrian;
        }

        if input.action_is_down("select").unwrap_or(false) {
            return match self.selected_entity_type {
                EntityType::Vehicle => Some(Trans::Switch(Box::new(Yakuzaishi::new(
                    EntityType::Vehicle,
                )))),
                EntityType::Pedestrian => Some(Trans::Switch(Box::new(Yakuzaishi::new(
                    EntityType::Pedestrian,
                )))),
            };
        }
        None
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.create_ui_entities(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Input(input_event) = event {
            log::info!("Received input event: {:?}", input_event); // Add logging here
            let input = data.world.read_resource::<InputHandler<StringBindings>>();
            self.handle_input(&input).unwrap_or(Trans::None)
        } else {
            Trans::None
        }
    }
}

fn create_ui_entity(
    world: &mut World,
    label: &str,
    x: f32,
    y: f32,
    font_handle: Handle<FontAsset>,
) -> Entity {
    world
        .create_entity()
        .with(UiTransform::new(
            label.to_string(),
            Anchor::Middle,
            Anchor::Middle,
            x,
            y,
            1.0,
            200.0,
            50.0,
        ))
        .with(UiText::new(
            font_handle,
            label.to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build()
}

fn load_font(world: &mut World, font_path: &str) -> Handle<FontAsset> {
    let loader = world.read_resource::<Loader>();
    let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
    loader.load(font_path, TtfFormat, (), &font_storage)
}
