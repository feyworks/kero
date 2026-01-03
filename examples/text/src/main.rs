use polywog::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();
    polywog::new_game()
        .with_title("Text")
        .with_size(1280, 720)
        .run::<TextExample>()
}

pub struct TextExample {}

impl Game for TextExample {
    fn new(_ctx: &Context) -> Result<Self, GameError> {
        Ok(Self {})
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, _draw: &mut Draw) -> Result<(), GameError> {
        Ok(())
    }
}
