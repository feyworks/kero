/// How an animation should loop.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, strum::FromRepr)]
pub enum LoopDir {
    /// Play forward, looping around to the beginning.
    Forward = 0,

    /// Starting at the end, play in reverse, looping around.
    Reverse = 1,

    /// Instead of looping, reverse animation direction at the end.
    PingPong = 2,

    /// The same as [`PingPong`](LoopDir::PingPong), but starting at the end.
    PingPongReverse = 3,
}
