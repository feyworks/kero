use crate::core::Context;
use crate::gfx::{Screen, ScreenMut, ScreenRef};
use crate::lua::LuaModule;
use crate::math::{Numeric, Vec2F, vec2};
use mlua::prelude::LuaResult;
use mlua::{Either, Lua, UserData, UserDataMethods, Value};

pub struct ScreenModule;

impl LuaModule for ScreenModule {
    const PATH: &'static str = "Screen";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for ScreenModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new_frame", |lua, (w, h, fract): (u32, u32, bool)| {
            let ctx = Context::from_lua(lua);
            Ok(Screen::new_frame(&ctx, (w, h), fract))
        });
        methods.add_function("new_scaled", |lua, scale: f32| {
            let ctx = Context::from_lua(lua);
            Ok(Screen::new_fill(&ctx, scale))
        });
        add_methods(methods);
    }
}

impl UserData for Screen {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("surface", |_, this: ScreenRef| {
        Ok(this.surface_userdata().clone())
    });
    methods.add_function("size", |_, this: ScreenRef| Ok(this.size().to_f32()));
    methods.add_function("width", |_, this: ScreenRef| Ok(this.width()));
    methods.add_function("height", |_, this: ScreenRef| Ok(this.height()));
    methods.add_function("window_rect", |_, this: ScreenRef| Ok(*this.window_rect()));
    methods.add_function("scale", |_, this: ScreenRef| Ok(this.scale()));
    methods.add_function("mouse_pos", |_, this: ScreenRef| Ok(this.mouse_pos()));
    methods.add_function("mouse_x", |_, this: ScreenRef| Ok(this.mouse_x()));
    methods.add_function("mouse_y", |_, this: ScreenRef| Ok(this.mouse_y()));
    methods.add_function("mouse_y", |_, this: ScreenRef| Ok(this.mouse_y()));
    methods.add_function("update", |lua, mut this: ScreenMut| {
        let ctx = lua.app_data_ref::<Context>().unwrap();
        this.update(&ctx);
        Ok(())
    });
    methods.add_function(
        "map_pos",
        |_, (this, x, y): (ScreenRef, Either<Vec2F, f32>, Option<f32>)| {
            Ok(this
                .map_pos(match x {
                    Either::Left(pos) => pos,
                    Either::Right(x) => vec2(x, y.unwrap()),
                })
                .round())
        },
    );
    // methods.add_function(
    //     "set_as_draw_surface",
    //     |lua, (this, col): (ScreenRef, Option<Rgba8>)| {
    //         let mut gfx = lua.app_data_mut::<Graphics>().unwrap();
    //         this.target_surface(col, &mut gfx);
    //         Ok(())
    //     },
    // );
    // methods.add_function(
    //     "draw_to_window",
    //     |lua, (this, col): (ScreenRef, Option<Rgba8>)| {
    //         let mut gfx = lua.app_data_mut::<Graphics>().unwrap();
    //         this.draw_to_window(col, &mut gfx);
    //         Ok(())
    //     },
    // );
}
