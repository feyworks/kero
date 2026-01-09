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
            "total",
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
        Ok(Value::Table(m))
    }
}
