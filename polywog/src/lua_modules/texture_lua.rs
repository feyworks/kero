use crate::core::Context;
use crate::gfx::{Texture, TextureRef};
use crate::img::DynImageRef;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{FromLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct TextureModule;

impl LuaModule for TextureModule {
    const PATH: &'static str = "Texture";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for TextureModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("default", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.default_texture_userdata().clone())
        });
        methods.add_function("from_img", |lua, img: DynImageRef| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.create_texture_from_dyn_img(&img))
        });
        add_methods(methods);
    }
}

impl UserData for Texture {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    // methods.add_function(
    //     "set_pixels",
    //     |lua, (tex, img): (TextureRef, DynImageRef)| {
    //         if tex.format().image_format() != img.format() {
    //             return Err(LuaError::runtime(format!(
    //                 "cannot upload [{}] image to [{}] texture",
    //                 img.format().lua_str(),
    //                 tex.format().lua_str()
    //             )));
    //         }
    //         let gfx = lua.app_data_ref::<Graphics>().unwrap();
    //         tex.upload(img.bytes(), &gfx.queue);
    //         Ok(())
    //     },
    // );

    methods.add_function("size", |_, tex: TextureRef| Ok(tex.size()));
    methods.add_function("width", |_, tex: TextureRef| Ok(tex.width()));
    methods.add_function("height", |_, tex: TextureRef| Ok(tex.height()));
    methods.add_function("format", |_, tex: TextureRef| Ok(tex.format()));
}

impl FromLua for Texture {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}
