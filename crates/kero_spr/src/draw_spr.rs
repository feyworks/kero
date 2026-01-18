use crate::{SpriteFont, SpritePatch};
use kero::prelude::*;

/// Extension for [`Draw`](kero::gfx::Draw) with sprite rendering methods.
pub trait DrawSpr {
    /// Draw a sprite at the provided position.
    fn sprite_at_flipped(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    );

    /// Draw a sprite at the provided position.
    fn sprite_at_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    );

    /// Draw a sprite at the provided position.
    #[inline]
    fn sprite_at(&mut self, sub: impl AsRef<SubTexture>, pos: impl Into<Vec2F>) {
        self.sprite_at_ext(sub, pos, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw text using a sprite font.
    fn sprite_text(&mut self, text: &str, pos: Vec2F, font: &SpriteFont, color: Rgba8);

    /// Draw a sprite patch filling a rectangle.
    fn patch_ext(&mut self, patch: &SpritePatch, rect: RectF, color: Rgba8, mode: ColorMode);

    /// Draw a sprite patch filling a rectangle.
    fn patch(&mut self, patch: &SpritePatch, rect: RectF) {
        self.patch_ext(patch, rect, Rgba8::WHITE, ColorMode::MULT);
    }
}

impl DrawSpr for Draw {
    #[inline]
    fn sprite_at_flipped(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        self.subtexture_at_flipped(sub, pos, color, mode, flip);
    }

    #[inline]
    fn sprite_at_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        self.subtexture_at_ext(sub, pos, color, mode);
    }

    fn sprite_text(&mut self, text: &str, mut pos: Vec2F, font: &SpriteFont, color: Rgba8) {
        let left = pos.x;
        for chr in text.chars() {
            if chr == '\n' {
                pos.x = left;
                pos.y += font.line_height();
            } else if let Some(g) = font.glyph(chr).or_else(|| font.glyph('\0')) {
                if let Some(sub) = g.sub() {
                    self.subtexture_at_ext(sub, pos, color, ColorMode::MULT);
                }
                pos.x += g.advance();
            } else {
                println!("no glyph for: [{}]", chr);
            }
        }
    }

    fn patch_ext(&mut self, patch: &SpritePatch, rect: RectF, color: Rgba8, mode: ColorMode) {
        let px = [
            rect.x,
            rect.x + patch.0.left_w,
            rect.right() - patch.0.right_w,
            rect.right(),
        ];
        let py = [
            rect.y,
            rect.y + patch.0.top_h,
            rect.bottom() - patch.0.bottom_h,
            rect.bottom(),
        ];
        let vert = |i, j| {
            Vertex::new(
                vec2(px[i], py[j]),
                vec2(patch.0.tx[i], patch.0.ty[j]),
                color,
                mode,
            )
        };
        self.custom(
            Some(patch.texture().clone()),
            Topology::Triangles,
            [
                vert(0, 0),
                vert(1, 0),
                vert(2, 0),
                vert(3, 0),
                vert(0, 1),
                vert(1, 1),
                vert(2, 1),
                vert(3, 1),
                vert(0, 2),
                vert(1, 2),
                vert(2, 2),
                vert(3, 2),
                vert(0, 3),
                vert(1, 3),
                vert(2, 3),
                vert(3, 3),
            ],
            [
                0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 4, 5, 9, 4, 9, 8, 5, 6, 10,
                5, 10, 9, 6, 7, 11, 6, 11, 10, 8, 9, 13, 8, 13, 12, 9, 10, 14, 9, 14, 13, 10, 11,
                15, 10, 15, 14,
            ],
        );
    }
}
