use std::io;
use binary_reader::BinaryReader;

use crate::{read::read::Read, write::write::Write};

pub struct UserData11 {
    checksum: [u8; 0x10],
    unk: [u8;0x10],
    regulation: Vec<u8>,
    rest: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self { 
            checksum: Default::default(), 
            unk: Default::default(), 
            regulation: vec![0; 0x1c5f70],
            rest: vec![0;0x7A090]
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, io::Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.checksum.copy_from_slice(br.read_bytes(0x10)?);
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);
        user_data_11.regulation.copy_from_slice(br.read_bytes(0x1c5f70)?);
        user_data_11.rest.copy_from_slice(br.read_bytes(0x7A090)?);
        assert_eq!(user_data_11.rest[0], 0);
        Ok(user_data_11)
    }
}

impl Write for UserData11 {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.checksum);
        bytes.extend(self.unk);
        bytes.extend(self.regulation.to_vec());
        bytes.extend(self.rest.to_vec());
        
        // Recalculate checksum at the end
        let digest = md5::compute(&bytes[0x10..bytes.len()]);
        for i in 0..0x10 {
            bytes[i] = digest[i];
        }

        Ok(bytes)
    }
}