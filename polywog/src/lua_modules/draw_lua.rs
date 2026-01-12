use crate::gfx::{BlendMode, ColorMode, Draw, Sampler};
use crate::lua_modules::{ShaderRef, SurfaceRef, TextureRef};
use fey_color::{Rgba8, rgba};
use fey_lua::LuaModule;
use fey_math::{
    Affine2F, LineF, Mat2F, Mat3F, Mat4F, Mat4Ref, Numeric, QuadF, RadiansF, RectF, RectU, Vec2F,
    Vec3F, Vec4F, line, vec2,
};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, Either, IntoLua, Lua, Number, Table, UserData, UserDataMethods, Value};
use std::ops::Deref;

impl Draw {
    pub fn from_lua(lua: &Lua) -> LuaResult<&mut Draw> {
        // SAFETY: app_data_mut() will panic if the pointer is attempted to be borrowed twice
        let draw = *lua
            .app_data_mut::<*mut Draw>()
            .ok_or_else(|| LuaError::runtime("cannot draw outside of render()"))?
            .deref();
        Ok(unsafe { &mut *draw })
    }
}

pub struct DrawModule;

impl LuaModule for DrawModule {
    const PATH: &'static str = "Draw";

    fn load(lua: &Lua) -> LuaResult<Value> {
        Self.into_lua(lua)
    }
}

impl UserData for DrawModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl UserData for Draw {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    #[inline]
    fn num_col(num: Option<Number>) -> Option<Rgba8> {
        num.map(|n| n as u32).map(rgba)
    }
    #[inline]
    fn num_col_or_white(num: Option<Number>) -> Rgba8 {
        num_col(num).unwrap_or(Rgba8::WHITE)
    }

    methods.add_function(
        "set_surface",
        |lua, (surf, col): (Option<SurfaceRef>, Option<Rgba8>)| {
            let draw = Draw::from_lua(lua)?;
            draw.set_surface(surf.map(|s| s.clone()), col.unwrap_or(Rgba8::WHITE));
            Ok(())
        },
    );
    methods.add_function("set_layer", |lua, layer: usize| {
        Draw::from_lua(lua)?.set_layer(layer);
        Ok(())
    });
    methods.add_function("set_shader", |lua, shader: Option<ShaderRef>| {
        Draw::from_lua(lua)?.set_shader(shader.map(|s| s.clone()));
        Ok(())
    });
    methods.add_function("set_param_i32", |lua, (name, value): (BorrowedStr, i32)| {
        Draw::from_lua(lua)?.set_param_i32(&name, value);
        Ok(())
    });
    methods.add_function("set_param_u32", |lua, (name, value): (BorrowedStr, u32)| {
        Draw::from_lua(lua)?.set_param_u32(&name, value);
        Ok(())
    });
    methods.add_function("set_param_f32", |lua, (name, value): (BorrowedStr, f32)| {
        Draw::from_lua(lua)?.set_param_f32(&name, value);
        Ok(())
    });
    methods.add_function(
        "set_param_vec2",
        |lua, (name, value): (BorrowedStr, Vec2F)| {
            Draw::from_lua(lua)?.set_param_vec2(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_vec3",
        |lua, (name, value): (BorrowedStr, Vec3F)| {
            Draw::from_lua(lua)?.set_param_vec3(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_vec4",
        |lua, (name, value): (BorrowedStr, Vec4F)| {
            Draw::from_lua(lua)?.set_param_vec4(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat2",
        |lua, (name, value): (BorrowedStr, Mat2F)| {
            Draw::from_lua(lua)?.set_param_mat2(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat3",
        |lua, (name, value): (BorrowedStr, Mat3F)| {
            Draw::from_lua(lua)?.set_param_mat3(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat4",
        |lua, (name, value): (BorrowedStr, Mat4F)| {
            Draw::from_lua(lua)?.set_param_mat4(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_texture",
        |lua, (name, value): (BorrowedStr, TextureRef)| {
            Draw::from_lua(lua)?.set_param_texture(&name, value.clone());
            Ok(())
        },
    );
    methods.add_function(
        "set_param_sampler",
        |lua, (name, value): (BorrowedStr, Sampler)| {
            Draw::from_lua(lua)?.set_param_sampler(&name, value.clone());
            Ok(())
        },
    );
    methods.add_function("set_view_matrix", |lua, value: Mat4Ref| {
        Draw::from_lua(lua)?.set_view_matrix(&value);
        Ok(())
    });
    methods.add_function("main_sampler", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.main_sampler())
    });
    methods.add_function("set_main_sampler", |lua, value: Sampler| {
        Draw::from_lua(lua)?.set_main_sampler(value);
        Ok(())
    });
    methods.add_function("blend_mode", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.blend_mode())
    });
    methods.add_function("set_blend_mode", |lua, value: BlendMode| {
        Draw::from_lua(lua)?.set_blend_mode(value);
        Ok(())
    });
    methods.add_function("clip_rect", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.clip_rect().copied())
    });
    methods.add_function("set_clip_rect", |lua, value: Option<RectU>| {
        Draw::from_lua(lua)?.set_clip_rect(value);
        Ok(())
    });
    methods.add_function("transform", |lua, _: ()| {
        Ok(*Draw::from_lua(lua)?.transform())
    });
    methods.add_function("push_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.push_transform(value);
        Ok(())
    });
    methods.add_function("push_new_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.push_new_transform(value);
        Ok(())
    });
    methods.add_function("set_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.set_transform(value);
        Ok(())
    });
    methods.add_function("push_translation", |lua, value: Vec2F| {
        Draw::from_lua(lua)?.push_translation(value);
        Ok(())
    });
    methods.add_function("push_rotation", |lua, value: RadiansF| {
        Draw::from_lua(lua)?.push_rotation(value);
        Ok(())
    });
    methods.add_function("push_scale", |lua, value: Either<Vec2F, f32>| {
        let draw = Draw::from_lua(lua)?;
        match value {
            Either::Left(s) => draw.push_scale(s),
            Either::Right(s) => draw.push_scale_of(s),
        }
        Ok(())
    });
    methods.add_function(
        "push_trs",
        |lua, (pos, rot, scale): (Vec2F, RadiansF, Either<Vec2F, f32>)| {
            let scale = match scale {
                Either::Left(s) => s,
                Either::Right(s) => vec2(s, s),
            };
            Draw::from_lua(lua)?.push_trs(pos, rot, scale);
            Ok(())
        },
    );
    methods.add_function("pop_transform", |lua, _: ()| {
        Draw::from_lua(lua)?
            .pop_transform()
            .map_err(LuaError::external)
    });
    methods.add_function("pop_transforms", |lua, count: usize| {
        Draw::from_lua(lua)?
            .pop_transforms(count)
            .map_err(LuaError::external)
    });
    methods.add_function(
        "texture_quad",
        |lua,
         (tex, quad, col, mode, fx, fy): (
            TextureRef,
            QuadF,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let tex = tex.deref();
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.textured_quad_ext(tex, quad, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.textured_quad_flipped(tex, quad, col, mode, vec2(fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function(
        "texture_at",
        |lua,
         (tex, pos, col, mode, fx, fy): (
            TextureRef,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let tex = tex.deref();
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.texture_at_ext(tex, pos, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.texture_at_flipped(tex, pos, col, mode, vec2(fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function("point", |lua, (pos, col): (Vec2F, Rgba8)| {
        Draw::from_lua(lua)?.point(pos, col);
        Ok(())
    });
    methods.add_function("points", |lua, (points, col): (Table, Rgba8)| {
        Draw::from_lua(lua)?.points(
            points.sequence_values::<Vec2F>().filter_map(|p| p.ok()),
            col,
        );
        Ok(())
    });
    // methods.add_function(
    //     "line",
    //     |lua,
    //      (a, b, c, d, e): (
    //         Either<f32, Either<Vec2F, LineF>>, // x1    | from  | line
    //         Either<Number, Vec2F>,             // y1    | to    | color
    //         Option<Number>,                    // x2    | color
    //         Option<Number>,                    // y2
    //         Option<Rgba8>,                     // color
    //     )| {
    //         let (line, col) = match a {
    //             Either::Left(a) => (
    //                 line(
    //                     vec2(a, b.left().unwrap().to_f32()),
    //                     vec2(c.unwrap().to_f32(), d.unwrap().to_f32()),
    //                 ),
    //                 e.unwrap(),
    //             ),
    //             Either::Right(a) => match a {
    //                 Either::Left(a) => (line(a, b.right().unwrap()), num_col_or_white(c)),
    //                 Either::Right(a) => (a, num_col_or_white(b.left())),
    //             },
    //         };
    //         Draw::from_lua(lua)?.line(line, col);
    //         Ok(())
    //     },
    // );
    methods.add_function("line", |lua, (line, col): (LineF, Option<Rgba8>)| {
        // let (line, col) = match a {
        //     Either::Left(a) => (
        //         line(
        //             vec2(a, b.left().unwrap().to_f32()),
        //             vec2(c.unwrap().to_f32(), d.unwrap().to_f32()),
        //         ),
        //         e.unwrap(),
        //     ),
        //     Either::Right(a) => match a {
        //         Either::Left(a) => (line(a, b.right().unwrap()), num_col_or_white(c)),
        //         Either::Right(a) => (a, num_col_or_white(b.left())),
        //     },
        // };
        Draw::from_lua(lua)?.line(line, col.unwrap_or(Rgba8::WHITE));
        Ok(())
    });

    //
    // ---Draw lines connecting the series of points into a chain, optionally looping to the start.
    // ---@param self Draw
    // ---@param points Vec2[]
    // ---@param color Color
    // ---@param loops boolean
    // function methods.lines(self, points, color, loops) end
    //
    // ---Draw a filled triangle.
    // ---@param self Draw
    // ---@param a Vec2
    // ---@param b Vec2
    // ---@param c Vec2
    // ---@param color Color
    // function methods.triangle(self, a, b, c, color) end
    //
    // ---Draw a filled triangle.
    // ---@param self Draw
    // ---@param tri Triangle
    // ---@param color Color
    // function methods.triangle(self, tri, color) end
    //
    // ---Draw a triangle outline.
    // ---@param self Draw
    // ---@param a Vec2
    // ---@param b Vec2
    // ---@param c Vec2
    // ---@param color Color
    // function methods.triangle_outline(self, a, b, c, color) end
    //
    // ---Draw a triangle outline.
    // ---@param self Draw
    // ---@param tri Triangle
    // ---@param color Color
    // function methods.triangle_outline(self, tri, color) end
    //
    // ---Draw a filled quad.
    // ---@param self Draw
    // ---@param a Vec2
    // ---@param b Vec2
    // ---@param c Vec2
    // ---@param d Vec2
    // ---@param color Color
    // function methods.quad(self, a, b, c, d, color) end
    //
    // ---Draw a filled quad.
    // ---@param self Draw
    // ---@param quad Quad
    // ---@param color Color
    // function methods.quad(self, quad, color) end
    //
    // ---Draw a quad outline.
    // ---@param self Draw
    // ---@param a Vec2
    // ---@param b Vec2
    // ---@param c Vec2
    // ---@param d Vec2
    // ---@param color Color
    // function methods.quad_outline(self, a, b, c, d, color) end
    //
    // ---Draw a quad outline.
    // ---@param self Draw
    // ---@param quad Quad
    // ---@param color Color
    // function methods.quad_outline(self, quad, color) end
    //
    // ---Draw a filled rectangle.
    // ---@param self Draw
    // ---@param x number
    // ---@param y number
    // ---@param w number
    // ---@param h number
    // ---@param color Color
    // function methods.rect(self, x, y, w, h, color) end
    //
    // ---Draw a filled rectangle.
    // ---@param self Draw
    // ---@param rect Rect
    // ---@param color Color
    // function methods.rect(self, rect, color) end
    //
    // ---Draw a rectangle outline.
    // ---@param self Draw
    // ---@param x number
    // ---@param y number
    // ---@param w number
    // ---@param h number
    // ---@param color Color
    // function methods.rect_outline(self, x, y, w, h, color) end
    //
    // ---Draw a rectangle outline.
    // ---@param self Draw
    // ---@param rect Rect
    // ---@param color Color
    // function methods.rect_outline(self, rect, color) end
    //
    // ---Draw a filled polygon.
    // ---@param self Draw
    // ---@param poly Polygon
    // ---@param color Color
    // function methods.polygon(self, poly, color) end
    //
    // ---Draw a polygon outline.
    // ---@param self Draw
    // ---@param poly Polygon
    // ---@param color Color
    // function methods.polygon_outline(self, poly, color) end
    //
    // ---Draw a filled circle.
    // ---@param self Draw
    // ---@param x number
    // ---@param y number
    // ---@param radius number
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle(self, x, y, radius, color, seg_count) end
    //
    // ---Draw a filled circle.
    // ---@param self Draw
    // ---@param center Vec2
    // ---@param radius number
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle(self, center, radius, color, seg_count) end
    //
    // ---Draw a filled circle.
    // ---@param self Draw
    // ---@param circ Circle
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle(self, circ, color, seg_count) end
    //
    // ---Draw a circle outline.
    // ---@param self Draw
    // ---@param x number
    // ---@param y number
    // ---@param radius number
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle_outline(self, x, y, radius, color, seg_count) end
    //
    // ---Draw a circle outline.
    // ---@param self Draw
    // ---@param center Vec2
    // ---@param radius number
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle_outline(self, center, radius, color, seg_count) end
    //
    // ---Draw a circle outline.
    // ---@param self Draw
    // ---@param circ Circle
    // ---@param color Color
    // ---@param seg_count integer?
    // function methods.circle_outline(self, circ, color, seg_count) end
    //
    // ---Draw a subtexture.
    // ---@param sub SubTexture
    // ---@param dst Quad
    // ---@param color Color?
    // ---@param mode ColorMode?
    // function methods.subtexture(sub, dst, color, mode) end
    //
    // ---Draw a subtexture.
    // ---@param sub SubTexture
    // ---@param pos Vec2
    // ---@param color Color?
    // ---@param mode ColorMode?
    // function methods.subtexture_at(sub, pos, color, mode) end
    //
    // ---Draw text with the provided font and size.
    // ---@param self DrawMethods
    // ---@param font Font
    // ---@param text string
    // ---@param size number?
    // ---@param pos Vec2
    // ---@param color Color
    // function methods.text(self, font, text, size, pos, color) end
    //
    // ---Draw a custom set of vertices & indices.
    // ---@param self Draw
    // ---@param texture Texture?
    // ---@param topology Topology
    // ---@param vertices Vertex[]
    // ---@param indices integer[]
    // function methods.custom(self, texture, topology, vertices, indices) end
    //
    // ---Draw the provided vertex & index buffers.
    // ---@param self Draw
    // ---@param texture Texture?
    // ---@param topology Topology
    // ---@param vertices VertexBuffer
    // ---@param indices IndexBuffer
    // function methods.buffers(self, texture, topology, vertices, indices) end
}
