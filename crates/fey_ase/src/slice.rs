use crate::UserData;
use fey_math::Vec2;

/// A sprite slice.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Slice {
    /// The slice's name.
    pub name: String,

    /// The slice's type.
    pub ty: SliceType,

    /// The slice's user-data.
    pub user_data: Option<UserData>,
}

/// Type of a sprite slice.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SliceType {
    /// The slice is a rectangle with a pivot point.
    Rect(Vec<SliceKey>),

    /// The slice is a [`Rect`](SliceType::Rect) slice, but with
    /// an additional center-region forming 9 rectangles total.
    Nine(Vec<NineSliceKey>),
}

/// A slice keyframe.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SliceKey {
    /// The starting frame of the slice.
    pub frame: u32,

    /// The slice's origin.
    pub origin: Vec2<i32>,

    /// The slice's size.
    pub size: Vec2<u32>,

    /// The slice's pivot point.
    pub pivot: Vec2<i32>,
}

/// A slice keyframe with a center-region.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NineSliceKey {
    /// The slice's keyframe.
    pub key: SliceKey,

    /// Position of the 9-slice's center rect.
    pub center_pos: Vec2<i32>,

    /// Size of the 9-slice's center rect.
    pub center_size: Vec2<u32>,
}
