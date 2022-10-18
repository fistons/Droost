use std::borrow::Cow;
use std::{fs, io, str};

#[derive(Debug)]
pub struct Wad {
    source: Vec<u8>,
    wad_type: WadType,
    lumps_number: i32,
}

#[derive(Debug)]
pub enum WadType {
    IWAD,
    PWAD,
}

#[derive(thiserror::Error, Debug)]
pub enum WadError {
    #[error("Unreadable file")]
    UnreadableFile(#[from] io::Error),
    #[error("Unknown WAD type {0}")]
    UnkownWadType(String),
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

pub fn parse(wad_path: &str) -> Result<Wad, WadError> {
    let source = fs::read(wad_path)?;
    let wad_type = String::from_utf8_lossy(&source[0..4]);
    
    let lumps_number : i32 = i32::from_le_bytes([source[4], source[5], source[6], source[7]]); // Are we sure?
    let wad_type = wad_type.try_into()?;

    let wad = Wad { source, wad_type, lumps_number };
    
    Ok(wad)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_read_wad() {
        assert!(matches!(
            parse("/home/emercier/Downloads/DOOM2.WAD"),
            Ok(Wad {
                source: _,
                wad_type: WadType::IWAD,
                lumps_number: 2919
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
