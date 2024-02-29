use crate::{read::read::Read, save::common::save_slot::SaveSlot, write::write::Write};

#[derive(Clone)]
pub struct PCSaveSlot {
    pub checksum: [u8; 0x10],
    pub save_slot: SaveSlot,
}

impl Default for PCSaveSlot{
    fn default() -> Self {
        Self { 
            checksum: [0x0; 0x10],
            save_slot: SaveSlot::default() 
        }
    }
}

impl Read for PCSaveSlot {
    fn read(br: &mut binary_reader::BinaryReader) -> Result<Self, std::io::Error> {
        let mut pc_save_slot = PCSaveSlot::default();

        // Checksum
        pc_save_slot.checksum.copy_from_slice(br.read_bytes(0x10)?);

        // Rest of the save slot
        pc_save_slot.save_slot = SaveSlot::read(br)?;

        Ok(pc_save_slot)
    }
}

impl Write for PCSaveSlot {
    fn write(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        
        // Get save slot bytes
        let save_slot_bytes = self.save_slot.write()?;

        // Calculate checksum 
        let digest = md5::compute(&save_slot_bytes);

        // Add Checksum
        bytes.extend(digest.iter().collect::<Vec<&u8>>());

        // Add the rest of the save slot
        bytes.extend(save_slot_bytes);

        Ok(bytes)
    }
}
