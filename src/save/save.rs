pub mod save {
    use std::{fs, io, path::PathBuf};
    use binary_reader::BinaryReader;
    use crate::{read::read::Read, save::{save_header::SaveHeader, save_slot::SaveSlot, user_data_10::UserData10, user_data_11::UserData11}, write::write::Write};

    // Using a checksum of the regulation bin file to check for Save Wizard .txt save file
    const REGULATION_MD5_CHECKSUM: [u8; 0x10] = [0x9D, 0xE4, 0x83, 0x80, 0x78, 0xB2, 0x28, 0x9D, 0x83, 0x8D, 0x28, 0x7A, 0x24, 0x31, 0xE6, 0x45];

    pub enum SaveType {
        Unknown,
        PC,
        SaveWizard
    }

    pub struct Save {
        pub save_type: SaveType,
        pub header: SaveHeader,
        pub save_slots: Vec<SaveSlot>,
        pub user_data_10: UserData10,
        pub user_data_11: UserData11
    }

    impl Default for Save {
        fn default() -> Self {
            Self {  
                save_type: SaveType::Unknown,
                header: Default::default(),
                save_slots: vec![SaveSlot::default(); 0xA],
                user_data_10: UserData10::default(),
                user_data_11: UserData11::default()
            }
        }
    }
    
    impl Read for Save {
        fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
            // Check if it's a PC save or a PS Save Wizard
            if Self::is_pc(br) {
                return Self::read_pc(br);
            }
            else if Self::is_ps_save_wizard(br) {
                return Self::read_ps_save_wizard(br);
            }

            Err( std::io::Error::new(io::ErrorKind::InvalidData, "Invalid data!") )
        }
    }

    impl Write for Save {
        fn write(&self) -> Result<Vec<u8>, io::Error> {
            let save_bytes: Vec<u8> =  match self.save_type {
                SaveType::Unknown => Vec::new(),
                SaveType::PC => self.write_pc()?,
                SaveType::SaveWizard => self.write_ps_save_wizard()?,
            };

            Ok(save_bytes)
        }
    }

    impl Save {
        pub fn from_path(path: &PathBuf) -> Result<Save, io::Error> {
            let contents = fs::read(path).expect("Should have been able to read the file");
            let mut br = BinaryReader::from_u8(&contents);
            br.set_endian(binary_reader::Endian::Little);

            // Check if it's an actual save file
            assert!(Self::is(&mut br));

            Self::read(&mut br)
        }

        // Check if it's a save file
        pub fn is(br: &mut BinaryReader) -> bool {
            let is = Self::is_pc(br) || Self::is_ps_save_wizard(br);
            is
        }

        // Check if it's a PC save file
        pub fn is_pc(br: &mut BinaryReader) -> bool {
            let magic = br.read_bytes(4).expect("");
            let is_pc = magic == [66, 78, 68, 52];
            br.jmp(0);
            is_pc
        }

        // Check if it's a PS Save Wizard save file
        pub fn is_ps_save_wizard(br: &mut BinaryReader) -> bool {
            br.jmp(0x1960080);
            let regulation = br.read_bytes(0x1c5f70).expect("");
            let digest = md5::compute(regulation);
            let is_ps_save_wizard = digest == md5::Digest(REGULATION_MD5_CHECKSUM);
            br.jmp(0);
            is_ps_save_wizard
        }

        fn read_pc(br: &mut BinaryReader) -> Result<Save, io::Error>  {
            let mut save = Save::default();
            save.save_type = SaveType::PC;

            save.header = SaveHeader::read(br)?;

            for i in 0..0xA {
                save.save_slots[i] = SaveSlot::read(br)?;
            }

            save.user_data_10 = UserData10::read(br)?;
            save.user_data_11 = UserData11::read(br)?;
            
            Ok(save)
        }

        fn read_ps_save_wizard(_br: &mut BinaryReader) -> Result<Save, io::Error>  {
            let mut save = Save::default();
            save.save_type = SaveType::SaveWizard;
            Ok(save)
        }

        fn write_pc(&self) -> Result<Vec<u8>, io::Error> {
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

        fn write_ps_save_wizard(&self) -> Result<Vec<u8>, io::Error> {
            let bytes: Vec<u8> = Vec::new();

            Ok(bytes)
        }
    }
    
}

