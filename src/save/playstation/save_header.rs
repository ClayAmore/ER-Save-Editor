use binary_reader::BinaryReader;
use std::io;

use crate::{read::read::Read, write::write::Write};

pub struct SaveHeader {
    pub data: Vec<u8>,
}

impl Default for SaveHeader {
    fn default() -> Self {
        Self {
            data: vec![Default::default(); 0x70],
        }
    }
}
impl Read for SaveHeader {
    fn read(br: &mut BinaryReader) -> Result<SaveHeader, io::Error> {
        let mut header = SaveHeader::default();
        header.data.copy_from_slice(br.read_bytes(0x70)?);
        Ok(header)
    }
}

impl Write for SaveHeader {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        Ok(self.data.clone())
    }
}
