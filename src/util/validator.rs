pub mod validator {
    use std::collections::{HashMap, HashSet};
    use crate::{save::{common::save_slot::{EquipInventoryItem, GaItem}, save::save::Save}, util::{param_structs::EQUIP_PARAM_GEM_ST, params::params::Row, regulation::Regulation}, vm::{inventory::{InventoryGaitemType, InventoryItemType}, regulation::regulation_view_model::{GoodsType, ProtectorCategory, WepType}}};

    pub struct Validator;

    impl Validator {
        pub fn validate(save: &Save) -> bool {
            // Get active characters
            for (index, active) in save.save_type.active_slots().iter().enumerate() {
                if *active {
                    if !Self::is_weapons_valid(save, index) {println!("is_weapons_valid"); return false; }
                    if !Self::is_items_valid(save, index) {println!("is_items_valid"); return false; }
                    if !Self::is_armor_valid(save, index) {println!("is_armor_valid"); return false; }
                    if !Self::is_physics_valid(save, index) {println!("is_physics_valid"); return false; }
                    if !Self::is_equipped_items_valid(save, index) {println!("is_equipped_items_valid"); return false; }
                }
            }
            true
        }

        fn is_weapons_valid(save: &Save, index: usize) -> bool {
            let slot = save.save_type.get_slot(index);
            let weapons = Regulation::equip_weapon_params_map();
            let gems = Regulation::equip_gem_param_map();

            // Map weapons
            let ga_item_weapons = &slot.ga_items.iter()
            .filter(|gaitem| gaitem.gaitem_handle.to_le_bytes()[3] == 0)
            .map(|g| g)
            .collect::<Vec<&GaItem>>();

            // Map gems
            let ga_item_gems = &slot.ga_items.iter()
            .filter(|gaitem| gaitem.gaitem_handle.to_le_bytes()[3] == 0xC0)
            .map(|g| (g.gaitem_handle, g))
            .collect::<HashMap<u32,&GaItem>>();

            for weapon_ga_item in ga_item_weapons {
                let res_weapon = weapons.get(&weapon_ga_item.item_id);

                if res_weapon.is_none() && weapon_ga_item.item_id != 0xFFFFFFFF {
                    return false;
                }

                // Get currently infused Ash Of War 
                let current_weapon_gem = weapon_ga_item.aow_gaitem_handle;

                // Skip rest of validation if there's no ash of war infused
                if current_weapon_gem == u32::MAX || current_weapon_gem == 0 {
                    continue;
                }

                // Look up the gem_param
                let gem_ga_item = ga_item_gems[&current_weapon_gem];
                let res_gem = gems.get(&gem_ga_item.item_id);

                // Check if gem_param exists, fail if it doesn't
                if res_gem.is_none() {
                    return false;
                }

                // Unwrap weapon and gem params
                let weapon_param = res_weapon.unwrap();
                let gem_param = res_gem.unwrap();

                // Ash of war on an item that doesn't support it. 
                if weapon_param.data.gemMountType == 0 {
                    return false;
                }

                // Ash of war not valid
                if !Self::validate_attached_gem(WepType::from(weapon_param.data.wepType), gem_param) {
                    return false
                }
            }

            true
        }

        fn is_items_valid(save: &Save, index: usize) -> bool {
            let inventory_common_items = &save.save_type.get_slot(index).equip_inventory_data.common_items;
            let storage_common_items = &save.save_type.get_slot(index).storage_inventory_data.common_items;
            Self::check_for_duplicate_items(inventory_common_items) && Self::check_for_duplicate_items(storage_common_items)
        }

        fn is_armor_valid(save: &Save, index: usize) -> bool{
            let head_protector_id = save.save_type.get_slot(index).chr_asm.head;
            let body_protector_id = save.save_type.get_slot(index).chr_asm.chest;
            let arms_protector_id = save.save_type.get_slot(index).chr_asm.arms;
            let legs_protector_id = save.save_type.get_slot(index).chr_asm.legs;

            if !Self::validate_armor_piece(head_protector_id, ProtectorCategory::Head) {return false;}
            if !Self::validate_armor_piece(body_protector_id, ProtectorCategory::Body) {return false;}
            if !Self::validate_armor_piece(arms_protector_id, ProtectorCategory::Arms) {return false;}
            if !Self::validate_armor_piece(legs_protector_id, ProtectorCategory::Legs) {return false;}

            let head_protector_id = save.save_type.get_slot(index).equipped_items.head ^ InventoryItemType::ARMOR as u32;
            let body_protector_id = save.save_type.get_slot(index).equipped_items.chest ^ InventoryItemType::ARMOR as u32;
            let arms_protector_id = save.save_type.get_slot(index).equipped_items.arms ^ InventoryItemType::ARMOR as u32;
            let legs_protector_id = save.save_type.get_slot(index).equipped_items.legs ^ InventoryItemType::ARMOR as u32;

            if !Self::validate_armor_piece(head_protector_id, ProtectorCategory::Head) {return false;}
            if !Self::validate_armor_piece(body_protector_id, ProtectorCategory::Body) {return false;}
            if !Self::validate_armor_piece(arms_protector_id, ProtectorCategory::Arms) {return false;}
            if !Self::validate_armor_piece(legs_protector_id, ProtectorCategory::Legs) {return false;}

            true
        }

        fn is_physics_valid(save: &Save, index: usize) -> bool {
            let physics_slot1 = save.save_type.get_slot(index).equip_physics_data.slot1;
            let physics_slot2 = save.save_type.get_slot(index).equip_physics_data.slot2;

            // Check if same tear is in the both slots
            if physics_slot1 != u32::MAX && physics_slot2 != u32::MAX && physics_slot1 == physics_slot2 { return false; }

            // Check if physic slot 1 is of type wondrous physics
            if physics_slot1 != u32::MAX {
                let res_physics1_good = Regulation::equip_goods_param_map().get(&(physics_slot1 ^ InventoryGaitemType::ITEM as u32));
                if res_physics1_good.is_some_and(|p| GoodsType::from(p.data.goodsType) != GoodsType::WonderousPhysicsTears) { return false; }
            }
            
            // Check if physic slot 2 is of type wondrous physics
            if physics_slot2 != u32::MAX {
                let res_physics2_good = Regulation::equip_goods_param_map().get(&(physics_slot2 ^ InventoryGaitemType::ITEM as u32));
                if res_physics2_good.is_some_and(|p| GoodsType::from(p.data.goodsType) != GoodsType::WonderousPhysicsTears) { return false; }
            }

            true
        }
        
        fn is_equipped_items_valid(save: &Save, index: usize) -> bool {
            let quick_slot_items = &save.save_type.get_slot(index).equip_item_data.quick_slot_items;
            let pouch_items = &save.save_type.get_slot(index).equip_item_data.pouch_items;

            // Check for invalid or duplicate quickslot items
            let mut item_ids = HashSet::new();
            for item in quick_slot_items.iter() {
                if item.item_id == 0 { continue; }
                if Regulation::equip_goods_param_map().get(&(item.item_id ^ InventoryGaitemType::ITEM as u32)).is_none() { println!("Item {} not found", item.item_id); return false; }
                if let Some(_existing_id) = item_ids.get(&item.item_id) {
                    println!("Duplicate item found: {}", _existing_id);
                    return false;
                } else {
                    item_ids.insert(item.item_id);
                }
            }
            
            // Check for invalid or duplicate pouch items
            let mut item_ids = HashSet::new();
            for item in pouch_items.iter() {
                if item.item_id == 0 { continue; }
                if Regulation::equip_goods_param_map().get(&(item.item_id ^ InventoryGaitemType::ITEM as u32)).is_none() { println!("Item {} not found", item.item_id); return false; }
                if let Some(_existing_id) = item_ids.get(&item.item_id) {
                    println!("Duplicate item found: {}", _existing_id);
                    return false;
                } else {
                    item_ids.insert(item.item_id);
                }
            }
            true
        }

        // region: utils

        fn validate_armor_piece(id: u32, protector_category: ProtectorCategory) -> bool {
            let res_armor_piece = Regulation::equip_protectors_param_map().get(&id);
            if  res_armor_piece.is_none() { return false; }
            let armor_piece = res_armor_piece.unwrap();
            let armor_piece_pc =  ProtectorCategory::try_from(armor_piece.data.protectorCategory);
            if armor_piece_pc.is_err() || armor_piece_pc.unwrap() != protector_category {return false;}
            true
        }

        // Validates the infsued gem against the weapon type by looking it up in the game params.
        fn validate_attached_gem(wep_type: WepType, gem_param: &Row<EQUIP_PARAM_GEM_ST>) -> bool {
            match wep_type {
                WepType::Dagger => gem_param.data.canMountWep_Dagger(),
                WepType::StraightSword => gem_param.data.canMountWep_SwordNormal(),
                WepType::Greatsword => gem_param.data.canMountWep_SwordLarge(),
                WepType::ColossalSword => gem_param.data.canMountWep_SwordGigantic(),
                WepType::CurvedSword => gem_param.data.canMountWep_SaberNormal(),
                WepType::CurvedGreatsword => gem_param.data.canMountWep_SaberLarge(),
                WepType::Katana => gem_param.data.canMountWep_katana(),
                WepType::Twinblade => gem_param.data.canMountWep_SwordDoubleEdge(),
                WepType::ThrustingSword => gem_param.data.canMountWep_SwordPierce(),
                WepType::HeavyThrustingSword => gem_param.data.canMountWep_RapierHeavy(),
                WepType::Axe => gem_param.data.canMountWep_AxeNormal(),
                WepType::Greataxe => gem_param.data.canMountWep_AxeLarge(),
                WepType::Hammer => gem_param.data.canMountWep_HammerNormal(),
                WepType::GreatHammer => gem_param.data.canMountWep_HammerLarge(),
                WepType::Flail => gem_param.data.canMountWep_Flail(),
                WepType::Spear => gem_param.data.canMountWep_SpearNormal(),
                WepType::HeavySpear => gem_param.data.canMountWep_SpearHeavy(),
                WepType::Halberd => gem_param.data.canMountWep_SpearAxe(),
                WepType::Scythe => gem_param.data.canMountWep_Sickle(),
                WepType::Fist => gem_param.data.canMountWep_Knuckle(),
                WepType::Claw => gem_param.data.canMountWep_Claw(),
                WepType::Whip => gem_param.data.canMountWep_Whip(),
                WepType::ColossalWeapon => gem_param.data.canMountWep_AxhammerLarge(),
                WepType::LightBow => gem_param.data.canMountWep_BowSmall(),
                WepType::Bow => gem_param.data.canMountWep_BowNormal(),
                WepType::Greatbow => gem_param.data.canMountWep_BowLarge(),
                WepType::Crossbow => gem_param.data.canMountWep_ClossBow(),
                WepType::Ballista => gem_param.data.canMountWep_Ballista(),
                WepType::Staff => gem_param.data.canMountWep_Staff(),
                WepType::Seal => gem_param.data.canMountWep_Talisman(),
                WepType::SmallShield => gem_param.data.canMountWep_ShieldSmall(),
                WepType::MediumShield => gem_param.data.canMountWep_ShieldNormal(),
                WepType::Greatshield => gem_param.data.canMountWep_ShieldLarge(),
                WepType::Torch => gem_param.data.canMountWep_Torch(),
                WepType::None |
                WepType::Arrow |
                WepType::Greatarrow |
                WepType::Bolt |
                WepType::BallistaBolt |
                WepType::Unknown => {
                    false
                },
            }
        }

        // Check if inventory_common_items only has EquipInventoryItem with unique ids
        fn check_for_duplicate_items(item_list: &Vec<EquipInventoryItem>) -> bool {
            let mut item_ids = HashSet::new();
            
            for item in item_list.iter().filter(|i| i.ga_item_handle.to_le_bytes()[3] == 0xB0) {
                if let Some(_existing_id) = item_ids.get(&item.ga_item_handle) {
                    return false;
                } else {
                    item_ids.insert(item.ga_item_handle);
                }
            }
            true
        }
        // endregion
    }
}