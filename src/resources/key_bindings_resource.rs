use amethyst::{
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
    Error,
};

use std::collections::HashMap;
use std::path::PathBuf;

use crate::state::entity_type::EntityType;

pub struct KeyBindingsResource {
    //TODO: not yet actually a hashmap, fix it
    bindings: HashMap<EntityType, InputBundle<StringBindings>>,
}

impl KeyBindingsResource {
    pub fn load(entity_type: EntityType, key_bindings_file_path: &str) -> Result<Self, Error> {
        let mut bindings = HashMap::new();
        let app_root = application_root_dir()?;
        let assets_path: PathBuf = app_root.join("assets");
        // Load the vehicle bindings
        let bindings_path = assets_path.join(key_bindings_file_path);

        let input_bundle = Self::load_bindings(bindings_path)?;
        bindings.insert(entity_type, input_bundle);

        Ok(Self { bindings })
    }

    pub fn get_bindings(&self, entity_type: &EntityType) -> Option<&InputBundle<StringBindings>> {
        self.bindings.get(entity_type)
    }

    fn load_bindings(bindings_path: PathBuf) -> Result<InputBundle<StringBindings>, Error> {
        InputBundle::<StringBindings>::new()
            .with_bindings_from_file(bindings_path)
            .map_err(Error::from)
    }
}
