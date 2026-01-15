use crate::color::Rgba8;
use crate::gfx::{ColorMode, Vertex};
use crate::lua::{Handle, LuaModule, Temp};
use crate::math::{Vec2F, vec2};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, Number, Value};

pub struct VertexModule;

impl LuaModule for VertexModule {
    const PATH: &'static str = "Vertex";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Vertex>::register(lua, "Vertex", |members| {
            members.getter("pos", |this| this.pos)?;
            members.setter("pos", |this, val: Vec2F| {
                this.pos = val;
            })?;
            members.getter("tex", |this| this.tex)?;
            members.setter("tex", |this, val: Vec2F| {
                this.tex = val;
            })?;
            members.getter("col", |this| this.col)?;
            members.setter("col", |this, val: Rgba8| {
                this.col = val;
            })?;
            members.getter("mode", |this| this.mode)?;
            members.setter("mode", |this, val: ColorMode| {
                this.mode = val;
            })?;
            members.method_mut(
                "set_pos",
                |this, (a, b): (Either<Vec2F, f32>, Option<f32>)| {
                    this.pos = match a {
                        Either::Left(a) => a,
                        Either::Right(a) => vec2(a, b.unwrap()),
                    };
                },
            )?;
            members.method_mut(
                "set_tex",
                |this, (a, b): (Either<Vec2F, f32>, Option<f32>)| {
                    this.tex = match a {
                        Either::Left(a) => a,
                        Either::Right(a) => vec2(a, b.unwrap()),
                    };
                },
            )?;
            members.method_mut("set_col", |this, val: Rgba8| {
                this.col = val;
            })?;
            members.method_mut("set_mode", |this, val: ColorMode| {
                this.mode = val;
            })?;
            Ok(())
        })?;
        module.set(
            "new",
            lua.create_function(
                |_,
                 (a, b, c, d, e, f): (
                    Either<Vec2F, f32>,
                    Either<Vec2F, f32>,
                    Number,
                    Number,
                    Option<Rgba8>,
                    Option<ColorMode>,
                )| {
                    let (pos, tex, col, mode) = match a {
                        Either::Left(a) => (
                            a,
                            b.unwrap_left(),
                            Rgba8::unpack(c as u32),
                            ColorMode::unpack(d as u32),
                        ),
                        Either::Right(a) => (
                            vec2(a, b.unwrap_right()),
                            vec2(c as f32, d as f32),
                            e.unwrap(),
                            f.unwrap(),
                        ),
                    };
                    Ok(Vertex::new(pos, tex, col, mode))
                },
            )?,
        )?;
        module.set(
            "simple",
            lua.create_function(
                |_,
                 (a, b, c, d): (
                    Either<Vec2F, f32>,
                    Either<Vec2F, f32>,
                    Option<f32>,
                    Option<f32>,
                )| {
                    let (pos, tex) = match a {
                        Either::Left(a) => (a, b.unwrap_left()),
                        Either::Right(a) => {
                            (vec2(a, b.unwrap_right()), vec2(c.unwrap(), d.unwrap()))
                        }
                    };
                    Ok(Vertex::simple(pos, tex))
                },
            )?,
        )?;
        module.set(
            "mult",
            lua.create_function(
                |_,
                 (a, b, c, d, e): (
                    Either<Vec2F, f32>,
                    Either<Vec2F, f32>,
                    Number,
                    Option<f32>,
                    Option<Rgba8>,
                )| {
                    let (pos, tex, col) = match a {
                        Either::Left(a) => (a, b.unwrap_left(), Rgba8::unpack(c as u32)),
                        Either::Right(a) => (
                            vec2(a, b.unwrap_right()),
                            vec2(c as f32, d.unwrap()),
                            e.unwrap(),
                        ),
                    };
                    Ok(Vertex::mult(pos, tex, col))
                },
            )?,
        )?;
        module.set(
            "wash",
            lua.create_function(
                |_,
                 (a, b, c, d, e): (
                    Either<Vec2F, f32>,
                    Either<Vec2F, f32>,
                    Number,
                    Option<f32>,
                    Option<Rgba8>,
                )| {
                    let (pos, tex, col) = match a {
                        Either::Left(a) => (a, b.unwrap_left(), Rgba8::unpack(c as u32)),
                        Either::Right(a) => (
                            vec2(a, b.unwrap_right()),
                            vec2(c as f32, d.unwrap()),
                            e.unwrap(),
                        ),
                    };
                    Ok(Vertex::wash(pos, tex, col))
                },
            )?,
        )?;
        module.set(
            "veto",
            lua.create_function(
                |_, (a, b, c): (Either<Vec2F, f32>, Number, Option<Rgba8>)| {
                    let (pos, col) = match a {
                        Either::Left(a) => (a, Rgba8::unpack(b as u32)),
                        Either::Right(a) => (vec2(a, b as f32), c.unwrap()),
                    };
                    Ok(Vertex::veto(pos, col))
                },
            )?,
        )?;
        module.set(
            "misc",
            lua.create_function(
                |_,
                 (a, b, c, d, e): (
                    Either<Vec2F, f32>,
                    Either<Vec2F, f32>,
                    Number,
                    Option<f32>,
                    Option<Rgba8>,
                )| {
                    let (pos, tex, col) = match a {
                        Either::Left(a) => (a, b.unwrap_left(), Rgba8::unpack(c as u32)),
                        Either::Right(a) => (
                            vec2(a, b.unwrap_right()),
                            vec2(c as f32, d.unwrap()),
                            e.unwrap(),
                        ),
                    };
                    Ok(Vertex::misc(pos, tex, col))
                },
            )?,
        )?;
        Ok(Value::Table(module))
    }
}

impl FromLua for Vertex {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for Vertex {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}
