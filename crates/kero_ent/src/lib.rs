//! World, entity, and component system for Kero games.

mod component;
mod component_lua;
mod component_of;
mod component_type;
mod entity;
mod entity_lua;
mod game_builder_ext;
mod into_component;
mod registry;
mod world;
mod world_lua;

pub use component::*;
pub use component_lua::*;
pub use component_of::*;
pub use component_type::*;
pub use entity::*;
pub use entity_lua::*;
pub use game_builder_ext::*;
pub use into_component::*;
pub use registry::*;
pub use world::*;
pub use world_lua::*;

pub(crate) mod private {
    pub trait Sealed {}
    impl Sealed for crate::EntityObj {}
    impl Sealed for super::WorldObj {}
    impl Sealed for kero::core::GameBuilder {}
}
