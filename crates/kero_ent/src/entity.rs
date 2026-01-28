use crate::{Component, ComponentObj, ComponentType, EntityObj, IntoComponent, Registry, WorldObj};
use kero::lua::UserDataOf;
use kero::math::Vec2F;
use mlua::Lua;
use mlua::prelude::{LuaError, LuaResult};
use std::ffi::c_void;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosVersion(u64);

impl PosVersion {
    pub const NONE: Self = Self(0);

    #[inline]
    fn increment(&mut self) {
        self.0 = self.0.checked_add(1).unwrap_or(1);
    }
}

#[derive(Debug)]
pub struct Entity {
    pub(crate) world: Option<WorldObj>,
    pub(crate) components: Vec<Option<Component>>,
    pub(crate) cleanup: bool,
    pub active: bool,
    pub visible: bool,
    pos: Vec2F,
    version: PosVersion,
}

impl Entity {
    #[inline]
    pub fn new(lua: &Lua) -> UserDataOf<Self> {
        Self::new_at(lua, Vec2F::ZERO)
    }

    #[inline]
    pub fn new_at(lua: &Lua, pos: Vec2F) -> UserDataOf<Self> {
        UserDataOf::new(
            lua,
            Self {
                world: None,
                components: Vec::new(),
                cleanup: false,
                active: true,
                visible: true,
                pos,
                version: PosVersion(1),
            },
        )
    }

    #[inline]
    pub fn pos(&self) -> Vec2F {
        self.pos
    }

    #[inline]
    pub fn set_pos(&mut self, val: Vec2F) {
        if self.pos != val {
            self.version.increment();
        }
        self.pos = val;
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.pos.x
    }

    #[inline]
    pub fn set_x(&mut self, val: f32) {
        if self.pos.x != val {
            self.version.increment();
        }
        self.pos.x = val;
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.pos.y
    }

    #[inline]
    pub fn set_y(&mut self, val: f32) {
        if self.pos.y != val {
            self.version.increment();
        }
        self.pos.y = val;
    }

    #[inline]
    pub fn pos_version(&self) -> PosVersion {
        self.version
    }

    #[inline]
    fn index_of_ptr(&self, ptr: *const c_void) -> Option<usize> {
        self.components
            .iter()
            .position(|c| c.as_ref().is_some_and(|c| c.ptr() == ptr))
    }

    #[inline]
    pub fn first_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<Option<Component>> {
        let type_ptr = Registry::get(lua).name_type_ptr(type_name)?;
        Ok(self.first_with_type_ptr(type_ptr))
    }

    #[inline]
    pub(crate) fn first_with_type_ptr(&self, type_ptr: *const c_void) -> Option<Component> {
        self.components
            .iter()
            .flatten()
            .find(|comp| comp.type_ptr() == type_ptr)
            .cloned()
    }

    #[inline]
    pub fn first_with_type<C: ComponentType>(&self) -> Option<ComponentObj<C>> {
        self.components
            .iter()
            .flatten()
            .find_map(Component::try_cast)
    }
}

#[inline]
pub(crate) fn ent_cleanup(this: &EntityObj) {
    let mut this = this.get_mut();
    if this.cleanup {
        this.cleanup = false;
        this.components.retain(|c| c.is_some());
    }
}

#[inline]
fn ent_do_removed(
    this: &EntityObj,
    lua: &Lua,
    comp: Component,
    idx: usize,
    in_world: bool,
) -> LuaResult<()> {
    if in_world {
        comp.do_despawned(lua)?;
    }
    comp.do_removed(lua)?;
    comp.set_entity(None);

    let mut this = this.get_mut();
    this.cleanup = true;
    this.components[idx] = None;
    if in_world {
        this.world.as_ref().unwrap().get_mut().cleanup = true;
    }

    Ok(())
}

#[inline]
fn ent_remove_first_with_type_ptr(
    this: &EntityObj,
    lua: &Lua,
    type_ptr: *const c_void,
) -> LuaResult<()> {
    let (len, in_world) = this.field(|e| (e.components.len(), e.world.is_some()))?;
    for idx in 0..len {
        let Some(comp) = this.get().components[idx].clone() else {
            continue;
        };
        if comp.type_ptr() == type_ptr {
            return ent_do_removed(this, lua, comp, idx, in_world);
        }
    }
    Ok(())
}

#[inline]
fn ent_remove_all_with_type_ptr(
    this: &EntityObj,
    lua: &Lua,
    type_ptr: *const c_void,
) -> LuaResult<()> {
    let (len, in_world) = this.field(|e| (e.components.len(), e.world.is_some()))?;
    for idx in 0..len {
        let Some(comp) = this.get().components[idx].clone() else {
            continue;
        };
        if comp.type_ptr() == type_ptr {
            ent_do_removed(this, lua, comp, idx, in_world)?;
        }
    }
    Ok(())
}

pub trait EntityExt: crate::private::Sealed {
    fn add<C: IntoComponent>(&self, lua: &Lua, comp: C) -> LuaResult<Component>;
    fn remove<C: IntoComponent>(&self, lua: &Lua, comp: C) -> LuaResult<()>;
    fn remove_first_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<()>;
    fn remove_first_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<()>;
    fn remove_all_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<()>;
    fn remove_all_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<()>;
    fn clear(&self, lua: &Lua) -> LuaResult<()>;
    fn for_each(&self, f: impl FnMut(Component) -> LuaResult<()>) -> LuaResult<()>;
    fn update(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()>;
}

impl EntityExt for EntityObj {
    #[inline]
    fn add<C: IntoComponent>(&self, lua: &Lua, comp: C) -> LuaResult<Component> {
        // set component's entity field if it's not already on one
        let comp = comp.into_component(lua)?;
        if comp.entity().is_some() {
            return Err(LuaError::runtime("component is already on an entity"));
        }
        comp.set_entity(Some(self.clone()));

        // add the component to the entity
        let in_world = {
            let mut ent = self.get_mut();
            ent.components.push(Some(comp.clone()));
            ent.world.is_some()
        };

        // callbacks
        comp.do_added(lua)?;
        if in_world {
            comp.do_spawned(lua)?;
        }

        Ok(comp)
    }

    #[inline]
    fn remove<C: IntoComponent>(&self, lua: &Lua, comp: C) -> LuaResult<()> {
        let comp = comp.into_component(lua)?;
        let (idx, in_world) = {
            let ent = self.get();
            let idx = ent
                .index_of_ptr(comp.ptr())
                .ok_or_else(|| LuaError::runtime("component is not on the entity"))?;
            (idx, ent.world.is_some())
        };
        ent_do_removed(self, lua, comp, idx, in_world)
    }

    #[inline]
    fn remove_first_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<()> {
        let type_ptr = Registry::get(lua).rust_type_ptr::<C>()?;
        ent_remove_first_with_type_ptr(self, lua, type_ptr)
    }

    #[inline]
    fn remove_first_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<()> {
        let type_ptr = Registry::get(lua).name_type_ptr(type_name)?;
        ent_remove_first_with_type_ptr(self, lua, type_ptr)
    }

    #[inline]
    fn remove_all_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<()> {
        let type_ptr = Registry::get(lua).rust_type_ptr::<C>()?;
        ent_remove_all_with_type_ptr(self, lua, type_ptr)
    }

    #[inline]
    fn remove_all_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<()> {
        let type_ptr = Registry::get(lua).name_type_ptr(type_name)?;
        ent_remove_all_with_type_ptr(self, lua, type_ptr)
    }

    #[inline]
    fn clear(&self, lua: &Lua) -> LuaResult<()> {
        let (len, in_world) = self.field(|e| (e.components.len(), e.world.is_some()))?;
        for idx in 0..len {
            let Some(comp) = self.get().components[idx].clone() else {
                continue;
            };
            ent_do_removed(self, lua, comp, idx, in_world)?;
        }
        Ok(())
    }

    #[inline]
    fn for_each(&self, mut f: impl FnMut(Component) -> LuaResult<()>) -> LuaResult<()> {
        let len = self.get().components.len();
        for idx in 0..len {
            let Some(comp) = self.get().components[idx].clone() else {
                continue;
            };
            f(comp)?;
        }
        Ok(())
    }

    #[inline]
    fn update(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()> {
        let len = {
            let this = self.get();
            if !this.active {
                return Ok(());
            }
            this.components.len()
        };
        match mask {
            Some(mask) => {
                for idx in 0..len {
                    let Some(comp) = self.get().components[idx].clone() else {
                        continue;
                    };
                    if comp.flags() & mask != 0 {
                        comp.do_update(lua)?;
                    }
                }
            }
            None => {
                for idx in 0..len {
                    let Some(comp) = self.get().components[idx].clone() else {
                        continue;
                    };
                    comp.do_update(lua)?;
                }
            }
        }
        Ok(())
    }
}
