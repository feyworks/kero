//! A 128-bit globally unique identifier.

mod guid;

#[cfg(feature = "lua")]
mod guid_lua;

pub use guid::*;

#[cfg(feature = "lua")]
pub use guid_lua::*;
