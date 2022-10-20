use std::borrow::Cow;
use std::{fs, io, str};

const LUMP_HEADER_SIZE: usize = 16;
#[derive(Debug)]
pub struct LumpHeader {
    offset: i32,
    size: i32,
    name: String,
}

#[derive(Debug)]
pub struct WadHeader {
    wad_type: WadType,
    lumps_number: i32,
    directory_offset: i32,
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

pub fn parse(wad_path: &str) -> Result<WadHeader, WadError> {
    let source = fs::read(wad_path)?;
    let wad_type = String::from_utf8_lossy(&source[0..4]);

    let lumps_number: i32 = i32::from_le_bytes([source[4], source[5], source[6], source[7]]); // Are we sure?
    let directory_offset: i32 = i32::from_le_bytes([source[8], source[9], source[10], source[11]]); // Are we sure?
    let wad_type = wad_type.try_into()?;
    
    
    for x in (0..source.len() - directory_offset as usize).step_by(LUMP_HEADER_SIZE) {
        let offset = directory_offset as usize + x;

 
        let lump_name = String::from_utf8_lossy(&source[offset + 8 .. offset + 16]);

        let sub = [
            source[offset],
            source[offset + 1],
            source[offset + 2],
            source[offset + 3],
        ];
        let data_position = i32::from_le_bytes(sub);

        let sub = [
            source[offset + 4],
            source[offset + 5],
            source[offset + 6],
            source[offset + 7],
        ];
        let lump_size = i32::from_le_bytes(sub);

        let lump_header = LumpHeader{offset: data_position, size: lump_size, name: lump_name.into_owned()};
        println!("{:?}", lump_header);
    }
    

    let wad = WadHeader {
        wad_type,
        lumps_number,
        directory_offset
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
            parse("/home/eric/Downloads/DOOM2.WAD"),
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
            parse("/home/eric/Downloads/url.png"),
            Err(WadError::UnkownWadType(_))
        ));
    }
}
