use std::collections::HashMap;
use std::path::PathBuf;

use amethyst::{
    Error,
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
};

use crate::enums::entity_type::EntityType;

pub struct KeyBindingsResource {
    bindings: HashMap<EntityType, PathBuf>,
}

impl KeyBindingsResource {
    pub fn load(entity_type: EntityType, key_bindings_file_path: &str) -> Result<Self, Error> {
        let mut bindings = HashMap::new();
        let app_root = application_root_dir()?;
        let assets_path = app_root.join("assets");
        let bindings_path = assets_path.join(key_bindings_file_path);

        bindings.insert(entity_type, bindings_path);

        Ok(Self { bindings })
    }

    pub fn get_bindings_path(&self, entity_type: &EntityType) -> Option<&PathBuf> {
        self.bindings.get(entity_type)
    }

    pub fn get_input_bundle(
        &self,
        entity_type: &EntityType,
    ) -> Result<InputBundle<StringBindings>, Error> {
        let bindings_path = self.get_bindings_path(entity_type).ok_or_else(|| {
            Error::from_string("Binding path not found for the given entity type")
        })?;

        InputBundle::<StringBindings>::new()
            .with_bindings_from_file(bindings_path)
            .map_err(Error::from)
    }
}
