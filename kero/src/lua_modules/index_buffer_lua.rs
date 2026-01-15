use crate::core::Context;
use crate::gfx::{IndexBuffer, IndexBufferRef};
use crate::lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct IndexBufferModule;

impl LuaModule for IndexBufferModule {
    const PATH: &'static str = "IndexBuffer";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for IndexBufferModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, cap: usize| {
            let ctx = Context::from_lua(lua);
            Ok(IndexBuffer::new(
                ctx.graphics.device(),
                ctx.graphics.queue().clone(),
                cap,
            ))
        });
        methods.add_function("with", |lua, inds: Vec<u32>| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.create_index_buffer(&inds))
        });
        add_methods(methods);
    }
}

impl UserData for IndexBuffer {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("len", |_, this: IndexBufferRef| Ok(this.count()));
    methods.add_function("capacity", |_, this: IndexBufferRef| Ok(this.capacity()));
    methods.add_function("upload", |_, (this, inds): (IndexBufferRef, Vec<u32>)| {
        this.upload(&inds).map_err(LuaError::external)
    });
}

impl FromLua for IndexBuffer {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}
