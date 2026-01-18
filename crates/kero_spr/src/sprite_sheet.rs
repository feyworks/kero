use crate::Sprite;
use kero::grid::{Coord, Grid, VecGrid};
use kero::math::Vec2F;
use std::rc::Rc;

/// A sheet of sprite tiles.
#[derive(Debug, Clone)]
pub struct SpriteSheet(Rc<Inner>);

#[derive(Debug)]
struct Inner {
    tile_size: Vec2F,
    tiles: VecGrid<Option<Sprite>>,
}

impl SpriteSheet {
    pub fn new(tile_size: Vec2F, tiles: VecGrid<Option<Sprite>>) -> Self {
        Self(Rc::new(Inner { tile_size, tiles }))
    }

    #[inline]
    pub fn tile_size(&self) -> Vec2F {
        self.0.tile_size
    }

    #[inline]
    pub fn tile(&self, x: u32, y: u32) -> Option<&Sprite> {
        self.0.tiles.get(x, y).and_then(|t| t.as_ref())
    }

    #[inline]
    pub fn tile_at(&self, pos: impl Coord) -> Option<&Sprite> {
        self.0.tiles.get_at(pos).and_then(|t| t.as_ref())
    }
}
