use crate::core::Context;
use crate::gfx::{Vertex, VertexBuffer, VertexBufferRef};
use crate::lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct VertexBufferModule;

impl LuaModule for VertexBufferModule {
    const PATH: &'static str = "VertexBuffer";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for VertexBufferModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, cap: usize| {
            let ctx = Context::from_lua(lua);
            Ok(VertexBuffer::new(
                ctx.graphics.device(),
                ctx.graphics.queue().clone(),
                cap,
            ))
        });
        methods.add_function("with", |lua, verts: Vec<Vertex>| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.create_vertex_buffer(&verts))
        });
        add_methods(methods);
    }
}

impl UserData for VertexBuffer {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("len", |_, this: VertexBufferRef| Ok(this.count()));
    methods.add_function("capacity", |_, this: VertexBufferRef| Ok(this.capacity()));
    methods.add_function(
        "upload",
        |_, (this, verts): (VertexBufferRef, Vec<Vertex>)| {
            this.upload(&verts).map_err(LuaError::external)
        },
    );
}

impl FromLua for VertexBuffer {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}
