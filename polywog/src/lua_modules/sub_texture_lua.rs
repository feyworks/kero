use crate::gfx::{SubTexture, SubTextureRef, TextureRef};
use crate::lua::LuaModule;
use fey_math::{RectF, Vec2F};
use mlua::prelude::LuaResult;
use mlua::{FromLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct SubTextureModule;

impl LuaModule for SubTextureModule {
    const PATH: &'static str = "SubTexture";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SubTextureModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_, (tex, rect, off, size): (TextureRef, RectF, Vec2F, Vec2F)| {
                Ok(SubTexture::new_ext(tex.clone(), rect, off, size))
            },
        );
        add_methods(methods);
    }
}

impl UserData for SubTexture {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("rect", |_, this: SubTextureRef| Ok(this.rect));
    methods.add_function("offset", |_, this: SubTextureRef| Ok(this.offset));
    methods.add_function("size", |_, this: SubTextureRef| Ok(this.size));
    methods.add_function("coords", |_, this: SubTextureRef| {
        let [a, b, c, d] = this.coords;
        Ok((a, b, c, d))
    });
}

impl FromLua for SubTexture {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}
