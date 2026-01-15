use crate::core::GameBuilder;

/// Set forth!
///
/// Here is a minimal example to run your game:
///
/// ```rust
/// use kero::core::{Context, Game, GameError};
/// use kero::gfx::Draw;
///
/// fn main() -> Result<(), GameError> {
///     kero::new_game()
///         .with_title("Minimal")
///         .with_size(1280, 720)
///         .run::<MinimalExample>(())
/// }
///
/// pub struct MinimalExample;
///
/// impl Game for MinimalExample {
///     type Config = ();
///
///     fn new(_ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError> {
///         Ok(Self)
///     }
///
///     fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
///         Ok(())
///     }
///
///     fn render(&mut self, _ctx: &Context, _draw: &mut Draw) -> Result<(), GameError> {
///         Ok(())
///     }
/// }
/// ```
///
/// See the [`Game`](crate::core::Game) trait for more info.
pub fn new_game() -> GameBuilder {
    GameBuilder::new().unwrap()
}
