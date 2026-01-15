use crate::core::Context;
use crate::gfx::{ParamType, Shader, ShaderRef, UniformType};
use crate::lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, UserData, UserDataMethods, UserDataRef, Value};

pub struct ShaderModule;

impl LuaModule for ShaderModule {
    const PATH: &'static str = "Shader";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for ShaderModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("default", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.default_shader_userdata().clone())
        });
        methods.add_function("new", |lua, source: BorrowedStr| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.graphics.create_shader(source.as_ref()))
        });
        add_methods(methods);
    }
}

impl UserData for Shader {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("params", |lua, this: ShaderRef| {
        let t = lua.create_table()?;
        for p in this.param_defs().defs.iter() {
            t.raw_set(p.name.as_str(), p.ty)?;
        }
        Ok(t)
    });
    methods.add_function("param_type", |_, (this, name): (ShaderRef, BorrowedStr)| {
        Ok(this
            .param_defs()
            .defs
            .iter()
            .find(|p| p.name == name.as_ref())
            .map(|p| p.ty))
    });
}

impl FromLua for Shader {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        UserDataRef::<Self>::from_lua(value, lua).map(|h| h.clone())
    }
}

impl FromLua for ParamType {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "texture" => Self::Texture,
            "sampler" => Self::Sampler,
            "int" => Self::Uniform(UniformType::Int),
            "uint" => Self::Uniform(UniformType::Uint),
            "float" => Self::Uniform(UniformType::Float),
            "vec2" => Self::Uniform(UniformType::Vec2),
            "vec3" => Self::Uniform(UniformType::Vec3),
            "vec4" => Self::Uniform(UniformType::Vec4),
            "mat2" => Self::Uniform(UniformType::Mat2),
            "mat3" => Self::Uniform(UniformType::Mat3),
            "mat4" => Self::Uniform(UniformType::Mat4),
            s => return Err(LuaError::runtime(format!("invalid param type [{s}]"))),
        })
    }
}

impl ParamType {
    #[inline]
    pub fn lua_str(self) -> &'static str {
        match self {
            Self::Texture => "texture",
            Self::Sampler => "sampler",
            Self::Uniform(u) => match u {
                UniformType::Int => "int",
                UniformType::Uint => "uint",
                UniformType::Float => "float",
                UniformType::Vec2 => "vec2",
                UniformType::Vec3 => "vec3",
                UniformType::Vec4 => "vec4",
                UniformType::Mat2 => "mat2",
                UniformType::Mat3 => "mat3",
                UniformType::Mat4 => "mat4",
            },
        }
    }
}

impl IntoLua for ParamType {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}
