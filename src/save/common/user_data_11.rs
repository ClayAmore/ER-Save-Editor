use std::io::Error;
use binary_reader::BinaryReader;
use crate::write::write::Write;
use crate::read::read::Read;

// The regulation and the zero padding that follows it always add up to this
// much. The total has been stable across game patches, but the boundary between
// the two moves every time the regulation itself grows (DLC, balance patches),
// so it is located by scanning rather than hardcoded.
const REGULATION_BLOCK_SIZE: usize = 0x240000;

pub struct UserData11 {
    unk: [u8;0x10],
    pub regulation: Vec<u8>,
    rest: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self { 
            unk: Default::default(), 
            regulation: vec![0; REGULATION_BLOCK_SIZE],
            rest: Vec::new()
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);

        let block = br.read_bytes(REGULATION_BLOCK_SIZE)?;

        // The block is zero filled after the regulation, so the trailing run of
        // zeros marks where the real data ended.
        let data_len = block.iter().rposition(|byte| *byte != 0).map_or(0, |i| i + 1);

        // Rounding up to a whole AES block keeps decryption aligned, and also
        // takes back any zero bytes the ciphertext happened to end on.
        let regulation_len = data_len.next_multiple_of(0x10).min(REGULATION_BLOCK_SIZE);

        user_data_11.regulation = block[..regulation_len].to_vec();
        user_data_11.rest = block[regulation_len..].to_vec();

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