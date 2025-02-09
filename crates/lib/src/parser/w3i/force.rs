use binary_reader::BinaryReader;
use serde::{Deserialize, Serialize};

use crate::parser::binary_reader::{AutoReadable, BinaryReadable};
use crate::parser::error::ParserError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Force {
    pub flags: u32,
    pub player_masks: u32,
    pub name: String,
}

impl BinaryReadable for Force {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        Ok(Self {
            flags: AutoReadable::read(stream)?,
            player_masks: AutoReadable::read(stream)?,
            name: AutoReadable::read(stream)?,
        })
    }
}
