use std::io;
use binary_reader::BinaryReader;

use crate::{read::read::Read, save::common::user_data_10::{CSKeyConfigSaveLoad, CSMenuSystemSaveLoad, ProfileSummary}, write::write::Write};

pub struct UserData10 {
    _0x19003b4: i32,
    pub steam_id: u64,
    _0x19004fc: [u8; 0x140],
    _cs_menu_system_save_load: CSMenuSystemSaveLoad,
    pub active_slot: [bool; 0xA],
    pub profile_summary: Vec<ProfileSummary>,
    _0x1903406: i32,
    _0x190340a: u8,
    _cs_key_config_save_load: CSKeyConfigSaveLoad,
    _0x8: u64,
    _rest: Vec<u8>
}

impl Default for UserData10 {
    fn default() -> Self {
        Self {
            _0x19003b4: 0,
            steam_id: 0,
            _0x19004fc: [0; 0x140],
            _cs_menu_system_save_load: CSMenuSystemSaveLoad::default(),
            active_slot: [false; 0xA],
            profile_summary: vec![ProfileSummary::default(); 0xA],
            _0x1903406: 0,
            _0x190340a: 0,
            _cs_key_config_save_load: CSKeyConfigSaveLoad::default(),
            _0x8: 0,
            _rest: Vec::new(),
        }
    }
}

impl Read for UserData10 {
    fn read(br: &mut BinaryReader) -> Result<UserData10, io::Error> {
        let mut user_data_10 = UserData10::default();

        let end = br.pos + 0x60000;
        
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
        user_data_10._cs_key_config_save_load = CSKeyConfigSaveLoad::read(br)?;
        user_data_10._0x8 = br.read_u64()?;

        user_data_10._rest.extend(br.read_bytes(end - br.pos)?);

        Ok(user_data_10)
    }
}

impl Write for UserData10 {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

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

        bytes.extend(self._cs_key_config_save_load.write()?);
        bytes.extend(self._0x8.to_le_bytes());
        bytes.extend(self._rest.to_vec());

        Ok(bytes)
    }
}