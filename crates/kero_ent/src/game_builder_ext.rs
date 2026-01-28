use crate::registry::Registry;
use crate::{ComponentModule, ComponentOfModule, ComponentType, EntityModule, WorldModule};
use kero::core::GameError;
use kero::prelude::GameBuilder;

pub trait GameBuilderExt: Sized {
    fn with_component<C: ComponentType>(self) -> Result<Self, GameError>;
}

impl GameBuilderExt for GameBuilder {
    fn with_component<C: ComponentType>(mut self) -> Result<Self, GameError> {
        // initialize the entity system and its modules if it hasn't already been
        if self.lua.app_data_ref::<Registry>().is_none() {
            Registry::init(&self.lua)?;
            self = self
                .with_module::<WorldModule>()?
                .with_module::<EntityModule>()?
                .with_module::<ComponentModule>()?;
        }

        // register the rust component
        let mut reg = self.lua.app_data_mut::<Registry>().unwrap();
        reg.register_rust::<C>();
        drop(reg);
        self.with_module::<ComponentOfModule<C>>()
    }
}
