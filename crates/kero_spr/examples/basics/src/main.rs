use kero::prelude::*;
use kero_spr::SpritePacker;

fn main() -> Result<(), GameError> {
    kero::new_game()
        .with_default_logger()
        .with_title("Basics")
        .with_size(1280, 720)
        .run::<BasicsExample>(())
}

pub struct BasicsExample {
    screen: Screen,
}

impl Game for BasicsExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let screen = Screen::new_frame(ctx, (320, 180), false);
        // let screen = Screen::new_fill(ctx, 5.0);

        let mut packer = SpritePacker::new();
        //packer.add_sprite("portrait", )

        Ok(Self { screen })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.screen.update(ctx);
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        self.screen
            .set_as_draw_surface(draw, Rgba8::CORNFLOWER_BLUE);

        draw.line((Vec2F::ZERO, self.screen.mouse_pos()), Rgba8::WHITE);

        self.screen.draw_to_window(draw, Rgba8::BLACK);
        Ok(())
    }
}
