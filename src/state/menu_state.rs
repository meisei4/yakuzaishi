use amethyst::ui::FontAsset;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    prelude::*,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

use crate::state::main_game_state::Yakuzaishi;

pub struct MenuState {
    selected_option: usize,
    vehicle_option: Option<Entity>,
    pedestrian_option: Option<Entity>,
    font_handle: Option<Handle<FontAsset>>,
}

impl MenuState {
    const HORIZONTAL_SPACING: f32 = 150.0;
    const VEHICLE_LABEL: &'static str = "Vehicle";
    const PEDESTRIAN_LABEL: &'static str = "Pedestrian";
    const FONT_PATH: &'static str = "font_data/dosei_en.ttf";

    // Constants for bindings file paths
    const VEHICLE_BINDINGS_CONFIG_FILENAME: &'static str = "key_bindings/vehicle_bindings.ron";
    const PEDESTRIAN_BINDINGS_CONFIG_FILENAME: &'static str =
        "key_bindings/pedestrian_bindings.ron";

    fn create_menu_option(
        &mut self,
        world: &mut World,
        label: &str,
        x_coordinate: f32,
    ) -> Option<Entity> {
        self.font_handle
            .clone()
            .map(|font_handle| create_ui_entity(world, label, x_coordinate, 0.0, font_handle))
    }

    fn handle_input(&mut self, input: &InputHandler<StringBindings>) -> Option<SimpleTrans> {
        if input.action_is_down("select").unwrap_or(false) {
            return Some(match self.selected_option {
                0 => Trans::Switch(Box::new(Yakuzaishi::new(
                    Self::VEHICLE_LABEL,
                    Self::VEHICLE_BINDINGS_CONFIG_FILENAME,
                ))),
                1 => Trans::Switch(Box::new(Yakuzaishi::new(
                    Self::PEDESTRIAN_LABEL,
                    Self::PEDESTRIAN_BINDINGS_CONFIG_FILENAME,
                ))),
                _ => Trans::None,
            });
        }

        if input.action_is_down("move_right").unwrap_or(false) {
            self.selected_option = (self.selected_option + 1) % 2;
        }

        if input.action_is_down("move_left").unwrap_or(false) {
            self.selected_option = if self.selected_option == 0 { 1 } else { 0 };
        }

        None
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.font_handle = Some(load_font(world, Self::FONT_PATH));

        self.vehicle_option =
            self.create_menu_option(world, Self::VEHICLE_LABEL, -Self::HORIZONTAL_SPACING);
        self.pedestrian_option =
            self.create_menu_option(world, Self::PEDESTRIAN_LABEL, Self::HORIZONTAL_SPACING);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        if let Some(vehicle_option) = self.vehicle_option.take() {
            world
                .delete_entity(vehicle_option)
                .expect("Failed to delete vehicle option");
        }

        if let Some(pedestrian_option) = self.pedestrian_option.take() {
            world
                .delete_entity(pedestrian_option)
                .expect("Failed to delete pedestrian option");
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Input(_input_event) = event {
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
