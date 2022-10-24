use crate::wad::WadError;

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
