use crate::{AnimDir, SpriteAnim};
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::{LuaError, LuaResult, LuaString};
use mlua::{
    BorrowedStr, FromLua, IntoLua, Lua, Table, UserData, UserDataMethods, UserDataRef,
    UserDataRefMut, Value,
};

pub struct SpriteAnimModule;

pub type SpriteAnimObj = UserDataOf<SpriteAnim>;
pub type SpriteAnimRef = UserDataRef<SpriteAnim>;
pub type SpriteAnimMut = UserDataRefMut<SpriteAnim>;

impl LuaModule for SpriteAnimModule {
    const PATH: &'static str = "SpriteAnim";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpriteAnimModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl UserData for SpriteAnim {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl SpriteAnim {
    fn lua_frame(&self, lua: &Lua, idx: usize) -> LuaResult<Option<Table>> {
        let Some(frame) = self.frames.get(idx) else {
            return Ok(None);
        };
        let t = lua.create_table()?;
        t.raw_set("duration", frame.duration)?;
        t.raw_set("cels", {
            let cels = lua.create_table()?;
            for cel in &frame.cels {
                let t = lua.create_table()?;
                t.set("layer", cel.layer)?;
                t.set("index", cel.index)?;
                cels.raw_push(t)?;
            }
            cels
        })?;
        Ok(Some(t))
    }

    fn lua_tag(&self, lua: &Lua, idx: usize) -> LuaResult<Option<Table>> {
        let Some(tag) = self.tags.get(idx) else {
            return Ok(None);
        };
        let t = lua.create_table()?;
        t.raw_set("name", lua.create_string(&tag.name)?)?;
        t.raw_set("from", tag.from)?;
        t.raw_set("to", tag.to)?;
        t.raw_set("dir", tag.dir)?;
        Ok(Some(t))
    }

    fn lua_layer(&self, lua: &Lua, idx: usize) -> LuaResult<Option<Table>> {
        let Some(layer) = self.layers.get(idx) else {
            return Ok(None);
        };
        let t = lua.create_table()?;
        t.raw_set("opacity", layer.opacity)?;
        t.raw_set("name", lua.create_string(&layer.name)?)?;
        t.raw_set("group", layer.group)?;
        t.raw_set("level", layer.level)?;
        Ok(Some(t))
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("size", |_, this: SpriteAnimRef| Ok(this.size));
    methods.add_function("width", |_, this: SpriteAnimRef| Ok(this.size.x));
    methods.add_function("height", |_, this: SpriteAnimRef| Ok(this.size.y));
    methods.add_function("num_frames", |_, this: SpriteAnimRef| Ok(this.frames.len()));
    methods.add_function(
        "frame_duration",
        |_, (this, idx): (SpriteAnimRef, usize)| {
            Ok(this.frames.get(idx).map(|f| f.duration).unwrap_or(0.0))
        },
    );
    methods.add_function("frame", |lua, (this, idx): (SpriteAnimRef, usize)| {
        this.lua_frame(lua, idx)
    });
    methods.add_function(
        "frame_duration",
        |_, (this, idx): (SpriteAnimRef, usize)| {
            Ok(this.frames.get(idx).map(|f| f.duration).unwrap_or(0.0))
        },
    );
    methods.add_function("frames", |lua, this: SpriteAnimRef| {
        let t = lua.create_table()?;
        for i in 0..this.frames.len() {
            t.raw_push(this.lua_frame(lua, i)?)?;
        }
        Ok(t)
    });
    methods.add_function("num_tags", |_, this: SpriteAnimRef| Ok(this.tags.len()));
    methods.add_function("tag", |lua, (this, idx): (SpriteAnimRef, usize)| {
        this.lua_tag(lua, idx)
    });
    methods.add_function(
        "find_tag",
        |lua, (this, name): (SpriteAnimRef, BorrowedStr)| match this
            .tags
            .iter()
            .position(|tag| tag.name == name.as_ref())
        {
            Some(idx) => this.lua_tag(lua, idx),
            None => Ok(None),
        },
    );
    methods.add_function("tags", |lua, this: SpriteAnimRef| {
        let t = lua.create_table()?;
        for i in 0..this.tags.len() {
            t.raw_push(this.lua_tag(lua, i)?)?;
        }
        Ok(t)
    });
    methods.add_function("num_layers", |_, this: SpriteAnimRef| Ok(this.layers.len()));
    methods.add_function("layer", |lua, (this, idx): (SpriteAnimRef, usize)| {
        this.lua_layer(lua, idx)
    });
    methods.add_function("layers", |lua, this: SpriteAnimRef| {
        let t = lua.create_table()?;
        for i in 0..this.tags.len() {
            t.raw_push(this.lua_layer(lua, i)?)?;
        }
        Ok(t)
    });
    methods.add_function(
        "layer_idx",
        |_, (this, name): (SpriteAnimRef, BorrowedStr)| Ok(this.layer_idx(name.as_ref())),
    );
    methods.add_function(
        "layer_mask",
        |_, (this, name): (SpriteAnimRef, BorrowedStr)| Ok(this.layer_mask(name.as_ref())),
    );
    methods.add_function("layer_masks", |_, (this, names): (SpriteAnimRef, Table)| {
        let mut mask = 0;
        for name in names.sequence_values::<BorrowedStr>().flatten() {
            if let Some(m) = this.layer_mask(name.as_ref()) {
                mask |= m;
            }
        }
        Ok(mask)
    });
    methods.add_function(
        "draw",
        |lua,
         (this, frame, pos, layers, col, mode, fx, fy): (
            SpriteAnimRef,
            usize,
            Vec2F,
            Option<u64>,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let layers = layers.unwrap_or(u64::MAX);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    this.draw_ext(draw, frame, pos, layers, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    this.draw_flipped(draw, frame, pos, layers, col, mode, (fx, fy));
                }
            }
            Ok(())
        },
    );
}

impl FromLua for AnimDir {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = LuaString::from_lua(value, lua)?;
        Ok(match s.to_str()?.as_ref() {
            "forward" => AnimDir::Forward,
            "reverse" => AnimDir::Reverse,
            "ping_pong" => AnimDir::PingPong,
            "ping_pong_reverse" => AnimDir::PingPongReverse,
            s => return Err(LuaError::runtime(format!("invalid anim dir [{s}]"))),
        })
    }
}

impl IntoLua for AnimDir {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        lua.create_string(match self {
            AnimDir::Forward => "forward",
            AnimDir::Reverse => "reverse",
            AnimDir::PingPong => "ping_pong",
            AnimDir::PingPongReverse => "ping_pong_reverse",
        })
        .map(Value::String)
    }
}
