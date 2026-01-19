use fey_color::Rgba8;

/// User-defined data.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UserData {
    /// The user-data's text.
    pub text: Option<String>,

    /// The user-data's color.
    pub color: Option<Rgba8>,
}
