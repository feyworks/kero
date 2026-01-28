use crate::{ComponentData, ComponentOf};
use kero::math::Vec2F;
use mlua::prelude::{LuaResult, LuaString};
use mlua::{AnyUserData, Lua, UserDataFields, UserDataMethods};

pub trait ComponentType: Sized + 'static {
    const NAME: &'static str;
    const PATH: &'static str;

    const ADDED_FN: Option<fn(this: &AnyUserData, lua: &Lua) -> LuaResult<()>> = None;
    const REMOVED_FN: Option<fn(this: &AnyUserData, lua: &Lua) -> LuaResult<()>> = None;
    const SPAWNED_FN: Option<fn(this: &AnyUserData, lua: &Lua) -> LuaResult<()>> = None;
    const DESPAWNED_FN: Option<fn(this: &AnyUserData, lua: &Lua) -> LuaResult<()>> = None;
    const UPDATE_FN: Option<fn(this: &AnyUserData, lua: &Lua) -> LuaResult<()>> = None;
    const RENDER_FN: Option<fn(this: &AnyUserData, lua: &Lua, pos: Vec2F) -> LuaResult<()>> = None;

    #[inline]
    fn tostring(this: ComponentData<Self>, lua: &Lua) -> LuaResult<LuaString> {
        let ptr = this.ptr() as usize;
        lua.create_string(format!("{}({:016X})", Self::NAME, ptr))
    }

    #[inline]
    #[allow(unused_variables)]
    fn fields<F: UserDataFields<ComponentOf<Self>>>(fields: &mut F) {}

    #[inline]
    #[allow(unused_variables)]
    fn methods<T, M: UserDataMethods<T>>(methods: &mut M) {}
}
