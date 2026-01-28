use crate::entity::ent_cleanup;
use crate::{Component, ComponentData, ComponentType, EntityExt, EntityObj, Registry, WorldObj};
use fnv::FnvHashMap;
use kero::gfx::Draw;
use kero::lua::UserDataOf;
use kero::math::Vec2F;
use mlua::Lua;
use mlua::prelude::{LuaError, LuaResult};
use std::ffi::c_void;
use std::mem::take;

#[derive(Debug)]
pub struct World {
    pub(crate) entities: Vec<Option<EntityObj>>,
    pub(crate) cleanup: bool,
    by_type: FnvHashMap<*const c_void, Vec<Component>>,
    render_list: Vec<RenderComp>,
}

#[derive(Debug)]
struct RenderComp {
    depth: f64,
    flags: u64,
    pos: Vec2F,
    comp: Component,
}

impl World {
    #[inline]
    pub fn new(lua: &Lua) -> UserDataOf<Self> {
        UserDataOf::new(
            lua,
            Self {
                entities: Vec::new(),
                cleanup: false,
                by_type: FnvHashMap::default(),
                render_list: Vec::new(),
            },
        )
    }

    #[inline]
    fn index_of_ptr(&self, ptr: *const c_void) -> Option<usize> {
        self.entities
            .iter()
            .position(|e| e.as_ref().is_some_and(|e| e.ptr() == ptr))
    }

    #[inline]
    fn remove_component_lookups(&mut self, ent: &EntityObj) -> LuaResult<()> {
        ent.for_each(|comp| {
            let list = self.by_type.get_mut(&comp.type_ptr()).unwrap();
            let ty_idx = list.iter().position(|c| comp.ptr_eq(c)).unwrap();
            list.swap_remove(ty_idx);
            Ok(())
        })
    }

    #[inline]
    pub fn find_with_type<C: ComponentType>(
        &self,
        lua: &Lua,
    ) -> LuaResult<Option<ComponentData<C>>> {
        let type_ptr = Registry::get(lua).rust_type_ptr::<C>()?;
        Ok(self
            .by_type
            .get(&type_ptr)
            .and_then(|list| list.first().cloned())
            .map(|c| c.cast()))
    }

    #[inline]
    pub fn type_slice<C: ComponentType>(&self, lua: &Lua) -> LuaResult<Option<&[Component]>> {
        let type_ptr = Registry::get(lua).rust_type_ptr::<C>()?;
        Ok(self.by_type.get(&type_ptr).map(|v| v.as_slice()))
    }
}

#[inline]
fn world_cleanup(this: &WorldObj) {
    let mut this = this.get_mut();
    if this.cleanup {
        this.cleanup = false;
        this.entities.retain(|c| c.is_some());
        for ent in this.entities.iter().flatten() {
            ent_cleanup(ent);
        }
    }
}

pub trait WorldExt: crate::private::Sealed {
    fn add(&self, lua: &Lua, ent: EntityObj) -> LuaResult<()>;
    fn remove(&self, lua: &Lua, ent: EntityObj) -> LuaResult<()>;
    fn clear(&self, lua: &Lua) -> LuaResult<()>;
    fn find_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<Option<ComponentData<C>>>;
    fn find_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<Option<Component>>;
    fn find_all_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<Vec<Component>>;
    fn for_each(&self, f: impl FnMut(EntityObj) -> LuaResult<()>) -> LuaResult<()>;
    fn for_each_component(&self, f: impl FnMut(Component) -> LuaResult<()>) -> LuaResult<()>;
    fn update(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()>;
    fn render(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()>;
}

impl WorldExt for WorldObj {
    #[inline]
    fn add(&self, lua: &Lua, ent: EntityObj) -> LuaResult<()> {
        let len = {
            let ent = ent.get();
            if ent.world.is_some() {
                return Err(LuaError::runtime("entity is already in a world"));
            }
            ent.components.len()
        };

        // wrap world/entity edits so our handles get dropped
        {
            // add the entity to the world
            let mut this = self.get_mut();
            this.render_list.clear();
            this.entities.push(Some(ent.clone()));

            // set the entity's world
            let mut ent = ent.get_mut();
            ent.world = Some(self.clone());

            // notify the world if the entity needs cleaned up
            if ent.cleanup {
                this.cleanup = true;
            }

            // add all spawned components to their respective type lists
            for i in 0..len {
                if let Some(comp) = ent.components[i].clone() {
                    this.by_type
                        .entry(comp.type_ptr())
                        .or_insert_with(Vec::new)
                        .push(comp);
                }
            }
        }

        // notify all components that they've spawned
        for i in 0..len {
            if let Some(comp) = ent.get().components[i].clone() {
                comp.do_spawned(lua)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn remove(&self, lua: &Lua, ent: EntityObj) -> LuaResult<()> {
        // despawn components
        ent.for_each(|comp| comp.do_despawned(lua))?;

        let mut this = self.get_mut();

        // remove components from type lookups
        this.remove_component_lookups(&ent)?;

        // remove the entity from the world
        let ent_idx = this
            .index_of_ptr(ent.ptr())
            .ok_or_else(|| LuaError::runtime("entity not in world"))?;
        this.entities[ent_idx] = None;
        ent.get_mut().world = None;
        Ok(())
    }

    #[inline]
    fn clear(&self, lua: &Lua) -> LuaResult<()> {
        let len = self.get().entities.len();
        for ent_idx in 0..len {
            let Some(ent) = self.get().entities[ent_idx].clone() else {
                continue;
            };

            // despawn components
            ent.for_each(|comp| comp.do_despawned(lua))?;

            let mut this = self.get_mut();

            // remove components from type lookups
            this.remove_component_lookups(&ent)?;

            // remove the entity from the world
            this.entities[ent_idx] = None;
            ent.get_mut().world = None;
        }
        Ok(())
    }

    #[inline]
    fn find_with_type<C: ComponentType>(&self, lua: &Lua) -> LuaResult<Option<ComponentData<C>>> {
        self.get().find_with_type(lua)
    }

    #[inline]
    fn find_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<Option<Component>> {
        let type_ptr = Registry::get(lua).name_type_ptr(type_name)?;
        Ok(self
            .get()
            .by_type
            .get(&type_ptr)
            .and_then(|list| list.first().cloned()))
    }

    #[inline]
    fn find_all_with_type_name(&self, lua: &Lua, type_name: &str) -> LuaResult<Vec<Component>> {
        let type_ptr = Registry::get(lua).name_type_ptr(type_name)?;
        Ok(self
            .get()
            .by_type
            .get(&type_ptr)
            .map(Vec::clone)
            .unwrap_or_else(Vec::new))
    }

    #[inline]
    fn for_each(&self, mut f: impl FnMut(EntityObj) -> LuaResult<()>) -> LuaResult<()> {
        let len = self.get().entities.len();
        for idx in 0..len {
            let Some(ent) = self.get().entities[idx].clone() else {
                continue;
            };
            f(ent)?;
        }
        Ok(())
    }

    #[inline]
    fn for_each_component(&self, mut f: impl FnMut(Component) -> LuaResult<()>) -> LuaResult<()> {
        self.for_each(|ent| ent.for_each(&mut f))
    }

    #[inline]
    fn update(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()> {
        self.get_mut().render_list.clear();
        world_cleanup(self);
        self.for_each(|ent| ent.update(lua, mask))
    }

    #[inline]
    fn render(&self, lua: &Lua, mask: Option<u64>) -> LuaResult<()> {
        world_cleanup(self);

        // prepare components for rendering
        let mut list = take(&mut self.get_mut().render_list);
        if list.is_empty() {
            for ent in self
                .get()
                .entities
                .iter()
                .flatten()
                .filter(|e| e.get().visible)
            {
                let ent = ent.get();
                let pos = ent.pos();
                for comp in ent.components.iter().flatten() {
                    if comp.visible() {
                        let comp = comp.clone();
                        let flags = comp.flags();
                        let depth = comp.depth();
                        list.push(RenderComp {
                            flags,
                            depth,
                            pos,
                            comp,
                        });
                    }
                }
            }
            list.sort_by(|a, b| b.depth.total_cmp(&a.depth));
        }

        // if we have nothing to draw, bail
        if list.is_empty() {
            self.get_mut().render_list = list;
            return Ok(());
        }

        // initialize the transform state and make sure we maintain the matrix stack
        let stack_size = {
            let draw = Draw::from_lua(lua)?;
            let transform = *draw.transform();
            draw.push_new_transform(transform);
            draw.transform_count()
        };

        // render all visible renderer components
        if let Some(mask) = mask {
            for rend in list.iter().filter(|r| r.flags & mask != 0) {
                rend.comp.do_render(lua, rend.pos)?;
            }
        } else {
            for rend in &list {
                rend.comp.do_render(lua, rend.pos)?;
            }
        }
        self.get_mut().render_list = list;

        // return the transform state to what it was before render
        {
            let draw = Draw::from_lua(lua)?;
            if draw.transform_count() != stack_size {
                return Err(LuaError::runtime(
                    "world render did not push/pop transforms uniformly",
                ));
            }
            draw.pop_transform().map_err(LuaError::external)
        }
    }
}
