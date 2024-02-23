pub mod save {
    use std::{fs, io, path::PathBuf};
    use binary_reader::BinaryReader;
    use crate::save::{save_slot::SaveSlot, user_data_10::UserData10};

    // Using a checksum of the regulation bin file to check for Save Wizard .txt save file
    const REGULATION_MD5_CHECKSUM: [u8; 0x10] = [0x9D, 0xE4, 0x83, 0x80, 0x78, 0xB2, 0x28, 0x9D, 0x83, 0x8D, 0x28, 0x7A, 0x24, 0x31, 0xE6, 0x45];

    pub enum SaveType {
        Unknown,
        PC,
        SaveWizard
    }

    pub struct Save {
        pub save_type: SaveType,
        pub save_slots: Vec<SaveSlot>,
        pub user_data_10: UserData10,
    }

    impl Default for Save {
        fn default() -> Self {
            Self {  
                save_type: SaveType::Unknown,
                save_slots: vec![SaveSlot::default(); 0xA],
                user_data_10: UserData10::default()
            }
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
            return Self::is_pc(br) || Self::is_ps_save_wizard(br);
        }

        // Check if it's a PC save file
        pub fn is_pc(br: &mut BinaryReader) -> bool {
            let magic = br.read_bytes(4).expect("");
            return magic == [66, 78, 68, 52];
        }

        // Check if it's a PS Save Wizard save file
        pub fn is_ps_save_wizard(br: &mut BinaryReader) -> bool {
            br.jmp(0x1960080);
            let regulation = br.read_bytes(0x1c5f70).expect("");
            let digest = md5::compute(regulation);
            return digest == md5::Digest(REGULATION_MD5_CHECKSUM);
        }

        pub fn read(br: &mut BinaryReader) -> Result<Save, io::Error> {
            // Reset pos to start of file.
            br.jmp(0);
            
            // Check if it's a PC save or a PS Save Wizard
            if Self::is_pc(br) {
                return Self::read_pc(br);
            }
            else if Self::is_ps_save_wizard(br) {
                return Self::read_ps_save_wizard(br);
            }

            Err( std::io::Error::new(io::ErrorKind::InvalidData, "Invalid data!") )
        }

        fn read_pc(br: &mut BinaryReader) -> Result<Save, io::Error>  {
            let mut save = Save::default();
            save.save_type = SaveType::PC;

            save.user_data_10 = UserData10::read(br)?;

            for i in 0..0xA {
                save.save_slots[i] = SaveSlot::read(br, i)?;
            }
            
            Ok(save)
        }

        fn read_ps_save_wizard(_br: &mut BinaryReader) -> Result<Save, io::Error>  {
            let mut save = Save::default();
            save.save_type = SaveType::SaveWizard;
            Ok(save)
        }
    }

    
}

