use crate::core::Context;
use crate::gfx::{Surface, SurfaceRef, TextureFormat};
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{FromLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct SurfaceModule;

impl LuaModule for SurfaceModule {
    const PATH: &'static str = "Surface";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SurfaceModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |lua, (w, h, fmt): (u32, u32, Option<TextureFormat>)| {
                let ctx = Context::from_lua(lua);
                let fmt = fmt.unwrap_or(TextureFormat::Rgba8);
                Ok(ctx.graphics.create_surface((w, h), fmt))
            },
        );
        add_methods(methods);
    }
}

impl UserData for Surface {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("texture", |_, this: SurfaceRef| Ok(this.texture().clone()));
    methods.add_function("size", |_, this: SurfaceRef| Ok(this.size()));
    methods.add_function("width", |_, this: SurfaceRef| Ok(this.width()));
    methods.add_function("height", |_, this: SurfaceRef| Ok(this.height()));
    methods.add_function("format", |_, this: SurfaceRef| Ok(this.format()));
    methods.add_function("texture", |_, this: SurfaceRef| Ok(this.texture().clone()));
}

impl FromLua for Surface {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}
