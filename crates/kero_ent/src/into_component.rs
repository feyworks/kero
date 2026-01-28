use crate::{Component, ComponentOf, ComponentType, LuaComponent, Registry, RustComponent};
use mlua::prelude::LuaResult;
use mlua::{AnyUserData, Either, FromLua, Lua, Table, Value};

pub trait IntoComponent {
    fn into_component(self, lua: &Lua) -> LuaResult<Component>;
}

impl IntoComponent for Component {
    #[inline]
    fn into_component(self, _lua: &Lua) -> LuaResult<Component> {
        Ok(self)
    }
}

impl IntoComponent for Value {
    #[inline]
    fn into_component(self, lua: &Lua) -> LuaResult<Component> {
        Component::from_lua(self, lua)
    }
}

impl IntoComponent for Table {
    #[inline]
    fn into_component(self, lua: &Lua) -> LuaResult<Component> {
        let ty = Registry::get(lua).lua_type(&self)?.clone();
        Ok(Component {
            obj: Either::Right(LuaComponent { table: self, ty }),
        })
    }
}

impl<T: ComponentType> IntoComponent for ComponentOf<T> {
    #[inline]
    fn into_component(self, lua: &Lua) -> LuaResult<Component> {
        let ty = Registry::get(lua).rust_type::<T>()?.clone();
        Ok(Component {
            obj: Either::Left(RustComponent {
                data: lua.create_userdata(self)?,
                ty,
            }),
        })
    }
}

impl IntoComponent for AnyUserData {
    #[inline]
    fn into_component(self, lua: &Lua) -> LuaResult<Component> {
        let ty = Registry::get(lua)
            .rust_id_type(self.type_id().unwrap())?
            .clone();
        Ok(Component {
            obj: Either::Left(RustComponent { data: self, ty }),
        })
    }
}
