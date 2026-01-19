use crate::{BlendMode, UserData};

/// A sprite's layer info.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Layer {
    /// Layer flags.
    pub flags: u16,

    /// The grouped depth of a layer.
    pub level: u16,

    /// The layer's blend mode.
    pub blend_mode: BlendMode,

    /// The layer's opacity.
    pub opacity: u8,

    /// The layer's name.
    pub name: String,

    /// The layer's user-data.
    pub user_data: Option<UserData>,

    /// If the layer is a folder.
    pub group: bool,
}

impl Layer {
    /// If the layer was toggled as visible.
    pub const fn visible(&self) -> bool {
        (self.flags & 1) != 0
    }

    /// If the layer was marked as a background layer.
    pub const fn background(&self) -> bool {
        (self.flags & 8) != 0
    }

    /// If the layer was marked as a reference layer.
    pub const fn reference(&self) -> bool {
        (self.flags & 64) != 0
    }
}

/// A layer type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LayerType {
    /// A normal layer that can have cels in it.
    Normal,

    /// A group layer is a folder that can contain other layers.
    Group,
}
