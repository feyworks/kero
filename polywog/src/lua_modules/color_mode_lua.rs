use crate::color::Channel;
use crate::gfx::ColorMode;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{FromLua, IntoLua, Lua, Value};

pub struct ColorModeModule;

impl LuaModule for ColorModeModule {
    const PATH: &'static str = "ColorMode";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("MULT", ColorMode::MULT)?;
        m.set("WASH", ColorMode::WASH)?;
        m.set("VETO", ColorMode::VETO)?;
        m.set("MISC", ColorMode::MISC)?;
        m.set(
            "new",
            lua.create_function(|_, (mult, wash, veto, edge): (f64, f64, f64, f64)| {
                Ok(ColorMode::new(
                    mult.to_channel(),
                    wash.to_channel(),
                    veto.to_channel(),
                    edge.to_channel(),
                ))
            })?,
        )?;
        m.set(
            "blend",
            lua.create_function(|_, (a, b, t): (ColorMode, ColorMode, f64)| {
                let t: u8 = t.to_channel();
                Ok(ColorMode::new(
                    a.mult.un_lerp(b.mult, t),
                    a.wash.un_lerp(b.wash, t),
                    a.veto.un_lerp(b.veto, t),
                    a.misc.un_lerp(b.misc, t),
                ))
            })?,
        )?;
        m.set(
            "mult",
            lua.create_function(|_, mode: ColorMode| Ok(mode.mult.to_channel::<f64>()))?,
        )?;
        m.set(
            "wash",
            lua.create_function(|_, mode: ColorMode| Ok(mode.wash.to_channel::<f64>()))?,
        )?;
        m.set(
            "veto",
            lua.create_function(|_, mode: ColorMode| Ok(mode.veto.to_channel::<f64>()))?,
        )?;
        m.set(
            "misc",
            lua.create_function(|_, mode: ColorMode| Ok(mode.misc.to_channel::<f64>()))?,
        )?;
        Ok(Value::Table(m))
    }
}

impl FromLua for ColorMode {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        u32::from_lua(value, lua).map(Self::unpack)
    }
}

impl IntoLua for ColorMode {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.pack().into_lua(lua)
    }
}
