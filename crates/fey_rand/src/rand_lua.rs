use crate::rand::Rand;
use fey_guid::Guid;
use fey_lua::LuaModule;
use mlua::prelude::{Lua, LuaError, LuaResult};
use mlua::{
    Table, UserData, UserDataFields, UserDataMethods, UserDataRef, UserDataRefMut, Value, Variadic,
};

pub type RandRef = UserDataRef<Rand>;
pub type RandMut = UserDataRefMut<Rand>;

pub struct RandModule;

impl LuaModule for RandModule {
    const PATH: &'static str = "Rand";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for RandModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, seed: Option<u64>| {
            Ok(seed.map(Rand::from_seed).unwrap_or_else(Rand::new))
        });
        add_methods(methods);
    }
}

impl UserData for Rand {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("seed", |_, this: &Rand| Ok(this.seed()));
        fields.add_field_method_set("seed", |_, this: &mut Rand, seed: u64| {
            this.set_seed(seed);
            Ok(())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("clone", |_, this: RandRef| Ok(this.clone()));
    methods.add_function("bool", |_, mut this: RandMut| Ok(this.boolean()));
    methods.add_function("chance", |_, (mut this, chance): (RandMut, f64)| {
        Ok(this.chance(chance))
    });
    methods.add_function(
        "choose",
        |_, (mut this, a, b, args): (RandMut, Value, Value, Variadic<Value>)| {
            let idx = this.range(0..(args.len() + 2));
            Ok(match idx {
                0 => a,
                1 => b,
                i => args.as_slice()[i].clone(),
            })
        },
    );
    methods.add_function("choose_from", |_, (mut this, args): (RandMut, Table)| {
        let idx = this.range(0..args.len()?);
        args.get::<Value>(idx + 1)
    });
    methods.add_function(
        "choose_weighted",
        |_, (mut this, values, weights): (RandMut, Table, Table)| {
            let mut sum: f64 = weights.sequence_values::<f64>().flatten().sum();
            for (i, w) in weights.pairs::<usize, f64>().flatten() {
                if this.range(0.0..sum) < w {
                    return values.get::<Value>(i);
                }
                sum -= w;
            }
            Err(LuaError::runtime("failed to choose weighted value"))
        },
    );
    methods.add_function("clone", |_, this: RandRef| Ok(this.clone()));
    methods.add_function("guid", |_, mut this: RandMut| Ok(Guid::from_rng(&mut this)));
    methods.add_function(
        "int",
        |_, (mut this, min, max): (RandMut, i64, Option<i64>)| {
            Ok(this.range(max.map(|max| min..max).unwrap_or_else(|| 0..min)))
        },
    );
    methods.add_function(
        "float",
        |_, (mut this, min, max): (RandMut, f64, Option<f64>)| {
            Ok(this.range(max.map(|max| min..max).unwrap_or_else(|| 0.0..min)))
        },
    );
    methods.add_function("shuffle", |_, (mut this, list): (RandMut, Table)| {
        let mut n = list.len()?;
        while n > 1 {
            let k = this.range(0..n);
            n -= 1;
            let a = list.get::<Value>(n)?;
            let b = list.get::<Value>(k)?;
            list.set(k, a)?;
            list.set(n, b)?;
        }
        Ok(())
    });
}
