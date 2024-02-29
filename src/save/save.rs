pub mod save {
    use std::{fs, io, path::PathBuf};
    use binary_reader::BinaryReader;
    use crate::{
        read::read::Read, save::{
            common::{ save_slot::SaveSlot, user_data_10::ProfileSummary },
            pc::pc_save::PCSave, 
            playstation::ps_save::PSSave, 
        }, util::bit::bit::set_bit, write::write::Write
    };

    // Using a checksum of the regulation bin file to check for Save Wizard .txt save file
    const REGULATION_MD5_CHECKSUM: [u8; 0x10] = [0x9D, 0xE4, 0x83, 0x80, 0x78, 0xB2, 0x28, 0x9D, 0x83, 0x8D, 0x28, 0x7A, 0x24, 0x31, 0xE6, 0x45];

    pub enum SaveType {
        Unknown,
        PC(PCSave),
        PlayStation(PSSave)
    }
    
    #[allow(unused)]
    impl SaveType {
        pub fn get_global_steam_id(&self) -> u64 {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.user_data_10.steam_id
                }
                SaveType::PlayStation(_) => 0,
            }
        }

        pub fn set_global_steam_id(&mut self, steam_id: u64) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.user_data_10.steam_id = steam_id;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn active_slots(&self) -> [bool; 10] {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => pc_save.user_data_10.active_slot,
                SaveType::PlayStation(ps_save) => ps_save.user_data_10.active_slot,
            }
        }

        pub fn get_character_steam_id(&self, index: usize) -> u64 {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id
                }
                SaveType::PlayStation(_) => 0,
            }
        }

        pub fn set_character_steam_id(&mut self, index: usize, steam_id: u64) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id = steam_id;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_name(&mut self, index: usize, character_name_str: String) {
            let mut character_name: [u16; 0x10] = [0; 0x10];
            let mut character_name2: [u16; 0x11] = [0; 0x11];
            for (i, char) in character_name_str.chars().enumerate() {
                character_name[i] = char as u16;
                character_name2[i] = char as u16;
            }
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.character_name.copy_from_slice(&character_name);
                    pc_save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_gender(&mut self, index: usize, gender: u8) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.gender = gender;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_health(&mut self, index: usize, health: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.health = health;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_base_max_health(&mut self, index: usize, base_max_health: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_health = base_max_health;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_fp(&mut self, index: usize, fp: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.fp = fp;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_base_max_fp(&mut self, index: usize, base_max_fp: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_fp = base_max_fp;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_sp(&mut self, index: usize, sp: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.sp = sp;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_base_max_sp(&mut self, index: usize, base_max_sp: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_sp = base_max_sp;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }
        
        pub fn set_character_level(&mut self, index: usize, level: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.level = level;
                    pc_save.user_data_10.profile_summary[index].level = level;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_vigor(&mut self, index: usize, vigor: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.vigor = vigor;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_mind(&mut self, index: usize, mind: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.mind = mind;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_endurance(&mut self, index: usize, endurance: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.endurance = endurance;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_strength(&mut self, index: usize, strength: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.strength = strength;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_dexterity(&mut self, index: usize, dexterity: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.dexterity = dexterity;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_intelligence(&mut self, index: usize, intelligence: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.intelligence = intelligence;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_faith(&mut self, index: usize, faith: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.faith = faith;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_arcane(&mut self, index: usize, arcane: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.arcane = arcane;
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn set_character_event_flag(&mut self, index: usize, offset: usize, bit_pos: u8, state: bool) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    let event_byte = pc_save.save_slots[index].save_slot.event_flags.flags[offset];
                    pc_save.save_slots[index].save_slot.event_flags.flags[offset] = set_bit(event_byte, bit_pos, state);
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn add_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    let res = pc_save.save_slots[index].save_slot.regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {},
                        None => {
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions.push(region_id);
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions_count = pc_save.save_slots[index].save_slot.regions.unlocked_regions_count + 1;
                        },
                    }
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn remove_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    let res = pc_save.save_slots[index].save_slot.regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions.swap_remove(i);
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions_count = pc_save.save_slots[index].save_slot.regions.unlocked_regions_count - 1;
                        },
                        None => {},
                    }
                }
                SaveType::PlayStation(_) => todo!(),
            }
        }

        pub fn get_profile_summary(&self, index: usize) -> ProfileSummary {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => pc_save.user_data_10.profile_summary[index],
                SaveType::PlayStation(ps_save) => ps_save.user_data_10.profile_summary[index],
            }
        }

        pub fn get_slot(&self, index: usize) -> &SaveSlot {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => &pc_save.save_slots[index].save_slot,
                SaveType::PlayStation(ps_save) => &ps_save.save_slots[index],
            }
        }
    }

    pub struct Save {
        pub save_type: SaveType,
    }

    impl Default for Save {
        fn default() -> Self {
            Self {  
                save_type: SaveType::Unknown,
            }
        }
    }
    
    impl Read for Save {
        fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
            let mut save = Save::default();

            if Self::is_pc(br) {
                save.save_type = SaveType::PC(PCSave::read(br)?);
            }
            else if Self::is_ps_save_wizard(br) {
                save.save_type = SaveType::PlayStation(PSSave::read(br)?);
            }
            else {
                return Err( std::io::Error::new(io::ErrorKind::InvalidData, "Invalid data!") );
            }
            
            Ok(save)
        }
    }

    impl Write for Save {
        fn write(&self) -> Result<Vec<u8>, io::Error> {
            let save_bytes: Vec<u8> =  match &self.save_type {
                SaveType::Unknown => Vec::new(),
                SaveType::PC(pc_save) => pc_save.write()?,
                SaveType::PlayStation(ps_save) => ps_save.write()?,
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
    }
    
}

