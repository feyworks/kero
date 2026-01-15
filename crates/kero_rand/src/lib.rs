//! Random number generation.

mod rand;

#[cfg(feature = "lua")]
mod rand_lua;

pub use rand::*;

#[cfg(feature = "lua")]
pub use rand_lua::*;
