use crate::wad::WadError;

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
