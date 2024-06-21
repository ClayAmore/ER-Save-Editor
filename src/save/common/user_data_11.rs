use std::io::Error;
use binary_reader::BinaryReader;
use crate::write::write::Write;
use crate::read::read::Read;

pub struct UserData11 {
    unk: [u8;0x10],
    pub regulation: Vec<u8>,
    rest: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self { 
            unk: Default::default(), 
            regulation: vec![0; 0x1e9fb0],
            rest: vec![0;0x56050]
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);
        user_data_11.regulation.copy_from_slice(br.read_bytes(0x1e9fb0)?);
        user_data_11.rest.copy_from_slice(br.read_bytes(0x56050)?);
        assert_eq!(user_data_11.rest[0], 0);
        Ok(user_data_11)
    }
}

impl Write for UserData11 {
    fn write(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk);
        bytes.extend(self.regulation.to_vec());
        bytes.extend(self.rest.to_vec());
        Ok(bytes)
    }
}