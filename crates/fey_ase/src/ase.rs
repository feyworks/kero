use crate::{
    BlendMode, Cel, CelType, Error, Format, Frame, Layer, LayerType, LoopDir, NineSliceKey, Slice,
    SliceKey, SliceType, Tag, UserData,
};
use byteorder::{LE, ReadBytesExt};
use fey_color::Rgba8;
use fey_math::{Numeric, Vec2};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use zune_inflate::{DeflateDecoder, DeflateOptions};

/// A parsed Aseprite file.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ase {
    /// Size of the sprite.
    pub size: Vec2<u16>,

    /// Format of the sprite.
    pub format: Format,

    /// Layers of the sprite.
    pub layers: Vec<Layer>,

    /// Frames of the sprite.
    pub frames: Vec<Frame>,

    /// Tags of the sprite.
    pub tags: Vec<Tag>,

    /// Slices of the sprite.
    pub slices: Vec<Slice>,

    /// Palette of the sprite.
    pub palette: Vec<Rgba8>,

    /// The sprite's user data.
    pub user_data: Option<UserData>,
}

impl Ase {
    /// Parse an Aseprite file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::parse(BufReader::new(File::open(path)?))
    }

    /// Parse an Aseprite file.
    pub fn parse<R: Read + Seek>(mut r: R) -> Result<Self, Error> {
        let _file_size = read_dword(&mut r)?;

        // parse the magic number
        let magic = read_word(&mut r)?;
        if magic != 0xA5E0 {
            return Err(Error::InvalidMagicNumber(magic));
        }

        // parse image info
        let num_frames = read_word(&mut r)? as usize;
        let size = Vec2::new(read_word(&mut r)?, read_word(&mut r)?);
        let mut format = match read_word(&mut r)? {
            32 => Format::Rgba,
            16 => Format::Grayscale,
            8 => Format::Indexed {
                transparent_index: 0,
            },
            depth => return Err(Error::InvalidColorDepth(depth)),
        };
        let flags = read_dword(&mut r)?;
        let opacity_valid = (flags & 1) != 0;
        let _ = read_word(&mut r)?;
        let _ = read_dword(&mut r)?;
        let _ = read_dword(&mut r)?;
        if let Format::Indexed { transparent_index } = &mut format {
            *transparent_index = r.read_u8()?;
        } else {
            skip(&mut r, 1)?;
        }

        skip(&mut r, 3)?;
        let _num_colors = read_word(&mut r).map(|n| if n > 0 { n } else { 256 })? as usize;
        let _pixel_size = Vec2::new(r.read_u8()? as usize, r.read_u8()? as usize);
        let _grid_pos = Vec2::new(read_short(&mut r)?, read_short(&mut r)?);
        let _grid_size = Vec2::new(read_word(&mut r)? as usize, read_word(&mut r)? as usize);
        skip(&mut r, 84)?;

        // track where the next parsed user-data should land
        let mut target = UserDataTarget::Sprite;
        let mut target_tag = 0;
        let mut user_data = None;

        // used for decompression
        let mut compressed = Vec::new();

        // animation info
        let mut palette = Vec::new();
        let mut old_palette = [Rgba8::WHITE; 256];
        let mut tags = Vec::new();
        let mut layers = Vec::new();
        let mut slices: Vec<Slice> = Vec::new();
        let mut frames = Vec::new();

        // parse frames
        for _ in 0..num_frames {
            let _num_bytes = read_dword(&mut r)? as usize;

            // parse the magic number
            let magic = read_word(&mut r)?;
            if magic != 0xF1FA {
                return Err(Error::InvalidFrameMagicNumber(magic));
            }

            // parse the chunk count and frame duration
            let old_chunk_num = read_word(&mut r)? as usize;
            let duration = read_word(&mut r)?;
            skip(&mut r, 2)?;
            let new_chunk_num = read_dword(&mut r)? as usize;
            let chunk_num = (new_chunk_num > 0)
                .then_some(new_chunk_num)
                .unwrap_or(old_chunk_num);

            let mut cels: Vec<Cel> = Vec::new();

            // parse the chunks
            for _ in 0..chunk_num {
                let chunk_start = r.stream_position().unwrap();
                let chunk_size = read_dword(&mut r)?;
                let chunk_end = chunk_start + (chunk_size as u64);

                let chunk_ty = read_word(&mut r)?;
                let chunk_ty = ChunkType::from_repr(chunk_ty)
                    .ok_or_else(|| Error::InvalidChunkType(chunk_ty))?;

                match chunk_ty {
                    ChunkType::Palette => {
                        target = UserDataTarget::Sprite;
                        let new_size = read_dword(&mut r)? as usize;
                        if new_size > palette.len() {
                            palette.resize(new_size, Rgba8::BLACK);
                        }
                        let first_i = read_dword(&mut r)? as usize;
                        let last_i = read_dword(&mut r)? as usize;
                        skip(&mut r, 8)?;
                        for i in first_i..=last_i {
                            let flags = read_word(&mut r)?;
                            palette[i] =
                                Rgba8::new(r.read_u8()?, r.read_u8()?, r.read_u8()?, r.read_u8()?);
                            if (flags & 0x1) != 0 {
                                let _name = read_string(&mut r)?;
                            }
                        }
                    }
                    ChunkType::OldPalette1 | ChunkType::OldPalette2 => {
                        if palette.is_empty() {
                            let num_packets = read_word(&mut r)? as usize;
                            let mut i = 0;
                            for _ in 0..num_packets {
                                i += r.read_u8()? as usize;
                                let mut num_colors = r.read_u8()? as usize;
                                if num_colors == 0 {
                                    num_colors = 256;
                                }
                                for _ in 0..num_colors {
                                    old_palette[i] =
                                        Rgba8::new(r.read_u8()?, r.read_u8()?, r.read_u8()?, 255);

                                    // TODO: should `i` be incremented here?
                                    i += 1;
                                }
                            }
                        }
                    }
                    ChunkType::Tags => {
                        target = UserDataTarget::Tag;
                        let num_tags = read_word(&mut r)?;
                        skip(&mut r, 8)?;
                        for _ in 0..num_tags {
                            let from = read_word(&mut r)?;
                            let to = read_word(&mut r)?;
                            let loop_dir = r.read_u8()?;
                            let loop_dir = LoopDir::from_repr(loop_dir)
                                .ok_or_else(|| Error::InvalidLoopDir(loop_dir))?;
                            let repeat = read_word(&mut r)?;
                            let repeat = (repeat != 0).then_some(repeat);
                            skip(&mut r, 10)?;
                            let name = read_string(&mut r)?;
                            tags.push(Tag {
                                name,
                                from,
                                to,
                                loop_dir,
                                repeat,
                                user_data: None,
                            });
                        }
                    }
                    ChunkType::Layer => {
                        target = UserDataTarget::Layer;

                        let flags = read_word(&mut r)?;
                        let ty = match read_word(&mut r)? {
                            0 => LayerType::Normal,
                            1 => LayerType::Group,
                            2 => unimplemented!("tilemaps not supported"),
                            ty => return Err(Error::InvalidLayerType(ty)),
                        };

                        //assert_ne!(ty, LayerType::Group, "groups not currently handled");

                        let level = read_word(&mut r)?;
                        let _default_width = read_word(&mut r)?;
                        let _default_height = read_word(&mut r)?;
                        let blend_mode = read_word(&mut r)?;
                        let blend_mode = BlendMode::from_repr(blend_mode)
                            .ok_or_else(|| Error::InvalidBlendMode(blend_mode))?;
                        let mut opacity = r.read_u8()?;
                        if !opacity_valid {
                            opacity = 255;
                        }
                        skip(&mut r, 3)?;
                        let name = read_string(&mut r)?;
                        /*if let LayerType::Tilemap { tileset_index } = &mut ty {
                            *tileset_index = read_dword(&mut r)? as usize;
                        }*/

                        layers.push(Layer {
                            flags,
                            level,
                            blend_mode,
                            opacity,
                            name,
                            user_data: None,
                            group: ty == LayerType::Group,
                        });
                    }
                    ChunkType::UserData => {
                        let flags = read_dword(&mut r)?;
                        let text = if (flags & 1) != 0 {
                            Some(read_string(&mut r)?)
                        } else {
                            None
                        };
                        let color = if (flags & 2) != 0 {
                            Some(Rgba8::new(
                                r.read_u8()?,
                                r.read_u8()?,
                                r.read_u8()?,
                                r.read_u8()?,
                            ))
                        } else {
                            None
                        };
                        if (flags & 4) != 0 {
                            unimplemented!("user-data properties not yet supported");
                        }
                        let data = Some(UserData { text, color });
                        match target {
                            UserDataTarget::Sprite => user_data = data,
                            UserDataTarget::Layer => {
                                layers.last_mut().unwrap().user_data = data;
                            }
                            UserDataTarget::Cel => cels.last_mut().unwrap().user_data = data,
                            UserDataTarget::Tag => {
                                tags[target_tag].user_data = data;
                                target_tag += 1;
                            }
                            UserDataTarget::Slice => {
                                slices.last_mut().unwrap().user_data = data;
                            }
                        }
                    }
                    ChunkType::Cel => {
                        target = UserDataTarget::Cel;

                        let layer_index = read_word(&mut r)? as usize;
                        let pos = Vec2::new(read_short(&mut r)?, read_short(&mut r)?);
                        let opacity = r.read_u8()?;
                        let ty = read_word(&mut r)?;
                        let z_index = read_short(&mut r)?;
                        skip(&mut r, 5)?;
                        let ty = match ty {
                            // linked cel
                            1 => {
                                let frame_index = read_word(&mut r)?;
                                CelType::Link { frame_index }
                            }

                            // compressed image
                            2 => {
                                let size = Vec2::new(read_word(&mut r)?, read_word(&mut r)?);
                                let data = read_image(
                                    &mut r,
                                    size.to_usize(),
                                    format,
                                    &mut compressed,
                                    chunk_end,
                                )?;
                                CelType::Image { size, data }
                            }

                            // compressed tilemap
                            3 => {
                                unimplemented!("tilemaps not supported");
                            }

                            ty => return Err(Error::InvalidCelType(ty)),
                        };
                        cels.push(Cel {
                            layer_index,
                            pos,
                            opacity,
                            z_index,
                            ty,
                            user_data: None,
                        });
                    }
                    ChunkType::Slice => {
                        target = UserDataTarget::Slice;

                        let num_keys = read_dword(&mut r)?;
                        let flags = read_dword(&mut r)?;
                        let _ = read_dword(&mut r)?;
                        let name = read_string(&mut r)?;

                        let ty = if (flags & 1) != 0 {
                            let mut keys = Vec::new();
                            for _ in 0..num_keys {
                                let frame = read_dword(&mut r)?;
                                let origin = Vec2::new(read_long(&mut r)?, read_long(&mut r)?);
                                let size = Vec2::new(read_dword(&mut r)?, read_dword(&mut r)?);
                                let center_pos = Vec2::new(read_long(&mut r)?, read_long(&mut r)?);
                                let center_size =
                                    Vec2::new(read_dword(&mut r)?, read_dword(&mut r)?);
                                let pivot = if (flags & 2) != 0 {
                                    Vec2::new(read_long(&mut r)?, read_long(&mut r)?)
                                } else {
                                    Vec2::ZERO
                                };
                                keys.push(NineSliceKey {
                                    key: SliceKey {
                                        frame,
                                        origin,
                                        size,
                                        pivot,
                                    },
                                    center_pos,
                                    center_size,
                                });
                            }
                            SliceType::Nine(keys)
                        } else {
                            let mut keys = Vec::new();
                            for _ in 0..num_keys {
                                let frame = read_dword(&mut r)?;
                                let origin = Vec2::new(read_long(&mut r)?, read_long(&mut r)?);
                                let size = Vec2::new(read_dword(&mut r)?, read_dword(&mut r)?);
                                let pivot = if (flags & 2) != 0 {
                                    Vec2::new(read_long(&mut r)?, read_long(&mut r)?)
                                } else {
                                    Vec2::ZERO
                                };
                                keys.push(SliceKey {
                                    frame,
                                    origin,
                                    size,
                                    pivot,
                                });
                            }
                            SliceType::Rect(keys)
                        };
                        slices.push(Slice {
                            name,
                            ty,
                            user_data: None,
                        });
                    }
                    ChunkType::Tileset => {
                        unimplemented!("tilemaps not supported");
                    }
                    _ => {}
                }

                r.seek(SeekFrom::Start(chunk_end))?;
            }

            frames.push(Frame { duration, cels });
        }

        Ok(Self {
            size,
            format,
            layers,
            frames,
            tags,
            slices,
            palette,
            user_data,
        })
    }
}

#[inline]
fn skip<R: Read + Seek>(read: &mut R, n: usize) -> Result<(), Error> {
    read.seek(SeekFrom::Current(n as i64))?;
    Ok(())
}

#[inline]
fn read_word<R: Read>(read: &mut R) -> Result<u16, Error> {
    Ok(read.read_u16::<LE>()?)
}

#[inline]
fn read_short<R: Read>(read: &mut R) -> Result<i16, Error> {
    Ok(read.read_i16::<LE>()?)
}

#[inline]
fn read_dword<R: Read>(read: &mut R) -> Result<u32, Error> {
    Ok(read.read_u32::<LE>()?)
}

#[inline]
fn read_long<R: Read>(read: &mut R) -> Result<i32, Error> {
    Ok(read.read_i32::<LE>()?)
}

#[inline]
fn read_string<R: Read>(read: &mut R) -> Result<String, Error> {
    let len = read_word(read)? as usize;
    let mut utf8 = Vec::new();
    utf8.resize(len, 0);
    read.read_exact(utf8.as_mut_slice())?;
    Ok(String::from_utf8(utf8)?)
}

fn read_image<R: Read + Seek>(
    r: &mut R,
    size: Vec2<usize>,
    format: Format,
    mut buf: &mut Vec<u8>,
    end: u64,
) -> Result<Vec<u8>, Error> {
    let compressed_len = (end - r.stream_position().unwrap()) as usize;
    buf.resize(compressed_len, 0);
    r.read_exact(&mut buf)?;

    let decompressed_len = size.x * size.y * format.bytes_per_pixel();
    let mut decoder = DeflateDecoder::new_with_options(
        &buf,
        DeflateOptions::default().set_size_hint(decompressed_len),
    );

    let decompressed = decoder.decode_zlib()?;
    assert_eq!(decompressed.len(), decompressed_len);

    Ok(decompressed)
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, strum::FromRepr)]
enum ChunkType {
    OldPalette1 = 0x0004,
    OldPalette2 = 0x0011,
    Layer = 0x2004,
    Cel = 0x2005,
    CelExtra = 0x2006,
    ColorProfile = 0x2007,
    ExternalFiles = 0x2008,
    Mask = 0x2016,
    Path = 0x2017,
    Tags = 0x2018,
    Palette = 0x2019,
    UserData = 0x2020,
    Slice = 0x2022,
    Tileset = 0x2023,
}

enum UserDataTarget {
    Sprite,
    Tag,
    Layer,
    Cel,
    Slice,
}
