//! Aseprite file loader.
//!
//! For a fast workflow, it can be beneficial for an editor/engine to load
//! [Aseprite](https://www.aseprite.org) files directly. This crate provides
//! such functionality, giving you access to all a sprite's layers, animations
//! cels, and all user-data associated with them.

mod ase;
mod blend_mode;
mod cel;
mod error;
mod format;
mod frame;
mod layer;
mod loop_dir;
mod slice;
mod tag;
mod user_data;

pub use ase::*;
pub use blend_mode::*;
pub use cel::*;
pub use error::*;
pub use format::*;
pub use frame::*;
pub use layer::*;
pub use loop_dir::*;
pub use slice::*;
pub use tag::*;
pub use user_data::*;
