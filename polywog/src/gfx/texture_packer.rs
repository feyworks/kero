use crate::gfx::{Graphics, SubTexture, Texture};
use crate::grid::{Grid, GridMut};
use crate::math::{Numeric, RectF, RectU, Vec2F, Vec2U};
use crate::prelude::TexturePixel;
use fey_color::{Grey8, GreyAlpha8, Rgb8, Rgba8, ToRgba};
use fey_img::Image;
use fey_packer::{Item, Packed, RectPacker};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

pub type TexturePackerRgba8<'a, K> = TexturePacker<'a, K, Rgba8>;
pub type TexturePackerRgb8<'a, K> = TexturePacker<'a, K, Rgb8>;
pub type TexturePackerGreyAlpha8<'a, K> = TexturePacker<'a, K, GreyAlpha8>;
pub type TexturePackerGrey8<'a, K> = TexturePacker<'a, K, Grey8>;

pub struct TexturePacker<'a, K, P: TexturePixel> {
    to_pack: Vec<ToPack<'a, K, P>>,
}

struct ToPack<'a, K, P: TexturePixel> {
    key: K,
    img: PackImage<'a, P>,
    src_rect: RectU,
    trim_rect: RectU,
}

impl<'a, K: Clone + Eq + Hash, P: TexturePixel> TexturePacker<'a, K, P> {
    pub fn new() -> Self {
        Self {
            to_pack: Vec::new(),
        }
    }

    pub fn add_image(
        &mut self,
        key: K,
        img: impl Into<PackImage<'a, P>>,
        src_rect: impl Into<Option<RectU>>,
        trim_threshold: impl Into<Option<P::Channel>>,
    ) {
        let img = img.into();
        let src_rect = src_rect.into().unwrap_or(RectU::sized(img.size()));
        let src = img.view_at(src_rect);
        let trim_rect = trim_threshold
            .into()
            .and_then(|a| src.get_bounds(|p| p.alpha() > a))
            .unwrap_or_else(|| RectU::sized(src_rect.size()));
        self.to_pack.push(ToPack {
            key,
            img,
            src_rect,
            trim_rect,
        });
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.to_pack.len()
    }

    pub fn pack(self, gfx: &Graphics) -> Option<(Texture, HashMap<K, SubTexture>)> {
        self.pack_ext(gfx, gfx.max_texture_size(), 1, 2)
    }

    pub fn pack_ext(
        self,
        gfx: &Graphics,
        max_size: u32,
        spacing: u32,
        padding: u32,
    ) -> Option<(Texture, HashMap<K, SubTexture>)> {
        let padding = Vec2U::splat(padding);

        let items: Vec<Item<usize>> = self
            .to_pack
            .iter()
            .enumerate()
            .map(|(i, item)| Item::new(item.trim_rect.size() + padding, i))
            .collect();

        let (size, mut packed) = RectPacker::new()
            .with_max_size(max_size)
            .with_spacing(spacing)
            .pack(items)?;
        packed.sort_by_key(|i| i.data);

        let mut tex_img = Image::<P, _>::new_vec(size, P::default());

        let padding = padding.to_f32();
        let sub_info: Vec<(K, RectF, Vec2F, Vec2F)> = packed
            .into_iter()
            .map(|Packed { data: i, pos }| {
                let ToPack {
                    key,
                    img,
                    src_rect,
                    trim_rect,
                } = &self.to_pack[i];
                let src = img.view_at(*trim_rect + src_rect.top_left());

                let dst_rect = RectU::pos_size(pos, trim_rect.size());
                let mut dst = tex_img.view_mut_at(dst_rect);
                dst.draw_copied(&src);
                (
                    key.clone(),
                    dst_rect.to_f32().inflate(padding),
                    trim_rect.top_left().to_f32() - padding,
                    src_rect.size().to_f32(),
                )
            })
            .collect();

        let tex_img = tex_img.to_rgba8();
        let tex = gfx.create_texture_from_img(&tex_img);
        let subs = sub_info
            .into_iter()
            .map(|(key, rect, offset, size)| {
                (key, SubTexture::new_ext(tex.clone(), rect, offset, size))
            })
            .collect();

        Some((tex, subs))
    }
}

#[derive(Debug, Clone)]
pub enum PackImage<'a, P: TexturePixel> {
    Ref(&'a Image<P>),
    Owned(Image<P>),
    Shared(Rc<Image<P>>),
}

impl<P: TexturePixel> Deref for PackImage<'_, P> {
    type Target = Image<P>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => *r,
            Self::Owned(r) => r,
            Self::Shared(r) => &r,
        }
    }
}

impl<P: TexturePixel> From<Image<P>> for PackImage<'_, P> {
    #[inline]
    fn from(value: Image<P>) -> Self {
        Self::Owned(value)
    }
}

impl<'a, P: TexturePixel> From<&'a Image<P>> for PackImage<'a, P> {
    #[inline]
    fn from(value: &'a Image<P>) -> Self {
        Self::Ref(value)
    }
}

impl<'a, P: TexturePixel> From<&'a mut Image<P>> for PackImage<'a, P> {
    #[inline]
    fn from(value: &'a mut Image<P>) -> Self {
        Self::Ref(value)
    }
}

impl<P: TexturePixel> From<Rc<Image<P>>> for PackImage<'_, P> {
    #[inline]
    fn from(value: Rc<Image<P>>) -> Self {
        Self::Shared(value)
    }
}

impl<P: TexturePixel> From<&Rc<Image<P>>> for PackImage<'_, P> {
    #[inline]
    fn from(value: &Rc<Image<P>>) -> Self {
        Self::Shared(value.clone())
    }
}
