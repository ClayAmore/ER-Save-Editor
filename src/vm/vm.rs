use er_save_lib::{SaveApi, SaveApiError};

use crate::{
    db::{
        bosses::Boss,
        colosseums::Colosseum,
        cookbooks::Cookbook,
        graces::maps::Grace,
        maps::Map,
        regions::Region,
        stats::{FP, HP, SP},
        summoning_pools::SummoningPool,
        whetblades::Whetblade,
    },
    vm::character::CharacterViewModel,
};

#[derive(Default)]
pub struct ViewModel {
    pub index: usize,
    pub dlc: bool, // Flag for activating DLC items, locations, etc..
    pub steam_id: String,
    pub characters: [CharacterViewModel; 10],
    // pub regulation: RegulationViewModel,
}

impl ViewModel {
    pub fn from_save(save_api: &SaveApi) -> Result<Self, SaveApiError> {
        let mut vm = ViewModel::default();

        // Steam Id
        vm.steam_id = save_api.steam_id().to_string();

        // Characters
        for (i, active) in save_api.active_characters().iter().enumerate() {
            if *active {
                vm.characters[i] = CharacterViewModel::from_save(save_api, i)?;
            }
        }

        Ok(vm)
    }

    pub fn update_save(&self, save_api: &mut SaveApi) -> Result<(), SaveApiError> {
        let steam_id = self.steam_id.parse::<u64>().expect("");

        // Update SteamID
        save_api.set_steam_id(steam_id)?;

        // Update data for each character
        for (i, active) in save_api.active_characters().iter().enumerate() {
            if *active {
                // Update Character name
                save_api.set_character_name(i, &self.characters[i].general_vm.character_name)?;

                // Update Gender
                save_api.set_gender(i, self.characters[i].general_vm.gender as u8)?;

                // // Update Character Weapon Match Making Level
                // self.update_weapon_match_making_level(save_type, i);

                // // Update Inventory (Held + Storage Box)
                // self.update_inventory(save_type, i);

                // Update Stats
                self.update_stats(save_api, i)?;

                // // Update Equipment
                // self.update_equipment(save_type, i);

                // Update Events
                self.update_events(save_api, i)?;

                // // Update Regions
                self.update_regions(save_api, i)?;
            }
        }
        Ok(())
    }

    fn update_stats(&self, save_api: &mut SaveApi, index: usize) -> Result<(), SaveApiError> {
        let stats_vm = &self.characters[index].stats_vm;

        // Determine is the stats has changed
        let has_changed = stats_vm.vigor != save_api.vigor(index)
            || stats_vm.mind != save_api.mind(index)
            || stats_vm.endurance != save_api.endurance(index)
            || stats_vm.strength != save_api.strength(index)
            || stats_vm.dexterity != save_api.dexterity(index)
            || stats_vm.intelligence != save_api.intelligence(index)
            || stats_vm.faith != save_api.faith(index)
            || stats_vm.arcane != save_api.arcane(index)
            || stats_vm.runes != save_api.runes(index);

        // Exit early if no stats have been changed
        if !has_changed {
            return Ok(());
        }

        // Calculate the level from stats
        let level = stats_vm.vigor
            + stats_vm.mind
            + stats_vm.endurance
            + stats_vm.strength
            + stats_vm.dexterity
            + stats_vm.intelligence
            + stats_vm.faith
            + stats_vm.arcane
            - 79;

        // Update hp from vigor
        save_api.set_hp(index, HP[stats_vm.vigor as usize] as u32)?;
        save_api.set_base_max_hp(index, HP[stats_vm.vigor as usize] as u32)?;

        // Update fp from mind
        save_api.set_fp(index, FP[stats_vm.mind as usize] as u32)?;
        save_api.set_base_max_fp(index, FP[stats_vm.mind as usize] as u32)?;

        // Update sp from endurance
        save_api.set_sp(index, SP[stats_vm.endurance as usize] as u32)?;
        save_api.set_base_max_sp(index, SP[stats_vm.endurance as usize] as u32)?;

        // Update the stats
        save_api.set_level(index, level)?;
        save_api.set_vigor(index, stats_vm.vigor)?;
        save_api.set_mind(index, stats_vm.mind)?;
        save_api.set_endurance(index, stats_vm.endurance)?;
        save_api.set_strength(index, stats_vm.strength)?;
        save_api.set_dexterity(index, stats_vm.dexterity)?;
        save_api.set_intelligence(index, stats_vm.intelligence)?;
        save_api.set_faith(index, stats_vm.faith)?;
        save_api.set_arcane(index, stats_vm.arcane)?;

        // Update runes
        save_api.set_runes(index, stats_vm.runes)?;

        // Max out runes memory to avoid ban for stat changing
        save_api.set_runes_memory(index, u32::MAX)?;

        Ok(())
    }

    // fn update_weapon_match_making_level(&self, save_type: &mut SaveType, index: usize) {
    //     let general_vm = &self.slots[index].general_vm;
    //     let inventory_vm = &self.slots[index].inventory_vm;

    //     // Don't update savefile equipment if it has not been changed
    //     if !inventory_vm.changed {
    //         return;
    //     }

    //     // Map somber to normal weapon upgrade
    //     let somber_to_normal: HashMap<u8, u8> = HashMap::from([
    //         (0, 0),
    //         (1, 0),
    //         (2, 5),
    //         (3, 7),
    //         (4, 10),
    //         (5, 12),
    //         (6, 15),
    //         (7, 17),
    //         (8, 20),
    //         (9, 24),
    //         (10, 25),
    //     ]);

    //     // Find the highest weapon upgrade in player inventory
    //     let mut max_level: u8 = 0;
    //     for (held_item, storage_item) in inventory_vm.storage[0]
    //         .common_items
    //         .iter()
    //         .zip(&inventory_vm.storage[1].common_items)
    //     {
    //         let held_weapon_res =
    //             Regulation::equip_weapon_params_map().get(&((&held_item.item_id / 100) * 100));
    //         let storage_weapon_res = Regulation::equip_weapon_params_map()
    //             .get(&((&storage_item.item_id / 100) * 100));
    //         // Check held inventory item
    //         if held_item.r#type == InventoryGaitemType::WEAPON {
    //             match held_weapon_res {
    //                 Some(weapon_param) => {
    //                     // Check if weapon is somber
    //                     let is_somber = weapon_param.data.reinforceTypeId != 0
    //                         && (weapon_param.data.reinforceTypeId % 2200 == 0
    //                             || weapon_param.data.reinforceTypeId % 2400 == 0
    //                             || weapon_param.data.reinforceTypeId % 3200 == 0
    //                             || weapon_param.data.reinforceTypeId % 3300 == 0
    //                             || weapon_param.data.reinforceTypeId % 8300 == 0
    //                             || weapon_param.data.reinforceTypeId % 8500 == 0);

    //                     // Extract weapon level based on wether weapon is somber or not
    //                     let weapon_level = if is_somber {
    //                         somber_to_normal[&((held_item.item_id % 100) as u8)]
    //                     } else {
    //                         (held_item.item_id % 100) as u8
    //                     };

    //                     // Update max weapon level if inventory weapon is higher
    //                     if weapon_level > max_level {
    //                         max_level = weapon_level;
    //                     }
    //                 }
    //                 None => {
    //                     println!(
    //                         "Couldn't find param info for weapon {}|{:#x}",
    //                         held_item.item_id, held_item.item_id
    //                     );
    //                 }
    //             }
    //         }

    //         // Check storage box item
    //         if storage_item.r#type == InventoryGaitemType::WEAPON {
    //             match storage_weapon_res {
    //                 Some(weapon_param) => {
    //                     // Check if weapon is somber
    //                     let is_somber = weapon_param.data.reinforceTypeId != 0
    //                         && (weapon_param.data.reinforceTypeId % 2200 == 0
    //                             || weapon_param.data.reinforceTypeId % 2400 == 0
    //                             || weapon_param.data.reinforceTypeId % 3200 == 0
    //                             || weapon_param.data.reinforceTypeId % 3300 == 0
    //                             || weapon_param.data.reinforceTypeId % 8300 == 0
    //                             || weapon_param.data.reinforceTypeId % 8500 == 0);

    //                     // Extract weapon level based on wether weapon is somber or not
    //                     let weapon_level = if is_somber {
    //                         somber_to_normal[&((storage_item.item_id % 100) as u8)]
    //                     } else {
    //                         (storage_item.item_id % 100) as u8
    //                     };

    //                     // Update max weapon level if inventory weapon is higher
    //                     if weapon_level > max_level {
    //                         max_level = weapon_level;
    //                     }
    //                 }
    //                 None => {
    //                     println!(
    //                         "Couldn't find param info for weapon {}|{:#x}",
    //                         held_item.item_id, held_item.item_id
    //                     );
    //                 }
    //             }
    //         }
    //     }
    //     // Update player match making level highest weapon upgrade is higher
    //     if max_level > general_vm.weapon_level {
    //         save_type.set_match_making_wpn_lvl(index, max_level);
    //     }
    // }

    // fn update_equipment(&self, save_type: &mut SaveType, index: usize) {
    //     let equipment_vm = &self.slots[index].equipment_vm;

    //     // Don't update savefile equipment if it has not been changed
    //     if !equipment_vm.changed {
    //         return;
    //     }

    //     // (gaitem_handle, item_id, equipment_index)
    //     let mut quickslots = [(0, u32::MAX, u32::MAX); 10];
    //     let mut pouch_items = [(0, u32::MAX, u32::MAX); 6];

    //     // Left hand armament
    //     for (weapon_slot_index, left_hand_armament) in
    //         equipment_vm.left_hand_armaments.iter().enumerate()
    //     {
    //         save_type.set_left_weapon_slot(
    //             index,
    //             weapon_slot_index,
    //             left_hand_armament.gaitem_handle,
    //             left_hand_armament.id,
    //             left_hand_armament.equip_index,
    //         );
    //     }

    //     // Right hand armament
    //     for (weapon_slot_index, right_hand_armament) in
    //         equipment_vm.right_hand_armaments.iter().enumerate()
    //     {
    //         save_type.set_right_weapon_slot(
    //             index,
    //             weapon_slot_index,
    //             right_hand_armament.gaitem_handle,
    //             right_hand_armament.id,
    //             right_hand_armament.equip_index,
    //         );
    //     }

    //     // Arrows
    //     for (arrow_slot_index, arrow) in equipment_vm.arrows.iter().enumerate() {
    //         save_type.set_arrow_slot(
    //             index,
    //             arrow_slot_index,
    //             arrow.gaitem_handle,
    //             arrow.id,
    //             arrow.equip_index,
    //         );
    //     }

    //     // Bolts
    //     for (bolt_slot_index, bolt) in equipment_vm.bolts.iter().enumerate() {
    //         save_type.set_bolt_slot(
    //             index,
    //             bolt_slot_index,
    //             bolt.gaitem_handle,
    //             bolt.id,
    //             bolt.equip_index,
    //         );
    //     }

    //     save_type.set_head_gear(
    //         index,
    //         equipment_vm.head.gaitem_handle,
    //         equipment_vm.head.id,
    //         equipment_vm.head.equip_index,
    //     );
    //     save_type.set_chest_piece(
    //         index,
    //         equipment_vm.chest.gaitem_handle,
    //         equipment_vm.chest.id,
    //         equipment_vm.chest.equip_index,
    //     );
    //     save_type.set_gauntlets(
    //         index,
    //         equipment_vm.arms.gaitem_handle,
    //         equipment_vm.arms.id,
    //         equipment_vm.arms.equip_index,
    //     );
    //     save_type.set_leggings(
    //         index,
    //         equipment_vm.legs.gaitem_handle,
    //         equipment_vm.legs.id,
    //         equipment_vm.legs.equip_index,
    //     );

    //     // Talismans
    //     for (talisman_slot_index, talisman) in equipment_vm.talismans.iter().enumerate() {
    //         save_type.set_talisman_slot(
    //             index,
    //             talisman_slot_index,
    //             talisman.gaitem_handle,
    //             talisman.id,
    //             talisman.equip_index,
    //         );
    //     }

    //     // Quickitem
    //     for (index, quickitem) in equipment_vm.quickitems.iter().enumerate() {
    //         if quickitem.id != 0 {
    //             quickslots[index] = (
    //                 quickitem.id | InventoryGaitemType::ITEM as u32,
    //                 quickitem.id | InventoryItemType::ITEM as u32,
    //                 quickitem.equip_index,
    //             )
    //         }
    //     }
    //     for (quickslot_index, quickslot) in quickslots.iter().enumerate() {
    //         save_type.set_quickslot_item(
    //             index,
    //             quickslot_index,
    //             quickslot.0,
    //             quickslot.1,
    //             quickslot.2,
    //         );
    //     }

    //     // Pouch
    //     for (index, pouch) in equipment_vm.pouch.iter().enumerate() {
    //         if pouch.id != 0 {
    //             pouch_items[index] = (
    //                 pouch.id | InventoryGaitemType::ITEM as u32,
    //                 pouch.id | InventoryItemType::ITEM as u32,
    //                 pouch.equip_index,
    //             )
    //         }
    //     }
    //     for (pouch_index, pouch_item) in pouch_items.iter().enumerate() {
    //         save_type.set_pouch_item(
    //             index,
    //             pouch_index,
    //             pouch_item.0,
    //             pouch_item.1,
    //             pouch_item.2,
    //         );
    //     }
    // }

    fn update_events(&self, save_api: &mut SaveApi, index: usize) -> Result<(), SaveApiError> {
        // Graces
        for (grace, on) in self.characters[index].events_vm.graces.iter() {
            if let Some((_, event_id, _)) = Grace::graces().get(grace) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Whetblades
        for (whetblade, on) in self.characters[index].events_vm.whetblades.iter() {
            if let Some((event_id, _)) = Whetblade::whetblades().get(&whetblade) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Cookbooks
        for (cookbook, on) in self.characters[index].events_vm.cookbooks.iter() {
            if let Some((event_id, _)) = Cookbook::cookbooks().get(&cookbook) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Maps
        for (map, on) in self.characters[index].events_vm.maps.iter() {
            if let Some((event_id, _)) = Map::maps().get(&map) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Bosses
        for (boss, on) in self.characters[index].events_vm.bosses.iter() {
            if let Some((event_id, _)) = Boss::bosses().get(&boss) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Summoning Pools
        for (summoning_pool, on) in self.characters[index].events_vm.summoning_pools.iter() {
            if let Some((event_id, _)) = SummoningPool::summoning_pools().get(&summoning_pool) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }

        // Colosseums
        for (colusseum, on) in self.characters[index].events_vm.colosseums.iter() {
            if let Some((event_id, _)) = Colosseum::colusseums().get(&colusseum) {
                save_api.set_event_flag(*event_id, index, *on)?;
            }
        }
        Ok(())
    }

    fn update_regions(&self, save_api: &mut SaveApi, index: usize) -> Result<(), SaveApiError> {
        for (region, (is_on, _, _, _)) in self.characters[index].regions_vm.regions.iter() {
            if let Some((region_id, _, _, _, _, _)) = Region::regions().get(region) {
                if *is_on {
                    save_api.add_region(index, *region_id)?;
                } else {
                    save_api.remove_region(index, *region_id)?;
                }
            }
        }
        Ok(())
    }

    // fn update_inventory(&self, save_type: &mut SaveType, index: usize) {
    //     let inventory_vm = &self.slots[index].inventory_vm;
    //     let inventory_held = &inventory_vm.storage[0];
    //     let inventory_storage_box = &inventory_vm.storage[1];

    //     // Don't update savefile equipment if it has not been changed
    //     if !inventory_vm.changed {
    //         return;
    //     }

    //     // Update gaitem map
    //     save_type.set_gaitem_map(index, inventory_vm.gaitem_map.clone());

    //     // Update projectile list
    //     save_type.set_equip_projectile_data(index, inventory_vm.projectile_list.clone());

    //     let mut counter = 0;
    //     // Update held inventory;
    //     let held_inventory = EquipInventoryData {
    //         common_inventory_items_distinct_count: inventory_held.common_item_count,
    //         common_items: inventory_held
    //             .common_items
    //             .iter()
    //             .map(|item| EquipInventoryItem {
    //                 ga_item_handle: item.ga_item_handle,
    //                 inventory_index: item.inventory_index,
    //                 quantity: item.quantity,
    //             })
    //             .collect::<Vec<EquipInventoryItem>>(),
    //         key_inventory_items_distinct_count: inventory_held.key_item_count,
    //         key_items: inventory_held
    //             .key_items
    //             .iter()
    //             .map(|item| {
    //                 counter = counter + 1;
    //                 EquipInventoryItem {
    //                     ga_item_handle: item.ga_item_handle,
    //                     inventory_index: item.inventory_index,
    //                     quantity: item.quantity,
    //                 }
    //             })
    //             .collect::<Vec<EquipInventoryItem>>(),
    //         next_acquisition_sort_id: inventory_held.next_acquisition_sort_order_index,
    //         next_equip_index: inventory_held.next_equip_index,
    //         ..Default::default()
    //     };
    //     save_type.set_held_inventory(index, held_inventory);

    //     // Update storage box inventory;
    //     let storage_box_inventory = EquipInventoryData {
    //         common_inventory_items_distinct_count: inventory_storage_box.common_item_count,
    //         common_items: inventory_storage_box
    //             .common_items
    //             .iter()
    //             .map(|item| EquipInventoryItem {
    //                 ga_item_handle: item.ga_item_handle,
    //                 inventory_index: item.inventory_index,
    //                 quantity: item.quantity,
    //             })
    //             .collect::<Vec<EquipInventoryItem>>(),
    //         key_inventory_items_distinct_count: inventory_storage_box.key_item_count,
    //         key_items: inventory_storage_box
    //             .key_items
    //             .iter()
    //             .map(|item| EquipInventoryItem {
    //                 ga_item_handle: item.ga_item_handle,
    //                 inventory_index: item.inventory_index,
    //                 quantity: item.quantity,
    //             })
    //             .collect::<Vec<EquipInventoryItem>>(),
    //         next_acquisition_sort_id: inventory_storage_box.next_acquisition_sort_order_index,
    //         next_equip_index: inventory_storage_box.next_equip_index,
    //         ..Default::default()
    //     };
    //     save_type.set_storage_box_inventory(index, storage_box_inventory);

    //     // Update gaitem item data
    //     let gaitem_data = inventory_vm.gaitem_data.clone();
    //     save_type.set_gaitem_item_data(index, gaitem_data);
    // }
}
