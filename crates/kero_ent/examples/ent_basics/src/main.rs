mod counter;

use crate::counter::Counter;
use kero::prelude::*;
use kero_ent::GameBuilderExt;

fn main() -> Result<(), GameError> {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    // components can be written in Lua or Rust, and Rust components need
    // to be registered here before the game starts
    kero::new_game()
        .with_default_logger()
        .with_title("Ent Basics")
        .with_size(1280, 720)
        .with_component::<Counter>()?
        .run_lua()
}
