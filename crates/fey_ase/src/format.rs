/// Aseprite image format.
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Format {
    /// 32-bit RGBA pixel data (bytes of image data are `(R, G, B, A)` quadruplets).
    Rgba,

    /// 16-bit greyscale-alpha pixel data (bytes of image data are value-alpha pairs).
    Grayscale,

    /// Palletized image (bytes of image data are indices into the palette).
    Indexed {
        /// Which index of the palette should be treated as transparent pixels.
        transparent_index: u8,
    },
}

impl Format {
    /// How many bytes are in each pixel of the image data of this format.
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Rgba => 4,
            Self::Grayscale => 2,
            Self::Indexed { .. } => 1,
        }
    }
}
