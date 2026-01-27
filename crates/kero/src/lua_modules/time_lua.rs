use crate::core::Context;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

pub struct TimeModule;

impl LuaModule for TimeModule {
    const PATH: &'static str = "Time";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set(
            "fps",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.fps())
            })?,
        )?;
        m.set(
            "delta",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.delta())
            })?,
        )?;
        m.set(
            "since_startup",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.since_startup())
            })?,
        )?;
        m.set(
            "frame",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.frame())
            })?,
        )?;
        m.set(
            "flicker",
            lua.create_function(|lua, (on, off): (f32, Option<f32>)| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.flicker(on, off.unwrap_or(on)))
            })?,
        )?;
        m.set(
            "wave",
            lua.create_function(|lua, (from, to, dur, off): (f32, f32, f32, Option<f32>)| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.time.wave_ext(from, to, dur, off.unwrap_or(0.0)))
            })?,
        )?;
        Ok(Value::Table(m))
    }
}
