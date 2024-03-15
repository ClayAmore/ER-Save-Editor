pub mod vm {
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
            whetblades::whetblades::WHETBLADES}, save::{common::save_slot::{EquipInventoryData, EquipInventoryItem, GaItem}, save::save::{Save, SaveType}}, util::validator::validator::Validator, vm::{profile_summary::slot_view_model::ProfileSummaryViewModel, regulation::regulation_view_model::RegulationViewModel, slot::slot_view_model::SlotViewModel}};
    
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
            
            vm.active = Some(Validator::new(&save).validate());
            if vm.active.is_some_and(|v| !v) {return vm;}

            // Steam Id
            vm.steam_id = save.save_type.get_global_steam_id().to_string();

            // Regulation (Params)
            vm.regulation = RegulationViewModel::from_save(save.save_type.get_regulation());

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

                    // Update Inventory (Held + Storage Box)
                    // self.update_inventory(save_type, i);

                    // Update Stats 
                    self.update_stats(save_type, i);

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
            for (region, activated) in self.slots[index].regions_vm.regions.iter() {
                let region_id = REGIONS.lock().unwrap()[region].0;
                if *activated {
                    save_type.add_region(index, region_id);
                }
                else {
                    save_type.remove_region(index, region_id);
                }
            }
        }

        fn _wip_update_inventory(&self, save_type: &mut SaveType, index: usize) {
            // Gaitem Map
            let mut new_gaitem_list = [GaItem::default(); 0x1400];
            let mut new_gaitems = self.slots[index].inventory_vm.gaitem_map.values().collect::<Vec<&GaItem>>();
            new_gaitems.sort_by(|a,b| b.gaitem_handle.cmp(&a.gaitem_handle));
            for (i, gaitem) in new_gaitems.iter().enumerate() { new_gaitem_list[i] = **gaitem; }
            save_type.set_gaitem_map(index, new_gaitem_list.to_vec());

            // Held items
            let mut new_inventory = [EquipInventoryItem::default();0xA80];
            let mut new_inventory_key_items = [EquipInventoryItem::default();0x180];
            
            let mut i = 0;
            for weapon in &self.slots[index].inventory_vm.storage[0].weapons {
                new_inventory[i] = EquipInventoryItem{
                    ga_item_handle: weapon.ga_item_handle,
                    inventory_index: weapon.inventory_index,
                    quantity: weapon.quantity,
                };
                i = i+1;
            }
            for armor in &self.slots[index].inventory_vm.storage[0].armors {
                new_inventory[i] = EquipInventoryItem{
                    ga_item_handle: armor.ga_item_handle,
                    inventory_index: armor.inventory_index,
                    quantity: armor.quantity,
                };
                i = i+1;
            }
            for accessory in &self.slots[index].inventory_vm.storage[0].accessories {
                new_inventory[i] = EquipInventoryItem{
                    ga_item_handle: accessory.ga_item_handle,
                    inventory_index: accessory.inventory_index,
                    quantity: accessory.quantity,
                };
                i = i+1;
            }
            for common_item in &self.slots[index].inventory_vm.storage[0].common_items {
                new_inventory[i] = EquipInventoryItem{
                    ga_item_handle: common_item.ga_item_handle,
                    inventory_index: common_item.inventory_index,
                    quantity: common_item.quantity,
                };
                i = i+1;
            }
            for aow in &self.slots[index].inventory_vm.storage[0].aows {
                new_inventory[i] = EquipInventoryItem{
                    ga_item_handle: aow.ga_item_handle,
                    inventory_index: aow.inventory_index,
                    quantity: aow.quantity,
                };
                i = i+1;
            }

            
            let mut i = 0;
            for key_item in &self.slots[index].inventory_vm.storage[0].key_items {
                new_inventory_key_items[i] = EquipInventoryItem{
                    ga_item_handle: key_item.ga_item_handle,
                    inventory_index: key_item.inventory_index,
                    quantity: key_item.quantity,
                };
                i = i+1;
            }

            // Storage box items
            let mut new_storage_box_common_items = [EquipInventoryItem::default();0xA80];
            let mut new_storage_box_key_items = [EquipInventoryItem::default();0x180];
            
            
            let mut i = 0;
            for weapon in &self.slots[index].inventory_vm.storage[1].weapons {
                new_storage_box_common_items[i] = EquipInventoryItem{
                    ga_item_handle: weapon.ga_item_handle,
                    inventory_index: weapon.inventory_index,
                    quantity: weapon.quantity,
                };
                i = i+1;
            }
            for armor in &self.slots[index].inventory_vm.storage[1].armors {
                new_storage_box_common_items[i] = EquipInventoryItem{
                    ga_item_handle: armor.ga_item_handle,
                    inventory_index: armor.inventory_index,
                    quantity: armor.quantity,
                };
                i = i+1;
            }
            for accessory in &self.slots[index].inventory_vm.storage[1].accessories {
                new_storage_box_common_items[i] = EquipInventoryItem{
                    ga_item_handle: accessory.ga_item_handle,
                    inventory_index: accessory.inventory_index,
                    quantity: accessory.quantity,
                };
                i = i+1;
            }
            for common_item in &self.slots[index].inventory_vm.storage[1].common_items {
                new_storage_box_common_items[i] = EquipInventoryItem{
                    ga_item_handle: common_item.ga_item_handle,
                    inventory_index: common_item.inventory_index,
                    quantity: common_item.quantity,
                };
                i = i+1;
            }
            for aow in &self.slots[index].inventory_vm.storage[1].aows {
                new_storage_box_common_items[i] = EquipInventoryItem{
                    ga_item_handle: aow.ga_item_handle,
                    inventory_index: aow.inventory_index,
                    quantity: aow.quantity,
                };
                i = i+1;
            }

            let mut i = 0;
            for key_item in &self.slots[index].inventory_vm.storage[1].key_items {
                new_storage_box_key_items[i] = EquipInventoryItem{
                    ga_item_handle: key_item.ga_item_handle,
                    inventory_index: key_item.inventory_index,
                    quantity: key_item.quantity,
                };
                i = i+1;
            }

            save_type.set_equip_inventory(index, [
               EquipInventoryData {
                common_inventory_items_distinct_count: self.slots[index].inventory_vm.storage[0].common_item_count,
                key_inventory_items_distinct_count: self.slots[index].inventory_vm.storage[0].key_item_count,
                common_items: new_inventory.to_vec(),
                key_items: new_inventory_key_items.to_vec(),
                next_acquisition_sort_id: self.slots[index].inventory_vm.storage[0].next_acquisition_sort_order_index,
                _0x4: save_type.get_slot(index).equip_inventory_data._0x4
               },
               EquipInventoryData {
                common_inventory_items_distinct_count: self.slots[index].inventory_vm.storage[1].common_item_count,
                key_inventory_items_distinct_count: self.slots[index].inventory_vm.storage[1].key_item_count,
                common_items: new_inventory.to_vec(),
                key_items: new_inventory_key_items.to_vec(),
                next_acquisition_sort_id: self.slots[index].inventory_vm.storage[1].next_acquisition_sort_order_index,
                _0x4: save_type.get_slot(index).storage_inventory_data._0x4
               }
            ]);

            // Update Gaitem
        }
    }
}