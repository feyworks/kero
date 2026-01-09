use crate::gfx::{AddressMode, FilterMode, Sampler};
use crate::lua::{Handle, LuaModule, Temp};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, Value};

pub struct SamplerModule;

impl LuaModule for SamplerModule {
    const PATH: &'static str = "Sampler";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Sampler>::register(lua, "Sampler", |members| {
            members.method("address_x", |this, _: ()| this.address_x)?;
            members.method("address_y", |this, _: ()| this.address_y)?;
            members.method("min_filter", |this, _: ()| this.min_filter)?;
            members.method("mag_filter", |this, _: ()| this.mag_filter)?;
            Ok(())
        })?;
        module.set(
            "default",
            lua.create_function(|_, _: ()| Ok(Sampler::default()))?,
        )?;
        module.set(
            "new",
            lua.create_function(
                |_, (x, y, min, mag): (AddressMode, AddressMode, FilterMode, FilterMode)| {
                    Ok(Sampler::new(x, y, min, mag))
                },
            )?,
        )?;
        module.set(
            "with",
            lua.create_function(|_, (addr, filter): (AddressMode, FilterMode)| {
                Ok(Sampler::with(addr, filter))
            })?,
        )?;
        Ok(Value::Table(module))
    }
}

impl FromLua for Sampler {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for Sampler {
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

impl FromLua for AddressMode {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "clamp" => Self::Clamp,
            "repeat" => Self::Repeat,
            "mirror_repeat" => Self::MirrorRepeat,
            s => return Err(LuaError::runtime(format!("invalid address mode [{s}]"))),
        })
    }
}

impl FromLua for FilterMode {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "nearest" => Self::Nearest,
            "linear" => Self::Linear,
            s => return Err(LuaError::runtime(format!("invalid filter mode [{s}]"))),
        })
    }
}

impl AddressMode {
    #[inline]
    pub fn lua_str(self) -> &'static str {
        match self {
            Self::Clamp => "clamp",
            Self::Repeat => "repeat",
            Self::MirrorRepeat => "mirror_repeat",
        }
    }
}

impl IntoLua for AddressMode {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}

impl FilterMode {
    #[inline]
    pub fn lua_str(self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Linear => "linear",
        }
    }
}

impl IntoLua for FilterMode {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}
