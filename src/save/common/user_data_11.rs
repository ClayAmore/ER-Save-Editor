use crate::read::read::Read;
use crate::write::write::Write;
use binary_reader::BinaryReader;
use std::io::Error;

pub struct UserData11 {
    unk: [u8; 0x10],
    pub regulation: Vec<u8>,
    rest: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self {
            unk: Default::default(),
            regulation: vec![0; 0x1c5f70],
            rest: vec![0; 0x7A090],
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);
        user_data_11
            .regulation
            .copy_from_slice(br.read_bytes(0x1c5f70)?);
        user_data_11.rest.copy_from_slice(br.read_bytes(0x7A090)?);
        assert!(user_data_11.rest[0] == 0 || user_data_11.rest[0] == 151);
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
