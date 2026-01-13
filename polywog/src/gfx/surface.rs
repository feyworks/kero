use crate::gfx::{Graphics, Texture};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[cfg(feature = "lua")]
pub type SurfaceObj = fey_lua::UserDataOf<Surface>;
#[cfg(feature = "lua")]
pub type SurfaceRef = mlua::UserDataRef<Surface>;

/// Handle to a surface that can be drawn to.
///
/// This handle can be cloned and passed around freely to give objects access to the surface.
///
/// Surfaces are created from [`Graphics`](super::Graphics).
#[derive(Clone)]
pub struct Surface(Rc<Inner>);

struct Inner {
    texture: Texture,

    #[cfg(feature = "lua")]
    texture_userdata: mlua::AnyUserData,
}

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Surface").finish_non_exhaustive()
    }
}

impl PartialEq for Surface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.texture == other.0.texture
    }
}

impl PartialOrd for Surface {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.texture.partial_cmp(&other.0.texture)
    }
}

impl Surface {
    pub(crate) fn new(gfx: &Graphics, texture: Texture) -> Self {
        Self(Rc::new(Inner {
            #[cfg(feature = "lua")]
            texture_userdata: gfx
                .lua()
                .upgrade()
                .create_userdata(texture.clone())
                .unwrap(),

            texture,
        }))
    }

    /// The surface's texture.
    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.0.texture
    }

    #[cfg(feature = "lua")]
    pub fn texture_userdata(&self) -> &mlua::AnyUserData {
        &self.0.texture_userdata
    }
}

impl Deref for Surface {
    type Target = Texture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.texture
    }
}

impl AsRef<Texture> for Surface {
    #[inline]
    fn as_ref(&self) -> &Texture {
        &self.0.texture
    }
}

impl Borrow<Texture> for Surface {
    #[inline]
    fn borrow(&self) -> &Texture {
        &self.0.texture
    }
}
