use crate::{
    ComponentData, ComponentMut, ComponentRef, ComponentType, EntityObj, EntityRef, IntoComponent,
    LuaType, RustType, WorldObj,
};
use kero::math::Vec2F;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{AnyUserData, Either, FromLua, IntoLua, Lua, Table, Value};
use std::ffi::c_void;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Component {
    pub(crate) obj: Either<RustComponent, LuaComponent>,
}

impl Component {
    #[inline]
    pub fn try_cast_ref<C: ComponentType>(&self) -> Option<ComponentRef<C>> {
        self.obj
            .as_ref()
            .left()
            .and_then(|obj| obj.data.borrow().ok())
    }

    #[inline]
    pub fn cast_ref<C: ComponentType>(&self) -> ComponentRef<C> {
        self.try_cast_ref().unwrap()
    }

    #[inline]
    pub fn try_cast<C: ComponentType>(&self) -> Option<ComponentData<C>> {
        self.obj
            .as_ref()
            .left()
            .and_then(|obj| ComponentData::try_from_any_ref(&obj.data))
    }

    #[inline]
    pub fn cast<C: ComponentType>(&self) -> ComponentData<C> {
        self.try_cast().unwrap()
    }

    #[inline]
    pub fn try_borrow<C: ComponentType>(&self) -> LuaResult<Option<ComponentRef<C>>> {
        if let Either::Left(obj) = &self.obj {
            obj.data.borrow().map(Some)
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn try_borrow_mut<C: ComponentType>(&mut self) -> LuaResult<Option<ComponentMut<C>>> {
        if let Either::Left(obj) = &self.obj {
            obj.data.borrow_mut().map(Some)
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn borrow<C: ComponentType>(&self) -> LuaResult<ComponentRef<C>> {
        self.obj.as_ref().left().unwrap().data.borrow()
    }

    #[inline]
    pub fn borrow_mut<C: ComponentType>(&mut self) -> LuaResult<ComponentMut<C>> {
        self.obj.as_ref().left().unwrap().data.borrow_mut()
    }

    #[inline]
    pub fn as_value(&self) -> Value {
        match &self.obj {
            Either::Left(obj) => Value::UserData(obj.data.clone()),
            Either::Right(obj) => Value::Table(obj.table.clone()),
        }
    }

    #[inline]
    pub fn as_table(&self) -> Option<&Table> {
        self.obj.as_ref().right().map(|obj| &obj.table)
    }

    #[inline]
    pub fn ptr(&self) -> *const c_void {
        match &self.obj {
            Either::Left(obj) => obj.data.to_pointer(),
            Either::Right(obj) => obj.table.to_pointer(),
        }
    }

    #[inline]
    pub(crate) fn type_ptr(&self) -> *const c_void {
        match &self.obj {
            Either::Left(obj) => Arc::as_ptr(&obj.ty) as _,
            Either::Right(obj) => Arc::as_ptr(&obj.ty) as _,
        }
    }

    #[inline]
    pub fn type_name(&self) -> &str {
        match &self.obj {
            Either::Left(obj) => obj.ty.type_name,
            Either::Right(obj) => &obj.ty.type_name,
        }
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.ptr() == other.ptr()
    }

    #[inline]
    pub fn entity(&self) -> Option<EntityObj> {
        match &self.obj {
            Either::Left(c) => (c.ty.entity_fn)(&c.data),
            Either::Right(c) => c.table.get("entity").unwrap(),
        }
    }

    #[inline]
    pub fn world(&self) -> Option<WorldObj> {
        match &self.obj {
            Either::Left(c) => (c.ty.world_fn)(&c.data),
            Either::Right(c) => c.table.get::<EntityRef>("entity").unwrap().world.clone(),
        }
    }

    #[inline]
    pub(crate) fn set_entity(&self, entity: Option<EntityObj>) {
        match &self.obj {
            Either::Left(c) => (c.ty.set_entity_fn)(&c.data, entity),
            Either::Right(c) => c.table.set("entity", entity).unwrap(),
        }
    }

    #[inline]
    pub fn active(&self) -> bool {
        match &self.obj {
            Either::Left(c) => (c.ty.active_fn)(&c.data),
            Either::Right(c) => c.table.get("active").unwrap(),
        }
    }

    #[inline]
    pub fn set_active(&self, active: bool) {
        match &self.obj {
            Either::Left(c) => (c.ty.set_active_fn)(&c.data, active),
            Either::Right(c) => c.table.set("active", active).unwrap(),
        }
    }

    #[inline]
    pub fn visible(&self) -> bool {
        match &self.obj {
            Either::Left(c) => (c.ty.visible_fn)(&c.data),
            Either::Right(c) => c.table.get("visible").unwrap(),
        }
    }

    #[inline]
    pub fn set_visible(&self, visible: bool) {
        match &self.obj {
            Either::Left(c) => (c.ty.set_visible_fn)(&c.data, visible),
            Either::Right(c) => c.table.set("active", visible).unwrap(),
        }
    }

    #[inline]
    pub fn flags(&self) -> u64 {
        match &self.obj {
            Either::Left(c) => (c.ty.flags_fn)(&c.data),
            Either::Right(c) => c.table.get("flags").unwrap(),
        }
    }

    #[inline]
    pub fn set_flags(&self, flags: u64) {
        match &self.obj {
            Either::Left(c) => (c.ty.set_flags_fn)(&c.data, flags),
            Either::Right(c) => c.table.set("flags", flags).unwrap(),
        }
    }

    #[inline]
    pub fn depth(&self) -> f64 {
        match &self.obj {
            Either::Left(c) => (c.ty.depth_fn)(&c.data),
            Either::Right(c) => c.table.get("depth").unwrap(),
        }
    }

    #[inline]
    pub fn set_depth(&self, depth: f64) {
        match &self.obj {
            Either::Left(c) => (c.ty.set_depth_fn)(&c.data, depth),
            Either::Right(c) => c.table.set("depth", depth).unwrap(),
        }
    }

    #[inline]
    pub(crate) fn do_render(&self, lua: &Lua, pos: Vec2F) -> LuaResult<()> {
        match &self.obj {
            Either::Left(c) => {
                if let Some(f) = c.ty.render_fn {
                    return (f)(&c.data, lua, pos);
                }
            }
            Either::Right(c) => {
                if let Some(f) = c.ty.render_fn.as_ref() {
                    return f.call((c.table.clone(),));
                }
            }
        }
        Ok(())
    }
}

macro_rules! impl_callback {
    ($name:ident $fn_name:ident) => {
        impl Component {
            #[inline]
            pub(crate) fn $name(&self, lua: &Lua) -> LuaResult<()> {
                match &self.obj {
                    Either::Left(c) => {
                        if let Some(f) = c.ty.$fn_name {
                            return (f)(&c.data, lua);
                        }
                    }
                    Either::Right(c) => {
                        if let Some(f) = c.ty.$fn_name.as_ref() {
                            return f.call((c.table.clone(),));
                        }
                    }
                }
                Ok(())
            }
        }
    };
}

impl_callback!(do_added added_fn);
impl_callback!(do_removed removed_fn);
impl_callback!(do_spawned spawned_fn);
impl_callback!(do_despawned despawned_fn);
impl_callback!(do_update update_fn);
//impl_callback!(do_render render_fn);

impl FromLua for Component {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::UserData(data) => data.into_component(lua),
            Value::Table(table) => table.into_component(lua),
            val => Err(LuaError::runtime(format!(
                "[{val:?}] is not a valid component"
            ))),
        }
    }
}

impl IntoLua for Component {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(self.as_value())
    }
}

#[derive(Debug, Clone)]
pub struct RustComponent {
    pub(crate) data: AnyUserData,
    pub(crate) ty: Arc<RustType>,
}

#[derive(Debug, Clone)]
pub struct LuaComponent {
    pub(crate) table: Table,
    pub(crate) ty: Arc<LuaType>,
}
