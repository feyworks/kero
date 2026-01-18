use kero::gfx::Texture;
use kero::math::{Numeric, RectF};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SpritePatch(pub(crate) Rc<Inner>);

#[derive(Debug)]
pub(crate) struct Inner {
    pub texture: Texture,
    pub left_w: f32,
    pub right_w: f32,
    pub top_h: f32,
    pub bottom_h: f32,
    pub tx: [f32; 4],
    pub ty: [f32; 4],
}

impl SpritePatch {
    pub fn new(texture: Texture, outer: RectF, inner: RectF) -> Self {
        let x = [outer.x, inner.x, inner.right(), outer.right()];
        let y = [outer.y, inner.y, inner.bottom(), outer.bottom()];

        let left_w = x[1] - x[0];
        let right_w = x[3] - x[2];
        let top_h = y[1] - y[0];
        let bottom_h = y[3] - y[2];

        let size = texture.size().to_f32();
        let tx = x.map(|x| x / size.x);
        let ty = y.map(|y| y / size.y);

        Self(Rc::new(Inner {
            texture,
            left_w,
            right_w,
            top_h,
            bottom_h,
            tx,
            ty,
        }))
    }

    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.0.texture
    }
}
