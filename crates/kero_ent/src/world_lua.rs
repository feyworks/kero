use crate::{Entity, World, WorldExt};
use kero::lua::{LuaModule, UserDataOf};
use mlua::prelude::LuaResult;
use mlua::{
    BorrowedStr, Function, IntoLua, IntoLuaMulti, Lua, MultiValue, Table, UserData,
    UserDataMethods, UserDataRef, UserDataRefMut, Value,
};

pub type WorldObj = UserDataOf<World>;
pub type WorldRef = UserDataRef<World>;
pub type WorldMut = UserDataRefMut<World>;

pub struct WorldModule;

impl LuaModule for WorldModule {
    const PATH: &'static str = "World";

    fn load(lua: &Lua) -> LuaResult<Value> {
        Self.into_lua(lua)
    }
}

impl UserData for WorldModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, _: ()| Ok(World::new(lua)));
        add_methods(methods);
    }
}

impl UserData for World {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function(
        "add",
        |lua, (this, ent): (UserDataOf<World>, UserDataOf<Entity>)| this.add(lua, ent),
    );
    methods.add_function(
        "add_many",
        |lua, (this, ents): (UserDataOf<World>, Table)| {
            for ent in ents.sequence_values::<UserDataOf<Entity>>() {
                this.add(lua, ent?)?;
            }
            Ok(())
        },
    );
    methods.add_function(
        "remove",
        |lua, (this, ent): (UserDataOf<World>, UserDataOf<Entity>)| this.remove(lua, ent),
    );
    methods.add_function(
        "remove_many",
        |lua, (this, ents): (UserDataOf<World>, Table)| {
            for ent in ents.sequence_values::<UserDataOf<Entity>>() {
                this.remove(lua, ent?)?;
            }
            Ok(())
        },
    );
    methods.add_function("clear", |lua, this: UserDataOf<World>| this.clear(lua));
    methods.add_function(
        "find",
        |lua, (this, ty): (UserDataOf<World>, BorrowedStr)| {
            this.find_with_type_name(lua, ty.as_ref())
        },
    );
    methods.add_function(
        "find_all",
        |lua, (this, ty, fill): (UserDataOf<World>, BorrowedStr, Option<Table>)| {
            let fill = match fill {
                Some(fill) => {
                    fill.clear()?;
                    fill
                }
                None => lua.create_table()?,
            };
            for comp in this.find_all_with_type_name(lua, ty.as_ref())? {
                fill.raw_push(comp)?;
            }
            Ok(fill)
        },
    );
    methods.add_function(
        "for_each",
        |lua, (this, ty, f): (UserDataOf<World>, BorrowedStr, Function)| {
            let comps = this.find_all_with_type_name(lua, ty.as_ref())?;
            for comp in comps {
                f.call::<()>((comp,))?;
            }
            Ok(())
        },
    );

    struct IterFunc(Function);
    methods.add_function("iter", |lua, this: UserDataOf<World>| {
        let func = match lua.app_data_ref::<IterFunc>() {
            Some(func) => func.0.clone(),
            None => {
                let func =
                    lua.create_function(|lua, (this, mut idx): (UserDataOf<World>, usize)| {
                        let this = this.get();
                        loop {
                            let Some(ent) = this.entities.get(idx).cloned() else {
                                return Ok(MultiValue::new());
                            };
                            idx += 1;
                            if let Some(ent) = ent {
                                return (idx, ent).into_lua_multi(lua);
                            }
                        }
                    })?;
                lua.set_app_data(IterFunc(func.clone()));
                func
            }
        };
        Ok((func, this, 0))
    });

    methods.add_function(
        "update",
        |lua, (this, flags): (UserDataOf<World>, Option<u64>)| this.update(lua, flags),
    );
    methods.add_function(
        "render",
        |lua, (this, flags): (UserDataOf<World>, Option<u64>)| this.render(lua, flags),
    );
}

//
// ---Call a function on each component of the requested type.
// ---@generic T: Component
// ---@param self WorldMethods
// ---@param ty `T`
// ---@param fn fun(comp: T)
// function methods.for_each(self, ty, fn) end
//
// ---Removes all entities from the world.
// ---@param self World
// function methods.clear(self) end
//
// ---Updates all components whose flags match any of the mask bits.
// ---@param self WorldMethods
// ---@param mask integer
// function methods.update(self, mask) end
//
// ---Renders all components whose flags match any of the mask bits.
// ---@param self WorldMethods
// ---@param mask integer
// function methods.render(self, mask) end
