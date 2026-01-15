use kero_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{FromLua, IntoLua, Lua, Value};

use super::Guid;

pub struct GuidModule;

impl LuaModule for GuidModule {
    const PATH: &'static str = "Guid";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Guid>::register(lua, "Guid", |members| {
            members.op_eq(|a, b: Guid| a == &b)?;
            members.op_lt(|a, b: Guid| a < &b)?;
            members.op_le(|a, b: Guid| a <= &b)?;
            members.op_tostring_ext(|lua, id| lua.create_string(id.encode_str(&mut [0; _])))?;
            Ok(())
        })?;
        module.set("new", lua.create_function(|_, _: ()| Ok(Guid::new()))?)?;
        Ok(Value::Table(module))
    }
}

impl FromLua for Guid {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|hot| hot.get(lua))
    }
}

impl IntoLua for Guid {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}
