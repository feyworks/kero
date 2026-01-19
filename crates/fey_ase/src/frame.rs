use crate::Cel;

/// An animation frame.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Frame {
    /// Duration of the frame.
    pub duration: u16,

    /// All the cels on this frame.
    pub cels: Vec<Cel>,
}
