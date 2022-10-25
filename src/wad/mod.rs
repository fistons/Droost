pub mod header;
pub mod lump;
pub mod patch;
pub mod thing;

use self::header::WadHeader;
use self::lump::LumpHeader;
use std::fs;

const LUMP_HEADER_SIZE: usize = 16;

#[derive(thiserror::Error, Debug)]
pub enum WadError {
    #[error("Unreadable file")]
    UnreadableFile(#[from] std::io::Error),
    #[error("Slice with incorrect length {0}")]
    SliceWithIncorrectLenght(#[from] std::array::TryFromSliceError),
    #[error("Unknown WAD type {0}")]
    UnkownWadType(String),
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

    use super::header::WadType;
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
