![Kero](assets/header.png)

An approachable cross-platform framework for creating 2D games in either Rust, Lua, or both.

> ⚠️ <span style="color:red">**KERO IS CURRENTLY IN UNSTABLE ALPHA TESTING PHASE AND NOT FOR GENERAL USE**</span>

- [✅ Features](#-features)
- [💡 Getting started](#-getting-started)
- [💃 Join the community](#-join-the-community)

## ✅ Features

Kero is a pure-code framework that programmers can use to code their games or even to build their
own game engines. It provides:

- 🖥️ a window, game loop, and rendering context out of the box and ready to go
- 🎮 mouse, keyboard, and gamepad input as well as virtual input mapping
- 🖼️ shaders, surfaces, textures, and other graphics resources
- 🖌️ a straightforward but powerful canvas-style drawing API
- 🧮 various math types for vectors, matrices, rotations, etc.
- 📐 geometry types for various shapes, overlap testing, extraction, raycasting, etc.
- 🎨 tools for working with colors, image encoding, decoding, and manipulation
- 🧳 texture packing and other techniques for rendering optimization
- 🦀 full access to Rust's speed, power, ecosystem, and pleasure of use
- 🌙 full Lua bindings if desired, with LuaLS type annotations

## 💡 Getting started

There's no fancy setup required, Kero is just a normal crate. To create a new empty game project,
first create it and add `kero` as a dependency:

```console
cargo new --bin my_game
cd my_game
cargo add kero
```

Then, replace `src/main.rs` with the following:

```rust
use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("My Game")
        .with_size(1280, 720)
        .run::<MyGame>(())
}

// store your game state and graphics resources here
pub struct MyGame {}

impl Game for MyGame {
    type Config = ();

    // initialize your game state here, such as creating graphics resources, etc.
    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    // perform your game logic here
    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    // perform your drawing code here
    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        Ok(())
    }
}
```

The [examples](https://github.com/feyworks/feyworks/tree/main/crates/kero/examples) folder has a
bunch of examples you can check out to see how different things are done.

## 💃 Join the community

Join our [Discord](https://discord.gg/AYjNw9WHJa) to chat, get help, report bugs, and share what you're working on!

Check out our [{{TODO: Contributing}}]() page if you're interested in helping maintain and improve the
project.

Say hello to our mascot [{{TODO: MASCOT_NAME}}]().