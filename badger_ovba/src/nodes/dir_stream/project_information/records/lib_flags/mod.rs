use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct LibFlagsRecord {
    id: u16,
    size: u32,
    lib_flags: u32,
}

impl LibFlagsRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0008,
            size: 0x00000004,
            lib_flags: 0x00000000,
        }
    }

    fn value(&self) -> u32 {
        self.lib_flags
    }
}

impl Parsable for LibFlagsRecord {
    type Output = LibFlagsRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let lib_flags = utils::get_u32(cursor)?;

        Ok(Self {
            id,
            size,
            lib_flags,
        })
    }
}
