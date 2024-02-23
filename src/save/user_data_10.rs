use std::io;
use binary_reader::BinaryReader;

#[derive(Copy, Clone)]
pub struct ProfileSummary{
    pub character_name: [u8; 0x22],
    pub level: u32,
}

impl Default for ProfileSummary {
    fn default() -> Self {
        Self {
            character_name: [0x0; 0x22],
            level: 0
        }
    }
}

impl ProfileSummary {
    fn read(br: &mut BinaryReader) -> Result<ProfileSummary, io::Error> {
        let mut profile_summary = ProfileSummary::default();
        
        let character_name = br.read_bytes(0x22)?;
        
        profile_summary.character_name.copy_from_slice(character_name);
        profile_summary.level = br.read_u32()?;
        
        let _0x28 = br.read_u32()?;
        let _0x2c = br.read_u32()?;
        let _0x30 = br.read_u32()?;
        let _0x34 = br.read_u32()?;
        let _0x38_0x150 = br.read_u32()?;
        let _0x38_0x8 = br.read_bytes(0x120)?;
        let _0x1a0 = br.read_bytes(0xe8)?;
        let _0x290 = br.read_bytes(1);
        let _0x291 = br.read_bytes(1);
        let _0x292 = br.read_bytes(1);
        let _0x293 = br.read_bytes(1);
        let _0x294 = br.read_bytes(1);
        let _0x295 = br.read_bytes(1);
        let _0x298 = br.read_i32();

        Ok(profile_summary)
    }
}

pub struct UserData10 {
    pub checksum: [u8; 0x10],
    pub steam_id: u64,
    pub active_slot: [bool; 0xA],
    pub profile_summary: Vec<ProfileSummary>
}

impl Default for UserData10 {
    fn default() -> Self {
        Self {
            checksum: [0; 0x10],
            steam_id: 0,
            active_slot: [false; 0xA],
            profile_summary: vec![ProfileSummary::default(); 0xA]
        }
    }
}

impl UserData10 {
    pub fn read(br: &mut BinaryReader) -> Result<UserData10, io::Error> {
        let mut user_data_10 = UserData10::default();

        // Jump to start of user_data_10
        br.jmp(0x19003a0);

        // Checksum
        user_data_10.checksum.copy_from_slice(br.read_bytes(0x10)?);
        
        let _0x19003b4 = br.read_i32()?;
        user_data_10.steam_id = br.read_u64()?;
        
        let _0x19004fc = br.read_bytes(0x140)?;
        let _0x1900500 = br.read_i32()?;
        
        let len = br.read_i32()?;
        let _0x1901d04 = br.read_bytes(len as usize)?;

        for i in 0..0xA {
            let slot_active = br.read_bytes(1)?[0];
            assert!(slot_active == 0x1 || slot_active == 0x0);
            user_data_10.active_slot[i] = slot_active == 0x1;
        }
        
        for i in 0..0xA {
            let profile_summary = ProfileSummary::read(br)?;
            user_data_10.profile_summary[i] = profile_summary;
        }

        let _0x1903406 = br.read_i32()?;
        let _0x190340a = br.read_bytes(0x1)?;
        let _0x190340b = br.read_bytes(0x12)?;
        let _0x1903414 = br.read_bytes(0xa0)?;
        let _0x19034b4 = br.read_i32()?;
        
        let len = br.read_i32()?;
        let _0x19034bc = br.read_bytes(len as usize)?;
        let _0x19054c5 = br.read_i32()?;

        Ok(user_data_10)
    }
}