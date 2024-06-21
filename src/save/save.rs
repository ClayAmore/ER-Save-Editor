pub mod save {
    use std::{fs, io, path::PathBuf};
    use binary_reader::BinaryReader;
    use crate::{
        read::read::Read, save::{
            common::{ save_slot::{EquipInventoryData, EquipProjectileData, GaItem, GaItemData, SaveSlot}, user_data_10::ProfileSummary, user_data_11::UserData11 },
            pc::pc_save::PCSave, 
            playstation::ps_save::PSSave, 
        }, util::bit::bit::set_bit, write::write::Write
    };

    // Using a checksum of the regulation bin file to check for Save Wizard .txt save file
    const REGULATION_MD5_CHECKSUM: [u8; 0x10] =[0x2E, 0x88, 0x1A, 0x15, 0xAC, 0x05, 0x88, 0x8D, 0xF2, 0xC2, 0x6A, 0xEC, 0xC2, 0x90, 0x89, 0x23];

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
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.user_data_10.steam_id = steam_id;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.user_data_10.steam_id = 0;
                },
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
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id
                }
                SaveType::PlayStation(_) => 0,
            }
        }

        pub fn set_character_steam_id(&mut self, index: usize, steam_id: u64) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id = steam_id;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].steam_id = 0;
                },
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
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.character_name.copy_from_slice(&character_name);
                    pc_save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.character_name.copy_from_slice(&character_name);
                    ps_save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
                },
            }
        }

        pub fn set_character_gender(&mut self, index: usize, gender: u8) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.gender = gender;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.gender = gender;
                },
            }
        }

        pub fn set_character_health(&mut self, index: usize, health: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.health = health;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.health = health;
                },
            }
        }

        pub fn set_character_base_max_health(&mut self, index: usize, base_max_health: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_health = base_max_health;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_health = base_max_health;
                },
            }
        }

        pub fn set_character_fp(&mut self, index: usize, fp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.fp = fp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.fp = fp;
                },
            }
        }

        pub fn set_character_base_max_fp(&mut self, index: usize, base_max_fp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_fp = base_max_fp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_fp = base_max_fp;
                },
            }
        }

        pub fn set_character_sp(&mut self, index: usize, sp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.sp = sp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.sp = sp;
                },
            }
        }

        pub fn set_character_base_max_sp(&mut self, index: usize, base_max_sp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_sp = base_max_sp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_sp = base_max_sp;
                },
            }
        }
        
        pub fn set_character_level(&mut self, index: usize, level: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.level = level;
                    pc_save.user_data_10.profile_summary[index].level = level;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.level = level;
                },
            }
        }

        pub fn set_character_vigor(&mut self, index: usize, vigor: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.vigor = vigor;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.vigor = vigor;
                },
            }
        }

        pub fn set_character_mind(&mut self, index: usize, mind: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.mind = mind;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.mind = mind;
                },
            }
        }

        pub fn set_character_endurance(&mut self, index: usize, endurance: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.endurance = endurance;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.endurance = endurance;
                },
            }
        }

        pub fn set_character_strength(&mut self, index: usize, strength: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.strength = strength;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.strength = strength;
                },
            }
        }

        pub fn set_character_dexterity(&mut self, index: usize, dexterity: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.dexterity = dexterity;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.dexterity = dexterity;
                },
            }
        }

        pub fn set_character_intelligence(&mut self, index: usize, intelligence: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.intelligence = intelligence;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.intelligence = intelligence;
                },
            }
        }

        pub fn set_character_faith(&mut self, index: usize, faith: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.faith = faith;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.faith = faith;
                },
            }
        }

        pub fn set_character_arcane(&mut self, index: usize, arcane: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.arcane = arcane;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.arcane = arcane;
                },
            }
        }

        pub fn set_character_souls(&mut self, index: usize, souls: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let orgininal_soulsmemory = pc_save.save_slots[index].save_slot.player_game_data.soulsmemory;
                    let orgininal_souls = pc_save.save_slots[index].save_slot.player_game_data.soulsmemory;
                    pc_save.save_slots[index].save_slot.player_game_data.souls = souls;
                    if souls > orgininal_souls {
                        pc_save.save_slots[index].save_slot.player_game_data.soulsmemory = orgininal_soulsmemory + souls;
                    }
                }
                SaveType::PlayStation(ps_save) => {
                    let orgininal_soulsmemory = ps_save.save_slots[index].player_game_data.soulsmemory;
                    let orgininal_souls = ps_save.save_slots[index].player_game_data.soulsmemory;
                    ps_save.save_slots[index].player_game_data.souls = souls;
                    if souls > orgininal_souls {
                        ps_save.save_slots[index].player_game_data.soulsmemory = orgininal_soulsmemory + souls;
                    }
                },
            }
        }

        pub fn set_character_event_flag(&mut self, index: usize, offset: usize, bit_pos: u8, state: bool) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let event_byte = pc_save.save_slots[index].save_slot.event_flags.flags[offset];
                    pc_save.save_slots[index].save_slot.event_flags.flags[offset] = set_bit(event_byte, bit_pos, state);
                }
                SaveType::PlayStation(ps_save) => {
                    let event_byte = ps_save.save_slots[index].event_flags.flags[offset];
                    ps_save.save_slots[index].event_flags.flags[offset] = set_bit(event_byte, bit_pos, state);
                },
            }
        }

        pub fn add_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
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
                SaveType::PlayStation(ps_save) => {
                    let res = ps_save.save_slots[index].regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {},
                        None => {
                            ps_save.save_slots[index].regions.unlocked_regions.push(region_id);
                            ps_save.save_slots[index].regions.unlocked_regions_count = ps_save.save_slots[index].regions.unlocked_regions_count + 1;
                        },
                    }
                },
            }
        }

        pub fn remove_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
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
                SaveType::PlayStation(ps_save) => {
                    let res = ps_save.save_slots[index].regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {
                            ps_save.save_slots[index].regions.unlocked_regions.swap_remove(i);
                            ps_save.save_slots[index].regions.unlocked_regions_count = ps_save.save_slots[index].regions.unlocked_regions_count - 1;
                        },
                        None => {},
                    }
                },
            }
        }

        pub fn get_profile_summary(&self, index: usize) -> ProfileSummary {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => pc_save.user_data_10.profile_summary[index],
                SaveType::PlayStation(ps_save) => ps_save.user_data_10.profile_summary[index],
            }
        }

        pub fn set_profile_summary(&mut self, index:usize, profile_summary: ProfileSummary) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.user_data_10.profile_summary[index] = profile_summary},
                SaveType::PlayStation(ps_save) => {ps_save.user_data_10.profile_summary[index] = profile_summary},
            }
        }

        pub fn get_slot(&self, index: usize) -> &SaveSlot {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.save_slots[index].save_slot,
                SaveType::PlayStation(ps_save) => &ps_save.save_slots[index],
            }
        }

        pub fn set_slot(&mut self, index:usize, save_slot: &SaveSlot) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.save_slots[index].save_slot = save_slot.clone()},
                SaveType::PlayStation(ps_save) => {ps_save.save_slots[index] = save_slot.clone()},
            }
        }
        
        pub fn get_user_data_11(&mut self) -> &UserData11{
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.user_data_11.user_data_11,
                SaveType::PlayStation(ps_save) => &ps_save.user_data_11,
            }
        }

        pub fn get_regulation(&self) -> &[u8]{
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.user_data_11.user_data_11.regulation,
                SaveType::PlayStation(ps_save) => &ps_save.user_data_11.regulation,
            }
        }

        pub fn set_gaitem_map(&mut self, index: usize, ga_items: Vec<GaItem>) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.save_slots[index].save_slot.ga_items = ga_items;}
                SaveType::PlayStation(ps_save) => {ps_save.save_slots[index].ga_items = ga_items;}
            }
        }

        pub fn set_held_inventory(&mut self, index: usize, held_inventory: EquipInventoryData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.equip_inventory_data = held_inventory;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].equip_inventory_data = held_inventory;
                }
            }
        }

        pub fn set_storage_box_inventory(&mut self, index: usize, storage_box_inventory: EquipInventoryData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.storage_inventory_data = storage_box_inventory;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].storage_inventory_data = storage_box_inventory;
                }
            }
        }

        pub fn set_gaitem_item_data(&mut self, index: usize, gaitem_data: GaItemData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.ga_item_data = gaitem_data;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].ga_item_data = gaitem_data;
                }
            }
        }

        pub fn set_quickslot_item(&mut self, slot_index: usize, quickslot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[slot_index].save_slot.equipped_items.quickitems[quickslot_index] = item_id;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.quick_slot_items[quickslot_index].item_id = gaitem_handle;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.quick_slot_items[quickslot_index].equipment_index = equip_index;
                },
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[slot_index].equipped_items.quickitems[quickslot_index] = item_id;
                    ps_save.save_slots[slot_index].equip_item_data.quick_slot_items[quickslot_index].item_id = gaitem_handle;
                    ps_save.save_slots[slot_index].equip_item_data.quick_slot_items[quickslot_index].equipment_index = equip_index;
                },
            }
        }

        pub fn set_pouch_item(&mut self, slot_index: usize, pouch_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[slot_index].save_slot.equipped_items.pouch[pouch_index] = item_id;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.pouch_items[pouch_index].item_id = gaitem_handle;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.pouch_items[pouch_index].equipment_index = equip_index;
                },
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[slot_index].equipped_items.pouch[pouch_index] = item_id;
                    ps_save.save_slots[slot_index].equip_item_data.pouch_items[pouch_index].item_id = gaitem_handle;
                    ps_save.save_slots[slot_index].equip_item_data.pouch_items[pouch_index].equipment_index = equip_index;
                },
            }
        }

        pub fn set_left_weapon_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.left_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.left_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.left_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.left_hand_armaments[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.left_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.left_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.left_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.left_hand_armaments[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_right_weapon_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.right_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.right_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.right_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.right_hand_armaments[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.right_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.right_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.right_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.right_hand_armaments[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_arrow_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arrows[weapon_slot_index] = equip_index;
                    slot.chr_asm.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.arrows[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.arrows[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.arrows[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arrows[weapon_slot_index] = equip_index;
                    slot.chr_asm.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.arrows[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.arrows[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.arrows[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_bolt_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.bolts[weapon_slot_index] = equip_index;
                    slot.chr_asm.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.bolts[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.bolts[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.bolts[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.bolts[weapon_slot_index] = equip_index;
                    slot.chr_asm.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.bolts[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.bolts[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.bolts[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_talisman_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.talismans[weapon_slot_index] = equip_index;
                    slot.chr_asm.talismans[weapon_slot_index] = item_id;
                    slot.chr_asm2.talismans[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.talismans[weapon_slot_index] = item_id | 0x20000000;
                    profile_summary.equipment_gaitem.talismans[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.talismans[weapon_slot_index] = item_id | 0x20000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.talismans[weapon_slot_index] = equip_index;
                    slot.chr_asm.talismans[weapon_slot_index] = item_id;
                    slot.chr_asm2.talismans[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.talismans[weapon_slot_index] = item_id | 0x20000000;
                    profile_summary.equipment_gaitem.talismans[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.talismans[weapon_slot_index] = item_id | 0x20000000;
                },
            }
        }
        
        pub fn set_head_gear(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.head = equip_index;
                    slot.chr_asm.head = item_id;
                    slot.chr_asm2.head = gaitem_handle;
                    slot.equipped_items.head = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.head = gaitem_handle;
                    profile_summary.equipment_item.head = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.head = equip_index;
                    slot.chr_asm.head = item_id;
                    slot.chr_asm2.head = gaitem_handle;
                    slot.equipped_items.head = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.head = gaitem_handle;
                    profile_summary.equipment_item.head = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_chest_piece(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.chest = equip_index;
                    slot.chr_asm.chest = item_id;
                    slot.chr_asm2.chest = gaitem_handle;
                    slot.equipped_items.chest = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.chest = gaitem_handle;
                    profile_summary.equipment_item.chest = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.chest = equip_index;
                    slot.chr_asm.chest = item_id;
                    slot.chr_asm2.chest = gaitem_handle;
                    slot.equipped_items.chest = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.chest = gaitem_handle;
                    profile_summary.equipment_item.chest = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_gauntlets(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arms = equip_index;
                    slot.chr_asm.arms = item_id;
                    slot.chr_asm2.arms = gaitem_handle;
                    slot.equipped_items.arms = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.arms = gaitem_handle;
                    profile_summary.equipment_item.arms = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arms = equip_index;
                    slot.chr_asm.arms = item_id;
                    slot.chr_asm2.arms = gaitem_handle;
                    slot.equipped_items.arms = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.arms = gaitem_handle;
                    profile_summary.equipment_item.arms = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_leggings(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.legs = equip_index;
                    slot.chr_asm.legs = item_id;
                    slot.chr_asm2.legs = gaitem_handle;
                    slot.equipped_items.legs = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.legs = gaitem_handle;
                    profile_summary.equipment_item.legs = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.legs = equip_index;
                    slot.chr_asm.legs = item_id;
                    slot.chr_asm2.legs = gaitem_handle;
                    slot.equipped_items.legs = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.legs = gaitem_handle;
                    profile_summary.equipment_item.legs = item_id | 0x10000000;
                },
            }
        }

        pub fn set_equip_projectile_data(&mut self, index: usize, projectile_list: EquipProjectileData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.equip_projectile_data = projectile_list.clone();
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].equip_projectile_data = projectile_list.clone();
                }
            }
        }

        pub fn set_match_making_wpn_lvl(&mut self, index: usize, weapon_level: u8) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.match_making_wpn_lvl = weapon_level;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.match_making_wpn_lvl = weapon_level;
                }
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
            br.jmp(0x1960070);
            let regulation = br.read_bytes(0x240010).expect("");
            let digest = md5::compute(regulation);
            let is_ps_save_wizard = digest == md5::Digest(REGULATION_MD5_CHECKSUM);
            br.jmp(0);
            is_ps_save_wizard
        }
    }
    
}

