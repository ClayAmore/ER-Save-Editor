// Common user_data_10 chunks that is in both PC and Playstation save 
use std::io;
use binary_reader::BinaryReader;
use crate::{read::read::Read, write::write::Write};

pub struct CSMenuSystemSaveLoad {
    unk: u32,
    length: u32,
    data: Vec<u8>
}

impl Default for CSMenuSystemSaveLoad {
    fn default() -> Self {
        Self { unk: Default::default(), length: Default::default(), data: Default::default() }
    }
}

impl Read for CSMenuSystemSaveLoad {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut cs_menu_system_save_load = CSMenuSystemSaveLoad::default();
        cs_menu_system_save_load.unk = br.read_u32()?;
        cs_menu_system_save_load.length = br.read_u32()?;
        cs_menu_system_save_load.data.extend(br.read_bytes(cs_menu_system_save_load.length as usize)?);
        Ok(cs_menu_system_save_load)
    }
}

impl Write for CSMenuSystemSaveLoad {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk.to_le_bytes());
        bytes.extend(self.length.to_le_bytes());
        bytes.extend(self.data.to_vec());
        Ok(bytes)
    }
}

#[derive(Copy, Clone)]
pub struct ProfileSummary{
    pub character_name: [u16; 0x11],
    pub level: u32,
    _0x28: u32 ,
    _0x2c: u32 ,
    _0x30: u32 ,
    _0x34: u32 ,
    _0x38_0x150: u32 ,
    _0x38_0x8: [u8;0x120] ,
    _0x1a0: [u8;0xe8] ,
    _0x290: u8 ,
    _0x291: u8 ,
    _0x292: u8 ,
    _0x293: u8 ,
    _0x294: u8 ,
    _0x295: u8 ,
    _0x298: i32 ,
}

impl Default for ProfileSummary {
    fn default() -> Self {
        Self {
            character_name: [0x0; 0x11],
            level: 0,
            _0x28: 0,
            _0x2c: 0,
            _0x30: 0,
            _0x34: 0,
            _0x38_0x150: 0,
            _0x38_0x8: [0x0; 0x120],
            _0x1a0: [0x0; 0xe8],
            _0x290: 0,
            _0x291: 0,
            _0x292: 0,
            _0x293: 0,
            _0x294: 0,
            _0x295: 0,
            _0x298: 0,
        }
    }
}

impl Read for ProfileSummary {
    fn read(br: &mut BinaryReader) -> Result<ProfileSummary, io::Error> {
        let mut profile_summary = ProfileSummary::default();
        for i in 0..0x11 { profile_summary.character_name[i] = br.read_u16()?;}
        profile_summary.level = br.read_u32()?;
        profile_summary._0x28 = br.read_u32()?;
        profile_summary._0x2c = br.read_u32()?;
        profile_summary._0x30 = br.read_u32()?;
        profile_summary._0x34 = br.read_u32()?;
        profile_summary._0x38_0x150 = br.read_u32()?;
        profile_summary._0x38_0x8.copy_from_slice(br.read_bytes(0x120)?);
        profile_summary._0x1a0.copy_from_slice(br.read_bytes(0xe8)?);
        profile_summary._0x290 = br.read_u8()?;
        profile_summary._0x291 = br.read_u8()?;
        profile_summary._0x292 = br.read_u8()?;
        profile_summary._0x293 = br.read_u8()?;
        profile_summary._0x294 = br.read_u8()?;
        profile_summary._0x295 = br.read_u8()?;
        profile_summary._0x298 = br.read_i32()?;
        Ok(profile_summary)
    }
}

impl Write for ProfileSummary{
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..0x11 { bytes.extend(self.character_name[i].to_le_bytes());}
        bytes.extend(self.level.to_le_bytes());
        bytes.extend(self._0x28.to_le_bytes());
        bytes.extend(self._0x2c.to_le_bytes());
        bytes.extend(self._0x30.to_le_bytes());
        bytes.extend(self._0x34.to_le_bytes());
        bytes.extend(self._0x38_0x150.to_le_bytes());
        bytes.extend(self._0x38_0x8);
        bytes.extend(self._0x1a0);
        bytes.push(self._0x290);
        bytes.push(self._0x291);
        bytes.push(self._0x292);
        bytes.push(self._0x293);
        bytes.push(self._0x294);
        bytes.push(self._0x295);
        bytes.extend(self._0x298.to_le_bytes());
        Ok(bytes)
    }
}


#[derive(Clone)]

pub struct CSKeyConfigSaveLoad {
    length: i32,
    elements: Vec<u8>
}

impl Default for CSKeyConfigSaveLoad {
    fn default() -> Self {
        Self { length: Default::default(), elements: Default::default() }
    }
}

impl Read for CSKeyConfigSaveLoad {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut cs_key_config_save_load = CSKeyConfigSaveLoad::default();
        cs_key_config_save_load.length = br.read_i32()?;
        cs_key_config_save_load.elements.extend(br.read_bytes(cs_key_config_save_load.length as usize)?);
        Ok(cs_key_config_save_load)
    }
}

impl Write for CSKeyConfigSaveLoad {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.length.to_le_bytes());
        bytes.extend(self.elements.to_vec());
        Ok(bytes)
    }
}