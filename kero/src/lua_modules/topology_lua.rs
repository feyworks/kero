use crate::gfx::Topology;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, Value};

impl FromLua for Topology {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "triangles" => Self::Triangles,
            "lines" => Self::Lines,
            "points" => Self::Points,
            s => return Err(LuaError::runtime(format!("invalid topology {s:?}"))),
        })
    }
}

impl Topology {
    #[inline]
    pub fn lua_str(&self) -> &'static str {
        match self {
            Self::Triangles => "triangles",
            Self::Lines => "lines",
            Self::Points => "points",
        }
    }
}

impl IntoLua for Topology {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}
