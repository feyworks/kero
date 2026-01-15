use crate::gfx::TextureFormat;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, Value};

impl FromLua for TextureFormat {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "r8" => Self::R8,
            "r16" => Self::R16,
            "r32f" => Self::R32F,
            "rg8" => Self::Rg8,
            "rg16" => Self::Rg16,
            "rg32f" => Self::Rg32F,
            "rgba8" => Self::Rgba8,
            "rgba16" => Self::Rgba16,
            "rgba32f" => Self::Rgba32F,
            s => return Err(LuaError::runtime(format!("invalid texture format [{s}]"))),
        })
    }
}

impl TextureFormat {
    pub fn lua_str(&self) -> &'static str {
        match self {
            Self::R8 => "r8",
            Self::R16 => "r16",
            Self::R32F => "r32f",
            Self::Rg8 => "rg8",
            Self::Rg16 => "rg16",
            Self::Rg32F => "rg32f",
            Self::Rgba8 => "rgba8",
            Self::Rgba16 => "rgba16",
            Self::Rgba32F => "rgba32f",
        }
    }
}

impl IntoLua for TextureFormat {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}
