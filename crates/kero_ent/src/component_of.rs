use crate::{ComponentObj, ComponentType, EntityMut, EntityObj, EntityRef, WorldRef};
use kero::lua::UserDataOf;
use mlua::Lua;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ComponentOf<T> {
    pub(crate) entity: Option<EntityObj>,
    pub active: bool,
    pub visible: bool,
    pub flags: u64,
    pub depth: f64,
    pub value: T,
}

impl<T: ComponentType> ComponentOf<T> {
    #[inline]
    pub fn new(
        lua: &Lua,
        active: bool,
        visible: bool,
        flags: u64,
        depth: f64,
        value: T,
    ) -> ComponentObj<T> {
        UserDataOf::new(
            lua,
            Self {
                entity: None,
                active,
                visible,
                flags,
                depth,
                value,
            },
        )
    }

    #[inline]
    pub fn with_flags(lua: &Lua, flags: u64, value: T) -> ComponentObj<T> {
        Self::new(lua, true, true, flags, 0.0, value)
    }

    #[inline]
    pub fn entity(&self) -> Option<&EntityObj> {
        self.entity.as_ref()
    }

    #[inline]
    pub fn try_entity_ref(&self) -> Option<EntityRef> {
        self.entity.as_ref().map(|e| e.get())
    }

    #[inline]
    pub fn entity_ref(&self) -> EntityRef {
        self.entity.as_ref().map(|e| e.get()).unwrap()
    }

    #[inline]
    pub fn entity_mut(&self) -> EntityMut {
        self.entity.as_ref().map(|e| e.get_mut()).unwrap()
    }

    #[inline]
    pub fn world_ref(&self) -> Option<WorldRef> {
        self.entity
            .as_ref()
            .and_then(|e| e.get().world.as_ref().map(|w| w.get()))
    }
}

impl<T: ComponentType> Deref for ComponentOf<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: ComponentType> DerefMut for ComponentOf<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
