use crate::{Component, ComponentOf, ComponentType, EntityExt, Registry};
use kero::lua::{LuaModule, UserDataOf};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{
    AnyUserData, BorrowedStr, IntoLua, Lua, Table, UserData, UserDataFields, UserDataMethods,
    UserDataRef, UserDataRefMut, Value,
};
use std::marker::PhantomData;

pub type ComponentData<T> = UserDataOf<ComponentOf<T>>;
pub type ComponentRef<T> = UserDataRef<ComponentOf<T>>;
pub type ComponentMut<T> = UserDataRefMut<ComponentOf<T>>;

pub struct ComponentModule;

impl LuaModule for ComponentModule {
    const PATH: &'static str = "Component";

    fn load(lua: &Lua) -> LuaResult<Value> {
        if lua.app_data_ref::<Registry>().is_none() {
            Registry::init(lua)?;
        }

        let module = lua.create_table()?; // Components
        let methods_metatable = module.clone();
        module.set(
            "register",
            lua.create_function(move |lua, (name, class, methods): (String, Table, Table)| {
                let mut types = lua.app_data_mut::<Registry>().unwrap();
                class.set_metatable(Some(class.clone()))?;
                methods.set_metatable(Some(methods_metatable.clone()))?;

                class.set_metatable(Some({
                    let ty_ptr = class.to_pointer();
                    let meta = lua.create_table()?;
                    meta.raw_set(
                        "__tostring",
                        lua.create_function(move |lua, _: ()| {
                            let types = lua.app_data_ref::<Registry>().unwrap();
                            let i = *types.module_lookup.get(&ty_ptr).unwrap();
                            lua.create_string(&types.lua_types[i].type_name)
                        })?,
                    )?;
                    meta.raw_set("__index", methods.clone())?;
                    meta
                }))?;

                if !class.contains_key("__tostring")? {
                    //let name = lua.create_string(&name)?;
                    let name = name.clone();
                    class.raw_set(
                        "__tostring",
                        lua.create_function(move |_, t: Table| {
                            Ok(format!("{}({:016X})", name, t.to_pointer() as usize))
                        })?,
                    )?;
                }

                class.raw_set(
                    "__index",
                    lua.create_function(move |_, (_, k): (Table, Value)| {
                        if let Some(k) = k.as_string().and_then(|k| k.to_str().ok()) {
                            match k.as_ref() {
                                "flags" | "depth" => return Ok(Value::Number(0.0)),
                                "active" | "visible" => return Ok(Value::Boolean(true)),
                                _ => {}
                            }
                        };
                        methods.get::<Value>(k)
                    })?,
                )?;

                types.register_lua(name, class)?;
                Ok(())
            })?,
        )?;

        let methods = lua.create_table()?;

        methods.set(
            "type_name",
            lua.create_function(|lua, this: Component| lua.create_string(this.type_name()))?,
        )?;
        methods.set(
            "world",
            lua.create_function(|_, this: Component| Ok(this.world()))?,
        )?;
        methods.set(
            "get",
            lua.create_function(
                |lua, (this, ty): (Component, BorrowedStr)| match this.entity() {
                    Some(ent) => ent.get().first_with_type_name(lua, ty.as_ref()),
                    None => Ok(None),
                },
            )?,
        )?;
        methods.set(
            "remove_self",
            lua.create_function(|lua, this: Component| match this.entity() {
                Some(ent) => ent.remove(lua, this),
                None => Err(LuaError::runtime("component is not on an entity")),
            })?,
        )?;

        module.set("__index", methods)?;
        module.set_metatable(Some(module.clone()))?;

        Ok(Value::Table(module))
    }
}

pub struct ComponentOfModule<T>(PhantomData<T>);

impl<T: ComponentType> LuaModule for ComponentOfModule<T> {
    const PATH: &'static str = T::PATH;

    fn load(lua: &Lua) -> LuaResult<Value> {
        Self(PhantomData).into_lua(lua)
    }
}

impl<T: ComponentType> UserData for ComponentOfModule<T> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |lua, _, _: ()| lua.create_string(T::NAME));
        T::methods(methods);
    }
}

impl<T: ComponentType> UserData for ComponentOf<T> {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("entity", |_, this| Ok(this.entity.clone()));
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
        fields.add_field_method_get("flags", |_, this| Ok(this.flags));
        fields.add_field_method_set("flags", |_, this, val: u64| {
            this.flags = val;
            Ok(())
        });
        fields.add_field_method_get("depth", |_, this| Ok(this.depth));
        fields.add_field_method_set("depth", |_, this, val: f64| {
            this.depth = val;
            Ok(())
        });
        T::fields(fields);
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function("__tostring", |lua, this: ComponentData<T>| {
            T::tostring(this, lua)
        });
        methods.add_function("type_name", |lua, this: Component| {
            lua.create_string(this.type_name())
        });
        methods.add_function("world", |_, this: Component| Ok(this.world()));
        methods.add_function(
            "get",
            |lua, (this, ty): (Component, BorrowedStr)| match this.entity() {
                Some(ent) => ent.get().first_with_type_name(lua, ty.as_ref()),
                None => Ok(None),
            },
        );
        methods.add_function("remove_self", |lua, this: Component| match this.entity() {
            Some(ent) => ent.remove(lua, this),
            None => Err(LuaError::runtime("component is not on an entity")),
        });
        if T::ADDED_FN.is_some() {
            methods.add_function("added", |lua, this: AnyUserData| {
                (T::ADDED_FN.unwrap())(&this, lua)
            });
        }
        if T::REMOVED_FN.is_some() {
            methods.add_function("removed", |lua, this: AnyUserData| {
                (T::REMOVED_FN.unwrap())(&this, lua)
            });
        }
        if T::SPAWNED_FN.is_some() {
            methods.add_function("spawned", |lua, this: AnyUserData| {
                (T::SPAWNED_FN.unwrap())(&this, lua)
            });
        }
        if T::DESPAWNED_FN.is_some() {
            methods.add_function("despawned", |lua, this: AnyUserData| {
                (T::DESPAWNED_FN.unwrap())(&this, lua)
            });
        }
        if T::UPDATE_FN.is_some() {
            methods.add_function("update", |lua, this: AnyUserData| {
                (T::UPDATE_FN.unwrap())(&this, lua)
            });
        }
        if T::RENDER_FN.is_some() {
            methods.add_function("render", |lua, this: AnyUserData| {
                let comp = this.borrow::<ComponentOf<T>>().unwrap();
                if let Some(pos) = comp.entity().map(|e| e.field(|e| e.pos())) {
                    (T::RENDER_FN.unwrap())(&this, lua, pos?)
                } else {
                    Ok(())
                }
            });
        }
        T::methods(methods);
    }
}
