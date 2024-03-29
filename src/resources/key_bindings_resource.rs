use std::{collections::HashMap, fs};

use bevy::asset::ron;
use bevy::prelude::{KeyCode, Resource};
use serde::Deserialize;

#[derive(Resource)]
pub struct KeyBindingsResource {
    pub axes: HashMap<String, AxisBinding>,
    pub actions: HashMap<String, ActionBinding>,
}

#[derive(Deserialize)]
pub struct AxisBinding {
    pub pos: KeyCode,
    pub neg: KeyCode,
}

#[derive(Deserialize)]
pub struct ActionBinding {
    pub key: KeyCode,
}

#[derive(Deserialize)]
struct KeyBindingsConfig {
    axes: HashMap<String, AxisBindingConfig>,
    actions: HashMap<String, ActionBindingConfig>,
}

#[derive(Deserialize)]
struct AxisBindingConfig {
    pos: String,
    neg: String,
}

#[derive(Deserialize)]
struct ActionBindingConfig {
    key: String,
}

impl KeyBindingsResource {
    pub fn load(file_path: &str) -> Self {
        let ron_data = fs::read_to_string(file_path)
            .expect("Failed to read RON file");

        let config: KeyBindingsConfig = ron::from_str(&ron_data)
            .expect("Failed to deserialize RON data");

        let axes = config.axes.into_iter().map(|(k, v)| (k, AxisBinding {
            pos: string_to_keycode(&v.pos),
            neg: string_to_keycode(&v.neg),
        })).collect();

        let actions = config.actions.into_iter().map(|(k, v)| (k, ActionBinding {
            key: string_to_keycode(&v.key),
        })).collect();

        Self { axes, actions }
    }
}

fn string_to_keycode(key: &str) -> KeyCode {
    match key {
        "W" => KeyCode::KeyW,
        "A" => KeyCode::KeyA,
        "S" => KeyCode::KeyS,
        "D" => KeyCode::KeyD,
        "Escape" => KeyCode::Escape,
        _ => panic!("Unsupported key: {}", key),
    }
}
