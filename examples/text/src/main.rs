use polywog::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();
    polywog::new_game()
        .with_title("Text")
        .with_size(1280, 720)
        .run::<TextExample>()
}

pub struct TextExample {
    font: Font,
    font_tex: Texture,
}

impl Game for TextExample {
    fn new(ctx: &Context) -> Result<Self, GameError> {
        let (font, font_tex) = Font::from_ttf_bytes(
            &ctx.graphics,
            include_bytes!("../assets/NotoSans-Regular.ttf"),
            32.0,
            false,
            BASIC_LATIN.chars(),
        )?
        .ok_or_else(|| GameError::custom("failed to load font"))?;
        Ok(Self { font, font_tex })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        //draw.push_scale_of(1.0 / ctx.window.scale_factor());

        let mouse = ctx.mouse.pos();
        draw.text_ext(
            &self.font,
            "Thinking meat! You're asking me to believe in thinking meat!",
            16.0,
            mouse,
            Rgba8::WHITE,
        );

        //draw.pop_transform()?;

        Ok(())
    }
}
