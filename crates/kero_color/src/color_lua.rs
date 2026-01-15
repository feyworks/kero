use crate::{
    Channel, FromRgb, FromRgba, Grey8, GreyAlpha8, Hsl, Hsv, Oklab, Rgb, Rgb8, Rgba, Rgba8, ToRgb,
    ToRgba,
};
use kero_lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{FromLua, IntoLua, Lua, Value};

pub struct ColorModule;

impl LuaModule for ColorModule {
    const PATH: &'static str = "Color";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;

        macro_rules! constant {
            ($name:literal $value:ident) => {
                m.set($name, lua.create_function(|_, _: ()| Ok(Rgba8::$value))?)?;
            };
        }
        constant!("transparent" TRANSPARENT);
        constant!("black" BLACK);
        constant!("white" WHITE);
        constant!("red" RED);
        constant!("green" GREEN);
        constant!("blue" BLUE);
        constant!("yellow" YELLOW);
        constant!("cyan" CYAN);
        constant!("fuchsia" FUCHSIA);
        constant!("fuchsia" FUCHSIA);

        m.set(
            "pack",
            lua.create_function(|_, (r, g, b, a): (u8, u8, u8, Option<u8>)| {
                Ok(Rgba8::new(r, g, b, a.unwrap_or(u8::MAX)))
            })?,
        )?;
        m.set(
            "packf",
            lua.create_function(|_, (r, g, b, a): (f32, f32, f32, Option<f32>)| {
                Ok(Rgba8::from_rgba(Rgba::new(r, g, b, a.unwrap_or(1.0))))
            })?,
        )?;
        m.set(
            "unpack",
            lua.create_function(|_, Rgba8 { r, g, b, a }: Rgba8| Ok((r, g, b, a)))?,
        )?;
        m.set(
            "unpackf",
            lua.create_function(|_, packed: Rgba8| {
                let Rgba { r, g, b, a } = Rgba::<f32>::from_rgba(packed);
                Ok((r, g, b, a))
            })?,
        )?;
        m.set("get_r", lua.create_function(|_, col: Rgba8| Ok(col.r))?)?;
        m.set("get_g", lua.create_function(|_, col: Rgba8| Ok(col.g))?)?;
        m.set("get_b", lua.create_function(|_, col: Rgba8| Ok(col.b))?)?;
        m.set("get_a", lua.create_function(|_, col: Rgba8| Ok(col.a))?)?;
        m.set(
            "mul",
            lua.create_function(|_, (a, b): (Rgba8, Rgba8)| Ok(a.mul_color(b)))?,
        )?;
        m.set(
            "mul_a",
            lua.create_function(|_, (col, a): (Rgba8, f32)| Ok(col.un_mul(a.to_channel())))?,
        )?;
        m.set(
            "add",
            lua.create_function(|_, (a, b): (Rgba8, Rgba8)| Ok(a.add_color(b)))?,
        )?;
        m.set(
            "sub",
            lua.create_function(|_, (a, b): (Rgba8, Rgba8)| Ok(a.sub_color(b)))?,
        )?;
        m.set(
            "hsl",
            lua.create_function(|_, (h, s, l): (f32, f32, f32)| {
                let col: Rgba8 = Hsl { h: h * 360.0, s, l }.to_rgba();
                Ok(col)
            })?,
        )?;
        m.set(
            "to_hsl",
            lua.create_function(|_, col: Rgb8| {
                let Hsl { h, s, l } = Hsl::<f32>::from_rgb(col);
                Ok((h, s, l))
            })?,
        )?;
        m.set(
            "hsv",
            lua.create_function(|_, (h, s, v): (f32, f32, f32)| {
                let col: Rgba8 = Hsv { h: h * 360.0, s, v }.to_rgba();
                Ok(col)
            })?,
        )?;
        m.set(
            "to_hsv",
            lua.create_function(|_, col: Rgb8| {
                let Hsv { h, s, v } = Hsv::<f32>::from_rgb(col);
                Ok((h, s, v))
            })?,
        )?;
        m.set(
            "oklab",
            lua.create_function(|_, (l, a, b): (f32, f32, f32)| {
                let col: Rgba<f32> = Oklab { l, a, b }.to_rgb().with_a(1.0);
                Ok(Rgba8::from_rgba(col))
            })?,
        )?;
        m.set(
            "to_oklab",
            lua.create_function(|_, col: Rgb8| {
                let col: Rgb<f32> = col.to_rgb();
                let Oklab { l, a, b } = Oklab::<f32>::from_rgb(col);
                Ok((l, a, b))
            })?,
        )?;
        m.set(
            "lerp",
            lua.create_function(|_, (a, b, t): (Rgba8, Rgba8, f32)| {
                let t: u8 = t.to_channel();
                Ok(Rgba8::new(
                    a.r.un_lerp(b.r, t),
                    a.g.un_lerp(b.g, t),
                    a.b.un_lerp(b.b, t),
                    a.a.un_lerp(b.a, t),
                ))
            })?,
        )?;

        Ok(Value::Table(m))
    }
}

impl FromLua for Grey8 {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Rgba8::from_lua(value, lua).map(|c| Grey8::new(c.r))
    }
}

impl IntoLua for Grey8 {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.to_rgba().into_lua(lua)
    }
}

impl FromLua for GreyAlpha8 {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Rgba8::from_lua(value, lua).map(|c| GreyAlpha8::new(c.r, c.a))
    }
}

impl IntoLua for GreyAlpha8 {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.to_rgba().into_lua(lua)
    }
}

impl FromLua for Rgb8 {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Rgba8::from_lua(value, lua).map(|c| Rgb8::new(c.r, c.g, c.b))
    }
}

impl IntoLua for Rgb8 {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.to_rgba().into_lua(lua)
    }
}

impl FromLua for Rgba8 {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        u32::from_lua(value, lua).map(Self::unpack)
    }
}

impl IntoLua for Rgba8 {
    #[inline]
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        self.pack().into_lua(lua)
    }
}
