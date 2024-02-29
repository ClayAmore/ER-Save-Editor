use std::io;
use binary_reader::BinaryReader;

use crate::{read::read::Read, save::common::user_data_10::{CSKeyConfigSaveLoad, CSMenuSystemSaveLoad, ProfileSummary}, write::write::Write};


#[derive(Copy, Clone)]
pub struct  PCOptionData {
    _0x12: [u16; 0x9],
    _0xa0: [u8; 0xA0],
}

impl Default for PCOptionData {
    fn default() -> Self {
        Self { 
            _0x12: [0x0;0x9], 
            _0xa0: [0x0;0xa0] 
        }
    }
}

impl Read for PCOptionData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut pc_option_data = PCOptionData::default();
        
        for i in 0..pc_option_data._0x12.len() {
            pc_option_data._0x12[i] = br.read_u16()?;
        }

        pc_option_data._0xa0.copy_from_slice(br.read_bytes(0xa0)?);

        Ok(pc_option_data)
    }
}

impl Write for PCOptionData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..self._0x12.len() {
            bytes.extend(self._0x12[i].to_le_bytes());
        }
        bytes.extend(self._0xa0);
        Ok(bytes)
    }
}

pub struct UserData10 {
    pub checksum: [u8; 0x10],
    _0x19003b4: i32,
    pub steam_id: u64,
    _0x19004fc: [u8; 0x140],
    _cs_menu_system_save_load: CSMenuSystemSaveLoad,
    pub active_slot: [bool; 0xA],
    pub profile_summary: Vec<ProfileSummary>,
    _0x1903406: i32,
    _0x190340a: u8,
    _pc_option_data: PCOptionData,
    _cs_key_config_save_load: CSKeyConfigSaveLoad,
    _0x8: u64,
    _rest: Vec<u8>
}

impl Default for UserData10 {
    fn default() -> Self {
        Self {
            checksum: [0; 0x10],
            _0x19003b4: 0,
            steam_id: 0,
            _0x19004fc: [0; 0x140],
            _cs_menu_system_save_load: CSMenuSystemSaveLoad::default(),
            active_slot: [false; 0xA],
            profile_summary: vec![ProfileSummary::default(); 0xA],
            _0x1903406: 0,
            _0x190340a: 0,
            _pc_option_data: PCOptionData::default(),
            _cs_key_config_save_load: CSKeyConfigSaveLoad::default(),
            _0x8: 0,
            _rest: Vec::new(),
        }
    }
}

impl UserData10 {
    pub fn read(br: &mut BinaryReader) -> Result<UserData10, io::Error> {
        let mut user_data_10 = UserData10::default();

        let end = br.pos + 0x60010;

        // Checksum
        user_data_10.checksum.copy_from_slice(br.read_bytes(0x10)?);
        
        user_data_10._0x19003b4 = br.read_i32()?;
        
        // Steam ID
        user_data_10.steam_id = br.read_u64()?;
        
        user_data_10._0x19004fc.copy_from_slice(br.read_bytes(0x140)?);

        user_data_10._cs_menu_system_save_load = CSMenuSystemSaveLoad::read(br)?;

        for i in 0..0xA {
            let slot_active = br.read_bytes(1)?[0];
            assert!(slot_active == 0x1 || slot_active == 0x0);
            user_data_10.active_slot[i] = slot_active == 0x1;
        }
        
        for i in 0..0xA {
            let profile_summary = ProfileSummary::read(br)?;
            user_data_10.profile_summary[i] = profile_summary;
        }

        user_data_10._0x1903406 = br.read_i32()?;
        user_data_10._0x190340a = br.read_bytes(0x1)?[0];
        user_data_10._pc_option_data = PCOptionData::read(br)?;
        user_data_10._cs_key_config_save_load = CSKeyConfigSaveLoad::read(br)?;
        user_data_10._0x8 = br.read_u64()?;

        user_data_10._rest.extend(br.read_bytes(end - br.pos)?);

        Ok(user_data_10)
    }
}

impl Write for UserData10 {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // Checksum
        bytes.extend(self.checksum);

        bytes.extend(self._0x19003b4.to_le_bytes());
        
        // Steam ID
        bytes.extend(self.steam_id.to_le_bytes());

        bytes.extend(self._0x19004fc);

        bytes.extend(self._cs_menu_system_save_load.write()?);

        // Active Slots list
        bytes.extend(self.active_slot.iter().map(|a| if *a {1} else {0}).collect::<Vec<u8>>());

        // Profile Summaries
        for i in 0..0xA {
            bytes.extend(self.profile_summary[i].write()?);
        }

        bytes.extend(self._0x1903406.to_le_bytes());
        bytes.extend(self._0x190340a.to_le_bytes());

        bytes.extend(self._pc_option_data.write()?);
        bytes.extend(self._cs_key_config_save_load.write()?);
        bytes.extend(self._0x8.to_le_bytes());
        bytes.extend(self._rest.to_vec());

        // Recalculate checksum at the end
        let digest = md5::compute(&bytes[0x10..bytes.len()]);
        for i in 0..0x10 {
            bytes[i] = digest[i];
        }
        
        Ok(bytes)
    }
}