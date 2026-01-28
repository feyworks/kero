use kero::prelude::*;
use kero_ent::{ComponentOf, ComponentType};
use mlua::prelude::LuaResult;
use mlua::{AnyUserData, Lua, UserDataMethods, UserDataRef, UserDataRefMut};

/// Reference to a `Counter` component stored inside `Lua`.
pub type CounterObj = UserDataOf<ComponentOf<Counter>>;

/// Borrow of a `Counter` component stored inside `Lua`.
pub type CounterRef = UserDataRef<ComponentOf<Counter>>;

/// Mutable borrow of a `Counter` component stored inside `Lua`.
pub type CounterMut = UserDataRefMut<ComponentOf<Counter>>;

/// Component that increases a counter every frame and renders a number of boxes representing its
/// current total.
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new(lua: &Lua, flags: u64) -> CounterObj {
        ComponentOf::with_flags(lua, flags, Self { count: 0 })
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

impl ComponentType for Counter {
    const NAME: &'static str = "Counter";
    const PATH: &'static str = "Counter";

    const ADDED_FN: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>> = None;
    const REMOVED_FN: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>> = None;
    const SPAWNED_FN: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>> = None;
    const DESPAWNED_FN: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>> = None;
    const UPDATE_FN: Option<fn(&AnyUserData, &Lua) -> LuaResult<()>> = Some(|this, _| {
        let mut this = this.borrow_mut::<ComponentOf<Self>>()?;
        this.count += 1;
        Ok(())
    });
    const RENDER_FN: Option<fn(&AnyUserData, &Lua, Vec2F) -> LuaResult<()>> =
        Some(|this, lua, pos| {
            let this = this.borrow::<ComponentOf<Self>>()?;
            let draw = Draw::from_lua(lua)?;

            // draw a grid of boxes representing the current count total
            let mut rect = rect(0.0, 0.0, 8.0, 8.0);
            for _ in 0..this.count {
                draw.rect(rect + pos, Rgba8::WHITE);
                rect.x += 10.0;
                if rect.x >= 200.0 {
                    rect.x = 0.0;
                    rect.y += 10.0;
                    if rect.y >= 100.0 {
                        break;
                    }
                }
            }
            Ok(())
        });

    fn methods<T, M: UserDataMethods<T>>(methods: &mut M) {
        // define a constructor
        methods.add_function("new", |lua, flags: u64| Ok(Self::new(lua, flags)));

        // define an immutable method
        methods.add_function("count", |_, this: CounterRef| Ok(this.count()));

        // define a mutable method
        methods.add_function("reset", |_, mut this: CounterMut| {
            this.reset();
            Ok(())
        });
    }
}
