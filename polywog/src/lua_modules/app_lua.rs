use crate::core::Context;
use fey_lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{IntoLua, Lua, UserData, UserDataMethods, Value};

pub struct AppModule;

impl LuaModule for AppModule {
    const PATH: &'static str = "App";

    fn load(lua: &Lua) -> LuaResult<Value> {
        Self.into_lua(lua)
    }
}

impl UserData for AppModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("quit", |lua, _: ()| {
            Context::from_lua(lua).quit();
            Ok(())
        });
        methods.add_function("quit_requested", |lua, _: ()| {
            Ok(Context::from_lua(lua).quit_requested())
        });
        methods.add_function("restart", |lua, _: ()| {
            Context::from_lua(lua).reload_lua();
            Ok(())
        });
        methods.add_function("restart_requested", |lua, _: ()| {
            Ok(Context::from_lua(lua).reload_lua_requested())
        });
        methods.add_function("cache_dir", |lua, _: ()| {
            Context::from_lua(lua).cache_dir().into_lua(lua)
        });
        methods.add_function("config_dir", |lua, _: ()| {
            Context::from_lua(lua).config_dir().into_lua(lua)
        });
        methods.add_function("config_local_dir", |lua, _: ()| {
            Context::from_lua(lua).config_local_dir().into_lua(lua)
        });
        methods.add_function("data_dir", |lua, _: ()| {
            Context::from_lua(lua).data_dir().into_lua(lua)
        });
        methods.add_function("data_local_dir", |lua, _: ()| {
            Context::from_lua(lua).data_local_dir().into_lua(lua)
        });
        methods.add_function("preferences_dir", |lua, _: ()| {
            Context::from_lua(lua).preferences_dir().into_lua(lua)
        });
    }
}
