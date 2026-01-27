use crate::SpriteFont;
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::{LuaResult, LuaString};
use mlua::{BorrowedStr, Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};

pub struct SpriteFontModule;

pub type SpriteFontObj = UserDataOf<SpriteFont>;
pub type SpriteFontRef = UserDataRef<SpriteFont>;
pub type SpriteFontMut = UserDataRefMut<SpriteFont>;

impl LuaModule for SpriteFontModule {
    const PATH: &'static str = "SpriteFont";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpriteFontModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl UserData for SpriteFont {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("ascent", |_, this: SpriteFontRef| Ok(this.ascent));
    methods.add_function("descent", |_, this: SpriteFontRef| Ok(this.descent));
    methods.add_function("height", |_, this: SpriteFontRef| Ok(this.height()));
    methods.add_function("line_gap", |_, this: SpriteFontRef| Ok(this.line_gap));
    methods.add_function(
        "kerning",
        |_, (this, left, right): (SpriteFontRef, char, char)| Ok(this.kerning(left, right)),
    );
    methods.add_function(
        "text_width",
        |_, (this, text): (SpriteFontRef, BorrowedStr)| Ok(this.text_width(&text)),
    );
    methods.add_function(
        "text_height",
        |_, (this, text, use_line_gap): (SpriteFontRef, BorrowedStr, Option<bool>)| {
            Ok(this.text_height(&text, use_line_gap.unwrap_or(false)))
        },
    );
    methods.add_function(
        "text_size",
        |_, (this, text, use_line_gap): (SpriteFontRef, BorrowedStr, Option<bool>)| {
            Ok(this.text_size(&text, use_line_gap.unwrap_or(false)))
        },
    );
    methods.add_function(
        "word_wrap",
        |_, (this, width, text): (SpriteFontRef, f32, BorrowedStr)| {
            let mut wrapped = String::with_capacity(text.len());
            let lines = this.word_wrap(width, &text, &mut wrapped);
            Ok((wrapped, lines))
        },
    );
    methods.add_function(
        "draw_text",
        |lua,
         (this, text, pos, col, mode): (
            SpriteFontRef,
            LuaString,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
        )| {
            let draw = Draw::from_lua(lua)?;
            this.draw_text_ext(
                draw,
                text.to_str()?.as_ref(),
                pos,
                col.unwrap_or(Rgba8::WHITE),
                mode.unwrap_or(ColorMode::MULT),
            );
            Ok(())
        },
    );
}
