use std::io;
use binary_reader::BinaryReader;
use crate::{read::read::Read, write::write::Write};
use super::{save_header::SaveHeader, save_slot::PCSaveSlot, user_data_10::UserData10, user_data_11::PcUserData11};

pub struct PCSave {
    pub header: SaveHeader,
    pub save_slots: Vec<PCSaveSlot>,
    pub user_data_10: UserData10,
    pub user_data_11: PcUserData11
}
impl Default for PCSave {
    fn default() -> Self {
        Self {
            header: SaveHeader::default(),
            save_slots: vec![PCSaveSlot::default(); 0xA],
            user_data_10: UserData10::default(),
            user_data_11: PcUserData11::default()
        }
    }
}
impl Read for PCSave {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut save = PCSave::default();

        save.header = SaveHeader::read(br)?;

        for i in 0..0xA {
            save.save_slots[i] = PCSaveSlot::read(br)?;
        }

        save.user_data_10 = UserData10::read(br)?;
        save.user_data_11 = PcUserData11::read(br)?;
        
        Ok(save)
    }
}
impl Write for PCSave {
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
