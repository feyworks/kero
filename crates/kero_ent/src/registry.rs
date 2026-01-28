use super::{ComponentOf, ComponentType, EntityObj, WorldObj};
use fnv::FnvHashMap;
use kero::math::Vec2F;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{AnyUserData, AppDataRef, Function, Lua, Table};
use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Registry {
    pub rust_types: Vec<Arc<RustType>>,
    pub lua_types: Vec<Arc<LuaType>>,
    pub type_lookup: FnvHashMap<TypeId, usize>,
    pub name_lookup: HashMap<String, Index>,
    pub module_lookup: FnvHashMap<*const c_void, usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Index {
    Rust(usize),
    Lua(usize),
}

impl Registry {
    pub fn init(lua: &Lua) -> LuaResult<()> {
        lua.set_app_data(Self {
            rust_types: Vec::new(),
            lua_types: Vec::new(),
            type_lookup: FnvHashMap::default(),
            name_lookup: HashMap::new(),
            module_lookup: FnvHashMap::default(),
        });
        Ok(())
    }

    #[inline]
    pub fn get(lua: &Lua) -> AppDataRef<'_, Self> {
        lua.app_data_ref::<Self>().unwrap()
    }

    // TODO: call this when Lua reloads to clear Lua-defined components
    // pub fn clear_lua(&mut self) {
    //     self.lua_types.clear();
    //     self.name_lookup.retain(|_, i| matches!(i, Index::Rust(_)));
    //     self.module_lookup.clear();
    // }

    pub fn register_rust<T: ComponentType>(&mut self) {
        let idx = self.rust_types.len();
        self.type_lookup.insert(TypeId::of::<ComponentOf<T>>(), idx);
        self.name_lookup
            .insert(T::NAME.to_string(), Index::Rust(idx));
        self.rust_types.push(Arc::new(RustType {
            type_name: T::NAME,
            entity_fn: |this| this.borrow::<ComponentOf<T>>().unwrap().entity.clone(),
            world_fn: |this| {
                if let Some(ent) = this.borrow_mut::<ComponentOf<T>>().unwrap().entity.as_ref() {
                    ent.get().world.clone()
                } else {
                    None
                }
            },
            set_entity_fn: |this, value: Option<EntityObj>| {
                this.borrow_mut::<ComponentOf<T>>().unwrap().entity = value;
            },
            flags_fn: |this| this.borrow_mut::<ComponentOf<T>>().unwrap().flags,
            set_flags_fn: |this, value: u64| {
                this.borrow_mut::<ComponentOf<T>>().unwrap().flags = value;
            },
            active_fn: |this| this.borrow_mut::<ComponentOf<T>>().unwrap().active,
            set_active_fn: |this, value: bool| {
                this.borrow_mut::<ComponentOf<T>>().unwrap().active = value;
            },
            visible_fn: |this| this.borrow_mut::<ComponentOf<T>>().unwrap().visible,
            set_visible_fn: |this, value: bool| {
                this.borrow_mut::<ComponentOf<T>>().unwrap().visible = value;
            },
            depth_fn: |this| this.borrow_mut::<ComponentOf<T>>().unwrap().depth,
            set_depth_fn: |this, value: f64| {
                this.borrow_mut::<ComponentOf<T>>().unwrap().depth = value;
            },
            added_fn: T::ADDED_FN,
            removed_fn: T::REMOVED_FN,
            spawned_fn: T::SPAWNED_FN,
            despawned_fn: T::DESPAWNED_FN,
            update_fn: T::UPDATE_FN,
            render_fn: T::RENDER_FN,
        }));
    }

    pub fn register_lua(&mut self, name: String, module: Table) -> LuaResult<()> {
        let idx = self.lua_types.len();
        self.name_lookup.insert(name.clone(), Index::Lua(idx));
        self.module_lookup.insert(module.to_pointer(), idx);
        self.lua_types.push(Arc::new(LuaType {
            type_name: name,
            added_fn: module.get::<Option<Function>>("added")?,
            removed_fn: module.get::<Option<Function>>("removed")?,
            spawned_fn: module.get::<Option<Function>>("spawned")?,
            despawned_fn: module.get::<Option<Function>>("despawned")?,
            update_fn: module.get::<Option<Function>>("update")?,
            render_fn: module.get::<Option<Function>>("render")?,
        }));
        Ok(())
    }

    #[inline]
    pub(crate) fn rust_id_type(&self, type_id: TypeId) -> LuaResult<&Arc<RustType>> {
        match self.type_lookup.get(&type_id) {
            Some(&i) => Ok(&self.rust_types[i]),
            None => Err(LuaError::runtime(format!(
                "rust component with type id [{:?}] is not registered",
                type_id
            ))),
        }
    }

    #[inline]
    pub(crate) fn rust_type<T: ComponentType>(&self) -> LuaResult<&Arc<RustType>> {
        match self.type_lookup.get(&TypeId::of::<ComponentOf<T>>()) {
            Some(&i) => Ok(&self.rust_types[i]),
            None => Err(LuaError::runtime(format!(
                "rust component type [{}] was not registered",
                type_name::<T>()
            ))),
        }
    }

    #[inline]
    pub(crate) fn rust_type_ptr<T: ComponentType>(&self) -> LuaResult<*const c_void> {
        Ok(self.rust_type::<T>().map(Arc::as_ptr)? as _)
    }

    #[inline]
    pub(crate) fn name_type_ptr(&self, ty_name: &str) -> LuaResult<*const c_void> {
        match self.name_lookup.get(ty_name) {
            Some(&i) => Ok(match i {
                Index::Rust(i) => Arc::as_ptr(&self.rust_types[i]) as _,
                Index::Lua(i) => Arc::as_ptr(&self.lua_types[i]) as _,
            }),
            None => Err(LuaError::runtime(format!(
                "type not found with the name [{ty_name}]"
            ))),
        }
    }

    #[inline]
    pub(crate) fn lua_type(&self, obj: &Table) -> LuaResult<&Arc<LuaType>> {
        obj.metatable()
            .and_then(|m| self.module_lookup.get(&m.to_pointer()))
            .map(|&i| &self.lua_types[i])
            .ok_or_else(|| {
                LuaError::runtime("component's metatable is not a registered component type")
            })
    }

    // #[inline]
    // pub(crate) fn lua_type_ptr(&self, obj: &Table) -> LuaResult<*const c_void> {
    //     Ok(self.lua_type(obj).map(Arc::as_ptr)? as _)
    // }
}

#[derive(Debug)]
pub struct RustType {
    pub type_name: &'static str,
    pub entity_fn: fn(&AnyUserData) -> Option<EntityObj>,
    pub world_fn: fn(&AnyUserData) -> Option<WorldObj>,
    pub set_entity_fn: fn(&AnyUserData, Option<EntityObj>),
    pub active_fn: fn(&AnyUserData) -> bool,
    pub set_active_fn: fn(&AnyUserData, bool),
    pub visible_fn: fn(&AnyUserData) -> bool,
    pub set_visible_fn: fn(&AnyUserData, bool),
    pub flags_fn: fn(&AnyUserData) -> u64,
    pub set_flags_fn: fn(&AnyUserData, u64),
    pub depth_fn: fn(&AnyUserData) -> f64,
    pub set_depth_fn: fn(&AnyUserData, f64),
    pub added_fn: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>>,
    pub removed_fn: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>>,
    pub spawned_fn: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>>,
    pub despawned_fn: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>>,
    pub update_fn: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>>,
    pub render_fn: Option<fn(&AnyUserData, &Lua, Vec2F) -> LuaResult<()>>,
}

#[derive(Debug)]
pub struct LuaType {
    pub type_name: String,
    pub added_fn: Option<Function>,
    pub removed_fn: Option<Function>,
    pub spawned_fn: Option<Function>,
    pub despawned_fn: Option<Function>,
    pub update_fn: Option<Function>,
    pub render_fn: Option<Function>,
}
