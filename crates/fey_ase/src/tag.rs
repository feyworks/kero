use crate::{LoopDir, UserData};

/// An animation tag.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Tag {
    /// Name of the tag.
    pub name: String,

    /// First frame of the animation.
    pub from: u16,

    /// Last frame of the animation.
    pub to: u16,

    /// How the animation should loop.
    pub loop_dir: LoopDir,

    /// How many times the animation should repeat.
    ///
    /// For the [`PingPong`](LoopDir::PingPong) variations, a *repeat*
    /// means in one direction. So a value of `2` would mean it plays once
    /// to the end, and then once back in reverse.
    pub repeat: Option<u16>,

    /// The animation's user-data.
    pub user_data: Option<UserData>,
}
