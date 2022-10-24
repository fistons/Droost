use std::borrow::Cow;

use crate::wad::WadError;

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

#[derive(Debug)]
pub struct WadHeader {
    pub wad_type: WadType,
    pub lumps_number: i32,
    pub directory_offset: i32,
}

impl WadHeader {
    pub fn new(data: &[u8; 16]) -> Result<WadHeader, WadError> {
        let wad_type: WadType = String::from_utf8_lossy(&data[0..4]).try_into()?;
        let lumps_number: i32 = i32::from_le_bytes(data[4..8].try_into()?);
        let directory_offset: i32 = i32::from_le_bytes(data[8..12].try_into()?);

        Ok(WadHeader {
            wad_type,
            lumps_number,
            directory_offset,
        })
    }
}
