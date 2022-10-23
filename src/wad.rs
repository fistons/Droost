use std::array::TryFromSliceError;
use std::borrow::Cow;
use std::{fs, io, str};

const LUMP_HEADER_SIZE: usize = 16;

#[derive(thiserror::Error, Debug)]
pub enum WadError {
    #[error("Unreadable file")]
    UnreadableFile(#[from] io::Error),
    #[error("Slice with incorrect length {0}")]
    SliceWithIncorrectLenght(#[from] TryFromSliceError),
    #[error("Unknown WAD type {0}")]
    UnkownWadType(String),
}

#[derive(Debug)]
pub struct LumpHeader {
    offset: i32,
    size: i32,
    name: String,
}

impl TryFrom<&[u8; 16]> for LumpHeader {
    type Error = WadError;

    fn try_from(value: &[u8; 16]) -> Result<Self, Self::Error> {
        let name = String::from_utf8_lossy(&value[8..16]).into_owned();
        let data_position = i32::from_le_bytes(value[..4].try_into()?);
        let size = i32::from_le_bytes(value[4..8].try_into()?);

        let lump_header = LumpHeader {
            offset: data_position,
            size,
            name,
        };
        Ok(lump_header)
    }
}

#[derive(Debug)]
pub enum LumpType {
    THINGS,
}

#[derive(Debug)]
pub struct WadHeader {
    wad_type: WadType,
    lumps_number: i32,
    directory_offset: i32,
}

#[derive(Debug)]
pub struct Thing {
    x: i32,
    y: i32,
    angle: i32,
    thing_type: i32,
    flags: i32,
}

impl TryFrom<&[u8; 10]> for Thing {
    type Error = WadError;

    fn try_from(value: &[u8; 10]) -> Result<Self, Self::Error> {
        let x = i32::from_le_bytes(value[0..2].try_into()?);
        let y = i32::from_le_bytes(value[2..4].try_into()?);
        let angle = i32::from_le_bytes(value[4..6].try_into()?);
        let thing_type = i32::from_le_bytes(value[6..8].try_into()?);
        let flags = i32::from_le_bytes(value[8..10].try_into()?);

        Ok(Thing {
            x,
            y,
            angle,
            thing_type,
            flags,
        })
    }
}

#[derive(Debug)]
pub enum WadType {
    IWAD,
    PWAD,
}

impl TryFrom<Cow<'_, str>> for WadType {
    type Error = WadError;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        match value {
            Cow::Borrowed("IWAD") => Ok(WadType::IWAD),
            Cow::Borrowed("PWAD") => Ok(WadType::PWAD),
            unkown_type => Err(WadError::UnkownWadType(String::from(unkown_type))),
        }
    }
}

pub fn parse(wad_path: &str) -> Result<WadHeader, WadError> {
    let source = fs::read(wad_path)?;
    let wad_type = String::from_utf8_lossy(&source[0..4]);

    let lumps_number: i32 = i32::from_le_bytes(source[4..8].try_into()?);
    let directory_offset: i32 = i32::from_le_bytes(source[8..12].try_into()?);
    let wad_type = wad_type.try_into()?;

    for x in (0..source.len() - directory_offset as usize).step_by(LUMP_HEADER_SIZE) {
        let offset = directory_offset as usize + x;
        let slice: &[u8; LUMP_HEADER_SIZE] =
            &source[offset..offset + LUMP_HEADER_SIZE].try_into()?;
        let lump: LumpHeader = slice.try_into()?;
        println!("{:?}", lump);
    }

    let wad = WadHeader {
        wad_type,
        lumps_number,
        directory_offset,
    };
    println!("{:?}", wad);
    Ok(wad)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_read_wad() {
        assert!(matches!(
            parse("/home/emercier/Downloads/DOOM2.WAD"),
            Ok(WadHeader {
                wad_type: WadType::IWAD,
                lumps_number: 2919,
                directory_offset: 14557880
            })
        ));
    }

    #[test]
    pub fn test_read_invalid_wad() {
        assert!(matches!(
            parse("/home/emercier/Downloads/url.png"),
            Err(WadError::UnkownWadType(_))
        ));
    }
}
