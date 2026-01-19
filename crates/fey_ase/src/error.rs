use std::string::FromUtf8Error;
use thiserror::Error;
use zune_inflate::errors::InflateDecodeErrors;

/// An Aseprite parsing error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("invalid color depth: {0}")]
    InvalidColorDepth(u16),

    #[error("root magic number mismatch (expected [0xA5E0], got [{0:#06X}]")]
    InvalidMagicNumber(u16),

    #[error("frame magic number mismatch (expected [0xF1FA], got [{0:#06X}]")]
    InvalidFrameMagicNumber(u16),

    #[error("{0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("unknown chunk type: [{0:#06X}]")]
    InvalidChunkType(u16),

    #[error("invalid loop direction value: {0}")]
    InvalidLoopDir(u8),

    #[error("invalid layer type: {0}")]
    InvalidLayerType(u16),

    #[error("invalid blend mode: {0}")]
    InvalidBlendMode(u16),

    #[error("invalid cel type: {0}")]
    InvalidCelType(u16),

    #[error("{0}")]
    Inflate(#[from] InflateDecodeErrors),
}
