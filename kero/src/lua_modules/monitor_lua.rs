use crate::core::{Context, Monitor, MonitorRef};
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Lua, UserData, UserDataMethods, Value};

pub struct MonitorModule;

impl LuaModule for MonitorModule {
    const PATH: &'static str = "Monitor";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for MonitorModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, true);
    }
}

impl UserData for Monitor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, false);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M, module: bool) {
    if module {
        methods.add_function("all", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.window.monitors().collect::<Vec<_>>())
        });
        methods.add_function("primary", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.window.primary_monitor())
        });
    }
    methods.add_function("name", |_, this: MonitorRef| Ok(this.name()));
    methods.add_function("size", |_, this: MonitorRef| Ok(this.size()));
    methods.add_function("width", |_, this: MonitorRef| Ok(this.size().x));
    methods.add_function("height", |_, this: MonitorRef| Ok(this.size().y));
    methods.add_function("pixel_size", |_, this: MonitorRef| Ok(this.pixel_size()));
    methods.add_function("pixel_width", |_, this: MonitorRef| Ok(this.pixel_size().x));
    methods.add_function("pixel_height", |_, this: MonitorRef| {
        Ok(this.pixel_size().y)
    });
    methods.add_function("pos", |_, this: MonitorRef| Ok(this.pos()));
    methods.add_function("x", |_, this: MonitorRef| Ok(this.pos().x));
    methods.add_function("y", |_, this: MonitorRef| Ok(this.pos().y));
    methods.add_function("pixel_pos", |_, this: MonitorRef| Ok(this.pixel_pos()));
    methods.add_function("pixel_x", |_, this: MonitorRef| Ok(this.pixel_pos().x));
    methods.add_function("pixel_y", |_, this: MonitorRef| Ok(this.pixel_pos().y));
    methods.add_function("refresh_rate", |_, this: MonitorRef| {
        Ok(this.0.refresh_rate_millihertz())
    });
    methods.add_function("scale_factor", |_, this: MonitorRef| {
        Ok(this.0.scale_factor())
    });
    methods.add_function("video_modes", |lua, this: MonitorRef| {
        let modes = lua.create_table()?;
        for mode in this.video_modes() {
            modes.raw_push(mode)?;
        }
        Ok(modes)
    });
}
