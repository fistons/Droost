//! PATCH picture format parser
use crate::wad::WadError;

#[derive(Debug)]
pub struct Header {
    pub width: u16,
    pub height: u16,
    pub left_offset: i16,
    pub top_offset: i16,
}

impl TryFrom<[u8; 8]> for Header {
    
    type Error = WadError;
    
    fn try_from(value: [u8; 8]) -> Result<Self, Self::Error> {
        let width = u16::from_le_bytes(value[0..2].try_into()?);
        let height = u16::from_le_bytes(value[2..4].try_into()?);
        let left_offset = i16::from_le_bytes(value[4..6].try_into()?);
        let top_offset = i16::from_le_bytes(value[6..8].try_into()?);

        Ok(Header {
            width,
            height,
            left_offset,
            top_offset,
        })
    }
}
