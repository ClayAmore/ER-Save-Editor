use std::io;
use binary_reader::BinaryReader;
use crate::{read::read::Read, save::common::{save_slot::SaveSlot, user_data_11::UserData11}, write::write::Write};
use super::{save_header::SaveHeader, user_data_10::UserData10};


pub struct PSSave {
    pub header: SaveHeader,
    pub save_slots: Vec<SaveSlot>,
    pub user_data_10: UserData10,
    pub user_data_11: UserData11
}
impl Default for PSSave {
    fn default() -> Self {
        Self {
            header: Default::default(),
            save_slots: vec![SaveSlot::default(); 0xA],
            user_data_10: UserData10::default(),
            user_data_11: UserData11::default()
        }
    }
}
impl Read for PSSave {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut save = PSSave::default();

        save.header = SaveHeader::read(br)?;

        for i in 0..0xA {
            save.save_slots[i] = SaveSlot::read(br)?;
        }

        save.user_data_10 = UserData10::read(br)?;
        save.user_data_11 = UserData11::read(br)?;
        
        Ok(save)
    }
}
impl Write for PSSave {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // Header
        bytes.extend(self.header.write()?);

        for i in 0..0xA {
            bytes.extend(self.save_slots[i].write()?)
        }

        bytes.extend(self.user_data_10.write()?);
        bytes.extend(self.user_data_11.write()?);

        Ok(bytes)
    }
}