use crate::UserData;
use fey_math::Vec2;
use std::fmt::{Debug, Formatter};

/// The pixels of a specific frame & layer.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cel {
    /// Which layer this cel is on.
    pub layer_index: usize,

    /// The cel's position in the sprite.
    pub pos: Vec2<i16>,

    /// The cel's opacity.
    pub opacity: u8,

    /// The cel's depth-ordering.
    pub z_index: i16,

    /// The cel's type (image or linked).
    pub ty: CelType,

    /// The cel's user-data.
    pub user_data: Option<UserData>,
}

/// Type of a cel.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum CelType {
    /// Cel duplicates the pixel data of another cel.
    Link {
        /// Frame of the duplicate cel.
        frame_index: u16,
    },
    /// Cel has image data.
    Image {
        /// Size of the cel's bitmap data.
        size: Vec2<u16>,

        /// The cel's bitmap data.
        data: Vec<u8>,
    },
}

impl Debug for CelType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Link { frame_index } => write!(f, "Link({})", frame_index),
            Self::Image { size, .. } => write!(f, "Image({} x {})", size.x, size.y),
        }
    }
}
