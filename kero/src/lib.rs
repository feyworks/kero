pub mod core;
pub mod gfx;
pub mod input;
pub mod misc;
mod new_game;

#[cfg(feature = "lua")]
pub use kero_lua as lua;

#[cfg(feature = "lua")]
pub mod lua_modules;

#[doc(inline)]
pub use kero_color as color;

#[doc(inline)]
pub use kero_grid as grid;

#[doc(inline)]
pub use kero_guid as guid;

#[doc(inline)]
pub use kero_img as img;

#[doc(inline)]
pub use kero_math as math;

#[doc(inline)]
pub use kero_rand as rand;

pub use new_game::new_game;

/// Include all types and traits.
pub mod prelude {
    pub use crate::color::*;
    pub use crate::core::*;
    pub use crate::gfx::*;
    pub use crate::grid::*;
    pub use crate::guid::*;
    pub use crate::img::*;
    pub use crate::input::*;
    pub use crate::math::*;
    pub use crate::misc::*;
    pub use crate::rand::*;
}
