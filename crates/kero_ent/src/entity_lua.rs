use crate::{Entity, EntityExt, Registry, WorldExt};
use kero::lua::{LuaModule, UserDataOf};
use kero::math::{Vec2F, vec2};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{
    BorrowedStr, Either, Function, IntoLuaMulti, Lua, MultiValue, UserData, UserDataFields,
    UserDataMethods, UserDataRef, UserDataRefMut, Value,
};

pub type EntityObj = UserDataOf<Entity>;
pub type EntityRef = UserDataRef<Entity>;
pub type EntityMut = UserDataRefMut<Entity>;

pub struct EntityModule;

impl LuaModule for EntityModule {
    const PATH: &'static str = "Entity";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for EntityModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, _: ()| Ok(Entity::new(lua)));
        methods.add_function(
            "new_at",
            |lua, (a, b): (Either<Vec2F, f32>, Option<f32>)| {
                Ok(Entity::new_at(
                    lua,
                    match a {
                        Either::Left(a) => a,
                        Either::Right(a) => vec2(a, b.unwrap()),
                    },
                ))
            },
        );
        add_methods(methods);
    }
}

impl UserData for Entity {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("active", |_, this| Ok(this.active));
        fields.add_field_method_set("active", |_, this, val: bool| {
            this.active = val;
            Ok(())
        });
        fields.add_field_method_get("visible", |_, this| Ok(this.visible));
        fields.add_field_method_set("visible", |_, this, val: bool| {
            this.visible = val;
            Ok(())
        });
        fields.add_field_method_get("pos", |_, this| Ok(this.pos()));
        fields.add_field_method_set("pos", |_, this, val: Vec2F| {
            this.set_pos(val);
            Ok(())
        });
        fields.add_field_method_get("x", |_, this| Ok(this.x()));
        fields.add_field_method_set("x", |_, this, val: f32| {
            this.set_x(val);
            Ok(())
        });
        fields.add_field_method_get("y", |_, this| Ok(this.y()));
        fields.add_field_method_set("y", |_, this, val: f32| {
            this.set_y(val);
            Ok(())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("world", |_, this: EntityRef| Ok(this.world.clone()));
    methods.add_function("remove_self", |lua, this: EntityObj| {
        match this.get().world.clone() {
            Some(world) => world.remove(lua, this),
            None => Err(LuaError::runtime("entity is not in a world")),
        }
    });

    methods.add_function(
        "set_pos",
        |_, (mut this, x, y): (EntityMut, Either<Vec2F, f32>, Option<f32>)| {
            this.set_pos(match x {
                Either::Left(pos) => pos,
                Either::Right(x) => vec2(x, y.unwrap()),
            });
            Ok(())
        },
    );
    methods.add_function("set_x", |_, (mut this, val): (EntityMut, f32)| {
        this.set_x(val);
        Ok(())
    });
    methods.add_function("set_y", |_, (mut this, val): (EntityMut, f32)| {
        this.set_y(val);
        Ok(())
    });
    methods.add_function(
        "move_by",
        |_, (mut this, x, y): (EntityMut, Either<Vec2F, f32>, Option<f32>)| {
            let pos = this.pos();
            this.set_pos(
                pos + match x {
                    Either::Left(pos) => pos,
                    Either::Right(x) => vec2(x, y.unwrap()),
                },
            );
            Ok(())
        },
    );

    methods.add_function("add", |lua, (this, comp): (EntityObj, Value)| {
        this.add(lua, comp)
    });
    methods.add_function("remove", |lua, (this, comp): (EntityObj, Value)| {
        this.remove(lua, comp)
    });
    methods.add_function("remove_all", |lua, (this, ty): (EntityObj, BorrowedStr)| {
        this.remove_all_with_type_name(lua, ty.as_ref())
    });
    methods.add_function("clear", |lua, this: EntityObj| this.clear(lua));
    methods.add_function("get", |lua, (this, ty): (EntityRef, BorrowedStr)| {
        this.first_with_type_name(lua, ty.as_ref())
    });

    methods.add_function(
        "for_each",
        |lua, (this, ty_name, call): (EntityObj, BorrowedStr, Function)| {
            let type_ptr = Registry::get(lua).name_type_ptr(ty_name.as_ref())?;
            let len = this.get().components.len();
            for i in 0..len {
                let Some(comp) = this.get().components.get(i).cloned().flatten() else {
                    continue;
                };
                if comp.type_ptr() == type_ptr {
                    call.call::<()>((comp,))?;
                }
            }
            Ok(())
        },
    );

    struct IterFunc(Function);
    methods.add_function("iter", |lua, this: EntityObj| {
        let func = match lua.app_data_ref::<IterFunc>() {
            Some(func) => func.0.clone(),
            None => {
                let func = lua.create_function(|lua, (ent, mut idx): (EntityObj, usize)| {
                    let ent = ent.get();
                    loop {
                        let Some(comp) = ent.components.get(idx).cloned() else {
                            return Ok(MultiValue::new());
                        };
                        idx += 1;
                        if let Some(comp) = comp {
                            return (idx, comp).into_lua_multi(lua);
                        }
                    }
                })?;
                lua.set_app_data(IterFunc(func.clone()));
                func
            }
        };
        Ok((func, this, 0))
    });

    struct IterTypeFunc(Function);
    methods.add_function(
        "iter_type",
        |lua, (this, ty_name): (EntityObj, BorrowedStr)| {
            let func = match lua.app_data_ref::<IterTypeFunc>() {
                Some(func) => func.0.clone(),
                None => {
                    let type_ptr = Registry::get(lua).name_type_ptr(ty_name.as_ref())?;
                    let func =
                        lua.create_function(move |lua, (ent, mut idx): (EntityObj, usize)| {
                            let ent = ent.get();
                            loop {
                                let Some(comp) = ent.components.get(idx).cloned() else {
                                    return Ok(MultiValue::new());
                                };
                                idx += 1;
                                if let Some(comp) = comp.filter(|c| c.type_ptr() == type_ptr) {
                                    return (idx, comp).into_lua_multi(lua);
                                }
                            }
                        })?;
                    lua.set_app_data(IterTypeFunc(func.clone()));
                    func
                }
            };
            Ok((func, this, 0))
        },
    );
}
