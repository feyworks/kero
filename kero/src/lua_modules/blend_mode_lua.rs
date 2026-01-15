use crate::gfx::BlendMode;
use mlua::prelude::LuaError;
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, Value};

impl FromLua for BlendMode {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "normal" => BlendMode::Normal,
            "add" => BlendMode::Add,
            "subtract" => BlendMode::Subtract,
            "multiply" => BlendMode::Multiply,
            s => return Err(LuaError::runtime(format!("invalid blend mode {s:?}"))),
        })
    }
}

impl BlendMode {
    #[inline]
    pub fn lua_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
        }
    }
}

impl IntoLua for BlendMode {
    #[inline]
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        self.lua_str().into_lua(lua)
    }
}
