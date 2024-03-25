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

#[derive(Default, Copy, Clone)]
pub struct ProfileSummaryEquipmentGaitem {
    unk: u32,
    unk1: u32,
    pub arm_style: u32,
    pub left_hand_active_slot: u32,
    pub right_hand_active_slot: u32,
    pub left_arrow_active_slot: u32,
    pub right_arrow_active_slot: u32,
    pub left_bolt_active_slot: u32,
    pub right_bolt_active_slot: u32,
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    pub _0x4: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    pub _0x4_2: u32,
    pub talismans: [u32; 4],
    pub _0x4_3: u32
}
impl Read for ProfileSummaryEquipmentGaitem {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equipment = ProfileSummaryEquipmentGaitem::default();
        equipment.unk = br.read_u32()?;
        equipment.unk1 = br.read_u32()?;

        equipment.arm_style = br.read_u32()?;
        equipment.left_hand_active_slot = br.read_u32()?;
        equipment.right_hand_active_slot = br.read_u32()?;
        equipment.left_arrow_active_slot = br.read_u32()?;
        equipment.right_arrow_active_slot = br.read_u32()?;
        equipment.left_bolt_active_slot = br.read_u32()?;
        equipment.right_bolt_active_slot = br.read_u32()?;
        equipment.left_hand_armaments[0] = br.read_u32()?;
        equipment.right_hand_armaments[0] = br.read_u32()?;
        equipment.left_hand_armaments[1] = br.read_u32()?;
        equipment.right_hand_armaments[1] = br.read_u32()?;
        equipment.left_hand_armaments[2] = br.read_u32()?;
        equipment.right_hand_armaments[2] = br.read_u32()?;
        equipment.arrows[0] = br.read_u32()?;
        equipment.bolts[0] = br.read_u32()?;
        equipment.arrows[1] = br.read_u32()?;
        equipment.bolts[1] = br.read_u32()?;
        equipment._0x4 = br.read_u32()?;
        equipment.head = br.read_u32()?;
        equipment.chest = br.read_u32()?;
        equipment.arms = br.read_u32()?;
        equipment.legs = br.read_u32()?;
        equipment._0x4_2 = br.read_u32()?;
        for i in 0..4 { equipment.talismans[i] = br.read_u32()?; }
        equipment._0x4_3 = br.read_u32()?;
        Ok(equipment)
    }
}
impl Write for ProfileSummaryEquipmentGaitem {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.unk.to_le_bytes());
        bytes.extend(self.unk1.to_le_bytes());

        bytes.extend(self.arm_style.to_le_bytes());
        bytes.extend(self.left_hand_active_slot.to_le_bytes());
        bytes.extend(self.right_hand_active_slot.to_le_bytes());
        bytes.extend(self.left_arrow_active_slot.to_le_bytes());
        bytes.extend(self.right_arrow_active_slot.to_le_bytes());
        bytes.extend(self.left_bolt_active_slot.to_le_bytes());
        bytes.extend(self.right_bolt_active_slot.to_le_bytes());
        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());
        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());
        bytes.extend(self._0x4_2.to_le_bytes());
        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        bytes.extend(self._0x4_3.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Default, Copy, Clone)]
pub struct ProfileSummaryEquipmentItem {
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    _0x4: u32,
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    _0x8: u64,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    _0x4_2: u32,
    pub talismans: [u32; 4],
    _0x4_3: [u32; 6]
}
impl Read for ProfileSummaryEquipmentItem {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equipment = ProfileSummaryEquipmentItem::default();

        equipment.left_hand_armaments[0] = br.read_u32()?;
        equipment.right_hand_armaments[0] = br.read_u32()?;
        equipment.left_hand_armaments[1] = br.read_u32()?;
        equipment.right_hand_armaments[1] = br.read_u32()?;
        equipment.left_hand_armaments[2] = br.read_u32()?;
        equipment.right_hand_armaments[2] = br.read_u32()?;
        equipment._0x4 = br.read_u32()?;
        equipment.arrows[0] = br.read_u32()?;
        equipment.bolts[0] = br.read_u32()?;
        equipment.arrows[1] = br.read_u32()?;
        equipment.bolts[1] = br.read_u32()?;
        equipment._0x8 = br.read_u64()?;
        equipment.head = br.read_u32()?;
        equipment.chest = br.read_u32()?;
        equipment.arms = br.read_u32()?;
        equipment.legs = br.read_u32()?;
        equipment._0x4_2 = br.read_u32()?;

        for i in 0..4 { equipment.talismans[i] = br.read_u32()?; }
        for i in 0..6 { equipment._0x4_3[i] = br.read_u32()?; }

        Ok(equipment)
    }
}
impl Write for ProfileSummaryEquipmentItem {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());
        bytes.extend(self._0x8.to_le_bytes());
        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());
        bytes.extend(self._0x4_2.to_le_bytes());
        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        for i in 0..6 { bytes.extend(self._0x4_3[i].to_le_bytes()); }
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
    pub equipment_gaitem: ProfileSummaryEquipmentGaitem,
    pub equipment_item: ProfileSummaryEquipmentItem,
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
            equipment_gaitem: Default::default(),
            equipment_item: Default::default(),
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
        profile_summary.equipment_gaitem = ProfileSummaryEquipmentGaitem::read(br)?;
        profile_summary.equipment_item = ProfileSummaryEquipmentItem::read(br)?;
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
        bytes.extend(self.equipment_gaitem.write()?);
        bytes.extend(self.equipment_item.write()?);
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