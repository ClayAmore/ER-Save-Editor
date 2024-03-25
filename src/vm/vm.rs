pub mod vm {
    use std::collections::HashMap;

    use crate::{
        db::{
            bosses::bosses::BOSSES, 
            colosseums::colosseums::COLOSSEUMS, 
            cookbooks::books::COOKBOKS, 
            event_flags::event_flags::EVENT_FLAGS, 
            graces::maps::GRACES, maps::maps::MAPS, 
            regions::regions::REGIONS, 
            stats::stats::{FP, HP, SP}, 
            summoning_pools::summoning_pools::SUMMONING_POOLS, 
            whetblades::whetblades::WHETBLADES
        }, 
        save::{
            common::save_slot::{
                EquipInventoryData, 
                EquipInventoryItem
            }, 
            save::save::{
                Save, SaveType
            }}, util::{
                regulation::Regulation, 
                validator::validator::Validator
            }, vm::{
                inventory::{
                    InventoryGaitemType, InventoryItemType
                }, 
                profile_summary::slot_view_model::ProfileSummaryViewModel, 
                regulation::regulation_view_model::RegulationViewModel, 
                slot::slot_view_model::SlotViewModel
            }};
    
    
    #[derive(Clone)]
    pub struct ViewModel {
        pub active: Option<bool>,
        pub index: usize, 
        pub steam_id: String,
        pub profile_summary: [ProfileSummaryViewModel; 0xA],
        pub slots: [SlotViewModel; 0xA],
        pub regulation: RegulationViewModel,
    }

    impl Default for ViewModel{
        fn default() -> Self {
            Self {
                active: Default::default(),
                index: Default::default(),
                steam_id: Default::default(), 
                slots: Default::default(),
                profile_summary: Default::default(),
                regulation: Default::default(),
            }
        }
    }

    impl ViewModel {
        pub fn from_save(save: &Save) -> Self {
            let mut vm = ViewModel::default();

            // Init params
            Regulation::init_params(save);

            // Check for irregular data
            vm.active = Some(Validator::validate(save));
            if vm.active.is_some_and(|v| !v) {return vm;}

            // Steam Id
            vm.steam_id = save.save_type.get_global_steam_id().to_string();

            // Regulation (Params)
            vm.regulation = RegulationViewModel::default();

            // Get active characters
            for (index, active) in save.save_type.active_slots().iter().enumerate() {
                if *active {
                    vm.profile_summary[index] = ProfileSummaryViewModel::from_save(&save.save_type.get_profile_summary(index));
                    vm.slots[index] = SlotViewModel::from_save(&save.save_type.get_slot(index));
                }
            }

            vm
        }

        pub fn update_save(&self,  save_type: &mut SaveType) {
            let steam_id = self.steam_id.parse::<u64>().expect("");
            // Update SteamID for UserData10
            save_type.set_global_steam_id(steam_id);

            // Update data for each character
            for (i, active) in save_type.active_slots().iter().enumerate() {
                if *active {
                    // Update Steam ID
                    save_type.set_character_steam_id(i, steam_id);

                    // Update Character name
                    save_type.set_character_name(i, self.slots[i].general_vm.character_name.to_string());
                    
                    // Update Gender
                    save_type.set_character_gender(i, self.slots[i].general_vm.gender as u8);

                    // Update Character Weapon Match Making Level
                    self.update_weapon_match_making_level(save_type, i);

                    // Update Inventory (Held + Storage Box)
                    self.update_inventory(save_type, i);

                    // Update Stats 
                    self.update_stats(save_type, i);

                    // Update Equipment
                    self.update_equipment(save_type, i);

                    // Update Events
                    self.update_events(save_type, i);

                    // Update Regions
                    self.update_regions(save_type, i);
                }
            }
            
        }

        fn update_stats(&self, save_type: &mut SaveType, index: usize) {
            let stats_vm = &self.slots[index].stats_vm;

            let level = 
                stats_vm.vigor +
                stats_vm.mind +
                stats_vm.endurance +
                stats_vm.strength +
                stats_vm.dexterity +
                stats_vm.intelligence +
                stats_vm.faith +
                stats_vm.arcane - 79;

            save_type.set_character_health(index, HP[stats_vm.vigor as usize] as u32);
            save_type.set_character_base_max_health(index, HP[stats_vm.vigor as usize] as u32);

            save_type.set_character_fp(index, FP[stats_vm.mind as usize] as u32);
            save_type.set_character_base_max_fp(index, FP[stats_vm.mind as usize] as u32);

            save_type.set_character_sp(index, SP[stats_vm.endurance as usize] as u32);
            save_type.set_character_base_max_sp(index, SP[stats_vm.endurance as usize] as u32);

            save_type.set_character_level(index, level);
            save_type.set_character_vigor(index, stats_vm.vigor);
            save_type.set_character_mind(index, stats_vm.mind);
            save_type.set_character_endurance(index, stats_vm.endurance);
            save_type.set_character_strength(index, stats_vm.strength);
            save_type.set_character_dexterity(index, stats_vm.dexterity);
            save_type.set_character_intelligence(index, stats_vm.intelligence);
            save_type.set_character_faith(index, stats_vm.faith);
            save_type.set_character_arcane(index, stats_vm.arcane);

            save_type.set_character_souls(index, stats_vm.souls);
        }

        fn update_weapon_match_making_level(&self, save_type: &mut SaveType, index: usize) {
            let general_vm = &self.slots[index].general_vm;
            let inventory_vm = &self.slots[index].inventory_vm;

            // Don't update savefile equipment if it has not been changed
            if !inventory_vm.changed {
                return;
            }

            // Map somber to normal weapon upgrade
            let somber_to_normal: HashMap<u8, u8> = HashMap::from([
                (0, 0), (1, 0),(2, 5), (3, 7), (4, 10), (5, 12), 
                (6, 15), (7, 17), (8, 20), (9, 24), (10, 25), 
            ]);
            
            // Find the highest weapon upgrade in player inventory
            let mut max_level: u8 = 0;
            for (held_item, storage_item) in inventory_vm.storage[0].common_items.iter().zip(&inventory_vm.storage[1].common_items) {
                let held_weapon_res = Regulation::equip_weapon_params_map().get(&((&held_item.item_id/100)*100));
                let storage_weapon_res = Regulation::equip_weapon_params_map().get(&((&storage_item.item_id/100)*100));
                // Check held inventory item
                if held_item.r#type == InventoryGaitemType::WEAPON {
                    match held_weapon_res {
                        Some(weapon_param) => {
                            // Check if weapon is somber
                            let is_somber = weapon_param.data.reinforceTypeId != 0 && (
                                weapon_param.data.reinforceTypeId % 2200 == 0 ||
                                weapon_param.data.reinforceTypeId % 2400 == 0 ||
                                weapon_param.data.reinforceTypeId % 3200 == 0 ||
                                weapon_param.data.reinforceTypeId % 3300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8500 == 0
                            );
                            
                            // Extract weapon level based on wether weapon is somber or not
                            let weapon_level = if is_somber{
                                somber_to_normal[&((held_item.item_id % 100) as u8)]
                            }
                            else {
                                (held_item.item_id % 100) as u8
                            };

                            // Update max weapon level if inventory weapon is higher 
                            if weapon_level > max_level {
                                max_level = weapon_level;
                            }
                        },
                        None => {
                            println!("Couldn't find param info for weapon {}|{:#x}", held_item.item_id, held_item.item_id);
                        },
                    }
                }

                // Check storage box item
                if storage_item.r#type == InventoryGaitemType::WEAPON {
                    match storage_weapon_res {
                        Some(weapon_param) => {
                            // Check if weapon is somber
                            let is_somber = weapon_param.data.reinforceTypeId != 0 && (
                                weapon_param.data.reinforceTypeId % 2200 == 0 ||
                                weapon_param.data.reinforceTypeId % 2400 == 0 ||
                                weapon_param.data.reinforceTypeId % 3200 == 0 ||
                                weapon_param.data.reinforceTypeId % 3300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8500 == 0
                            );
                            
                            // Extract weapon level based on wether weapon is somber or not
                            let weapon_level = if is_somber{
                                somber_to_normal[&((storage_item.item_id % 100) as u8)]
                            }
                            else {
                                (storage_item.item_id % 100) as u8
                            };

                            // Update max weapon level if inventory weapon is higher 
                            if weapon_level > max_level {
                                max_level = weapon_level;
                            }
                        },
                        None => {
                            println!("Couldn't find param info for weapon {}|{:#x}", held_item.item_id, held_item.item_id);
                        },
                    }
                }
            }
            // Update player match making level highest weapon upgrade is higher
            if max_level > general_vm.weapon_level {
                save_type.set_match_making_wpn_lvl(index, max_level);
            }
        }

        fn update_equipment(&self, save_type: &mut SaveType, index: usize) {
            let equipment_vm = &self.slots[index].equipment_vm;

            // Don't update savefile equipment if it has not been changed
            if !equipment_vm.changed {
                return;
            }

            // (gaitem_handle, item_id, equipment_index)
            let mut quickslots = [(0, u32::MAX, u32::MAX); 10]; 
            let mut pouch_items = [(0, u32::MAX, u32::MAX); 6];
            
            // Left hand armament
            for (weapon_slot_index, left_hand_armament) in equipment_vm.left_hand_armaments.iter().enumerate() {
                save_type.set_left_weapon_slot(index, weapon_slot_index, left_hand_armament.gaitem_handle, left_hand_armament.id, left_hand_armament.equip_index);
            }

            // Right hand armament
            for (weapon_slot_index, right_hand_armament) in equipment_vm.right_hand_armaments.iter().enumerate() {
                save_type.set_right_weapon_slot(index, weapon_slot_index, right_hand_armament.gaitem_handle, right_hand_armament.id, right_hand_armament.equip_index);
            }

            // Arrows
            for (arrow_slot_index, arrow) in equipment_vm.arrows.iter().enumerate() {
                save_type.set_arrow_slot(index, arrow_slot_index, arrow.gaitem_handle, arrow.id, arrow.equip_index);
            }

            // Bolts
            for (bolt_slot_index, bolt) in equipment_vm.bolts.iter().enumerate() {
                save_type.set_bolt_slot(index, bolt_slot_index, bolt.gaitem_handle, bolt.id, bolt.equip_index);
            }
            
            save_type.set_head_gear(index, equipment_vm.head.gaitem_handle, equipment_vm.head.id, equipment_vm.head.equip_index);
            save_type.set_chest_piece(index, equipment_vm.chest.gaitem_handle, equipment_vm.chest.id, equipment_vm.chest.equip_index);
            save_type.set_gauntlets(index, equipment_vm.arms.gaitem_handle, equipment_vm.arms.id, equipment_vm.arms.equip_index);
            save_type.set_leggings(index, equipment_vm.legs.gaitem_handle, equipment_vm.legs.id, equipment_vm.legs.equip_index);

            // Talismans
            for (talisman_slot_index, talisman) in equipment_vm.talismans.iter().enumerate() {
                save_type.set_talisman_slot(index, talisman_slot_index, talisman.gaitem_handle, talisman.id, talisman.equip_index);
            }

            // Quickitem
            for (index, quickitem) in equipment_vm.quickitems.iter().enumerate() {
                if quickitem.id != 0 {
                    quickslots[index] = (quickitem.id | InventoryGaitemType::ITEM as u32, quickitem.id | InventoryItemType::ITEM as u32, quickitem.equip_index)
                }
            }
            for (quickslot_index, quickslot) in quickslots.iter().enumerate() {
                save_type.set_quickslot_item(index, quickslot_index, quickslot.0, quickslot.1, quickslot.2);
            }
            
            // Pouch
            for (index, pouch) in equipment_vm.pouch.iter().enumerate() {
                if pouch.id != 0 {
                    pouch_items[index] = (pouch.id | InventoryGaitemType::ITEM as u32, pouch.id | InventoryItemType::ITEM as u32, pouch.equip_index)
                }
            }
            for (pouch_index, pouch_item) in pouch_items.iter().enumerate() {
                save_type.set_pouch_item(index, pouch_index, pouch_item.0, pouch_item.1, pouch_item.2);
            }

        }

        fn update_events(&self, save_type: &mut SaveType, index: usize) {
            
            for (grace, on) in self.slots[index].events_vm.graces.iter() {
                let grace_info = GRACES.lock().unwrap()[&grace];
                let offset = EVENT_FLAGS.lock().unwrap()[&grace_info.1];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Whetblades
            for (whetblade, on) in self.slots[index].events_vm.whetblades.iter() {
                let whetblade_info = WHETBLADES.lock().unwrap()[&whetblade];
                let offset = EVENT_FLAGS.lock().unwrap()[&whetblade_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Cookbooks
            for (cookbook, on) in self.slots[index].events_vm.cookbooks.iter() {
                let cookbook_info = COOKBOKS.lock().unwrap()[&cookbook];
                let offset = EVENT_FLAGS.lock().unwrap()[&cookbook_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Maps
            for (map, on) in self.slots[index].events_vm.maps.iter() {
                let map_info = MAPS.lock().unwrap()[&map];
                let offset = EVENT_FLAGS.lock().unwrap()[&map_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Bosses
            for (boss, on) in self.slots[index].events_vm.bosses.iter() {
                let boss_info = BOSSES.lock().unwrap()[&boss];
                let offset = EVENT_FLAGS.lock().unwrap()[&boss_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Summoning Pools
            for (summoning_pool, on) in self.slots[index].events_vm.summoning_pools.iter() {
                let summoning_pool_info = SUMMONING_POOLS.lock().unwrap()[&summoning_pool];
                let offset = EVENT_FLAGS.lock().unwrap()[&summoning_pool_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }

            // Colosseums
            for (colusseum, on) in self.slots[index].events_vm.colosseums.iter() {
                let colusseum_info = COLOSSEUMS.lock().unwrap()[&colusseum];
                let offset = EVENT_FLAGS.lock().unwrap()[&colusseum_info.0];
                save_type.set_character_event_flag(index, offset.0 as usize, offset.1, *on);
            }
        }

        fn update_regions(&self, save_type: &mut SaveType, index: usize) {
            for (region, (activated, _, _,_)) in self.slots[index].regions_vm.regions.iter() {
                let region_id = REGIONS.lock().unwrap()[region].0;
                if *activated {
                    save_type.add_region(index, region_id);
                }
                else {
                    save_type.remove_region(index, region_id);
                }
            }
        }

        fn update_inventory(&self, save_type: &mut SaveType, index: usize) {
            let inventory_vm = &self.slots[index].inventory_vm;
            let inventory_held = &inventory_vm.storage[0];
            let inventory_storage_box = &inventory_vm.storage[1];

            // Don't update savefile equipment if it has not been changed
            if !inventory_vm.changed {
                return;
            }

            // Update gaitem map
            save_type.set_gaitem_map(index, inventory_vm.gaitem_map.clone());


            // Update projectile list
            save_type.set_equip_projectile_data(index, inventory_vm.projectile_list.clone());

            let mut counter = 0;
            // Update held inventory;
            let held_inventory = EquipInventoryData{
                common_inventory_items_distinct_count: inventory_held.common_item_count,
                common_items: inventory_held.common_items.iter().map(|item| {
                    EquipInventoryItem{
                        ga_item_handle: item.ga_item_handle,
                        inventory_index: item.inventory_index,
                        quantity: item.quantity
                    }
                }).collect::<Vec<EquipInventoryItem>>(),
                key_inventory_items_distinct_count: inventory_held.key_item_count,
                key_items: inventory_held.key_items.iter().map(|item| {
                    counter = counter + 1;
                    EquipInventoryItem{
                        ga_item_handle: item.ga_item_handle,
                        inventory_index: item.inventory_index,
                        quantity: item.quantity
                    }
                }).collect::<Vec<EquipInventoryItem>>(),
                next_acquisition_sort_id: inventory_held.next_acquisition_sort_order_index,
                next_equip_index: inventory_held.next_equip_index,
                ..Default::default()
            };
            save_type.set_held_inventory(index, held_inventory);
            
            // Update storage box inventory;
            let storage_box_inventory = EquipInventoryData{
                common_inventory_items_distinct_count: inventory_storage_box.common_item_count,
                common_items: inventory_storage_box.common_items.iter().map(|item| {
                    EquipInventoryItem{
                        ga_item_handle: item.ga_item_handle,
                        inventory_index: item.inventory_index,
                        quantity: item.quantity
                    }
                }).collect::<Vec<EquipInventoryItem>>(),
                key_inventory_items_distinct_count: inventory_storage_box.key_item_count,
                key_items: inventory_storage_box.key_items.iter().map(|item| {
                    EquipInventoryItem{
                        ga_item_handle: item.ga_item_handle,
                        inventory_index: item.inventory_index,
                        quantity: item.quantity
                    }
                }).collect::<Vec<EquipInventoryItem>>(),
                next_acquisition_sort_id: inventory_storage_box.next_acquisition_sort_order_index,
                next_equip_index: inventory_storage_box.next_equip_index,
                ..Default::default()
            };
            save_type.set_storage_box_inventory(index, storage_box_inventory);

            // Update gaitem item data
            let gaitem_data = inventory_vm.gaitem_data.clone();
            save_type.set_gaitem_item_data(index, gaitem_data);


        }
    }
}