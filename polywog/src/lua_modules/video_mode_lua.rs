use crate::core::VideoMode;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Lua, UserData, UserDataMethods, UserDataRef, Value};

pub type VideoModeRef = UserDataRef<VideoMode>;

pub struct VideoModeModule;

impl LuaModule for VideoModeModule {
    const PATH: &'static str = "VideoMode";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for VideoModeModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, true);
    }
}

impl UserData for VideoMode {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, false);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M, _module: bool) {
    methods.add_function("size", |_, this: VideoModeRef| Ok(this.size()));
    methods.add_function("width", |_, this: VideoModeRef| Ok(this.size().x));
    methods.add_function("height", |_, this: VideoModeRef| Ok(this.size().y));
    methods.add_function("bit_depth", |_, this: VideoModeRef| Ok(this.bit_depth()));
    methods.add_function("refresh_rate", |_, this: VideoModeRef| {
        Ok(this.refresh_rate())
    });
    methods.add_function("monitor", |_, this: VideoModeRef| Ok(this.monitor()));
}
