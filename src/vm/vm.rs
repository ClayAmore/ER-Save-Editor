pub mod vm{
    use crate::{db::{bosses::bosses::BOSSES, colosseums::colosseums::COLOSSEUMS, cookbooks::books::COOKBOKS, event_flags::event_flags::EVENT_FLAGS, graces::maps::GRACES, maps::maps::MAPS, regions::regions::REGIONS, summoning_pools::summoning_pools::SUMMONING_POOLS, whetblades::whetblades::WHETBLADES}, save::save::save::Save, util::bit::bit::set_bit, vm::{profile_summary::slot_view_model::ProfileSummaryViewModel, slot::slot_view_model::SlotViewModel}};
    
    #[derive(Clone)]
    pub struct ViewModel {
        pub active: bool,
        pub index: usize, 
        pub steam_id: String,
        pub profile_summary: [ProfileSummaryViewModel; 0xA],
        pub slots: [SlotViewModel; 0xA]
    }

    impl Default for ViewModel{
        fn default() -> Self {
            Self {
                active: Default::default(),
                index: Default::default(),
                steam_id: Default::default(), 
                slots: Default::default(),
                profile_summary: Default::default()
            }
        }
    }

    impl ViewModel {
        pub fn from_save(save: &Save) -> Self {
            let mut vm = ViewModel::default();

            vm.active = true;

            // Steam Id
            vm.steam_id = save.user_data_10.steam_id.to_string();

            // Profile Summary and Save Slots
            for i in 0..0xA {
                if save.user_data_10.active_slot[i] {
                    vm.profile_summary[i] = ProfileSummaryViewModel::from_save(&save.user_data_10.profile_summary[i]);
                    vm.slots[i] = SlotViewModel::from_save(&save.save_slots[i])    
                }
            }

            vm
        }

        pub fn update_save(&self, save: &mut Save) {

            // Update SteamID for UserData10
            let steam_id =  self.steam_id.parse::<u64>().expect("");
            save.user_data_10.steam_id = steam_id;

            // Update data for each character
            for i in 0..0xA {
                if save.user_data_10.active_slot[i] {
                    // Update Steam ID
                    self.update_steam_id(save, i, steam_id);

                    // Update Stats 
                    self.update_stas(save, i);

                    // Update Character name
                    self.update_character_name(save, i);

                    // Update Events
                    self.update_events(save, i);

                    // Update Regions
                    self.update_regions(save, i);
                }
            }
            
        }

        fn update_steam_id(&self, save: &mut Save, index: usize, steam_id: u64) {
            save.save_slots[index].steam_id = steam_id;
        }

        fn update_character_name(&self, save: &mut Save, index: usize) {
            let mut character_name: [u16; 0x10] = [0; 0x10];
            let mut character_name2: [u16; 0x11] = [0; 0x11];

            for (i, char) in self.slots[index].general_vm.character_name.chars().enumerate() {
                character_name[i] = char as u16;
                character_name2[i] = char as u16;
            }
            save.save_slots[index].player_game_data.character_name.copy_from_slice(&character_name);
            save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
        }

        fn update_stas(&self, save: &mut Save, index: usize) {
            let player_game_data = &mut save.save_slots[index].player_game_data;
            let profile_summary = &mut save.user_data_10.profile_summary[index];
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

            player_game_data.level = level;
            player_game_data.vigor = stats_vm.vigor;
            player_game_data.mind = stats_vm.mind;
            player_game_data.endurance = stats_vm.endurance;
            player_game_data.strength = stats_vm.strength;
            player_game_data.dexterity = stats_vm.dexterity;
            player_game_data.intelligence = stats_vm.intelligence;
            player_game_data.faith = stats_vm.faith;
            player_game_data.arcane = stats_vm.arcane;

            profile_summary.level = level;

        }

        fn update_events(&self, save: &mut Save, index: usize) {
            // Graces
            for (grace, on) in self.slots[index].events_vm.graces.iter() {
                let grace_info = GRACES.lock().unwrap()[&grace];
                let offset = EVENT_FLAGS.lock().unwrap()[&grace_info.1];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
                
            }

            // Whetblades
            for (whetblade, on) in self.slots[index].events_vm.whetblades.iter() {
                let whetblade_info = WHETBLADES.lock().unwrap()[&whetblade];
                let offset = EVENT_FLAGS.lock().unwrap()[&whetblade_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }

            // Cookbooks
            for (cookbook, on) in self.slots[index].events_vm.cookbooks.iter() {
                let cookbook_info = COOKBOKS.lock().unwrap()[&cookbook];
                let offset = EVENT_FLAGS.lock().unwrap()[&cookbook_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }

            // Maps
            for (map, on) in self.slots[index].events_vm.maps.iter() {
                let map_info = MAPS.lock().unwrap()[&map];
                let offset = EVENT_FLAGS.lock().unwrap()[&map_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }

            // Bosses
            for (boss, on) in self.slots[index].events_vm.bosses.iter() {
                let boss_info = BOSSES.lock().unwrap()[&boss];
                let offset = EVENT_FLAGS.lock().unwrap()[&boss_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }

            // Summoning Pools
            for (summoning_pool, on) in self.slots[index].events_vm.summoning_pools.iter() {
                let summoning_pool_info = SUMMONING_POOLS.lock().unwrap()[&summoning_pool];
                let offset = EVENT_FLAGS.lock().unwrap()[&summoning_pool_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }

            // Colosseums
            for (colusseum, on) in self.slots[index].events_vm.colosseums.iter() {
                let colusseum_info = COLOSSEUMS.lock().unwrap()[&colusseum];
                let offset = EVENT_FLAGS.lock().unwrap()[&colusseum_info.0];
                let event_byte = save.save_slots[index].event_flags.flags[offset.0 as usize];
                save.save_slots[index].event_flags.flags[offset.0 as usize] = set_bit(event_byte, offset.1, *on);
            }
        }

        fn update_regions(&self, save: &mut Save, index: usize) {
            let save_slot = &mut save.save_slots[index];

            for (region, activated) in self.slots[index].regions_vm.regions.iter() {
                let region_id = REGIONS.lock().unwrap()[region].0;
                let res = save_slot.regions.unlocked_regions.iter().position(|r| *r == region_id);
                match res {
                    Some(index) => {
                        if !(*activated) {
                            save_slot.regions.unlocked_regions.swap_remove(index);
                            save_slot.regions.unlocked_regions_count = save_slot.regions.unlocked_regions_count - 1;
                        }
                    },
                    None => {
                        if *activated {
                            save_slot.regions.unlocked_regions.push(region_id);
                            save_slot.regions.unlocked_regions_count = save_slot.regions.unlocked_regions_count + 1;
                        }
                    },
                }
            }
        }

        fn _update_inventory(&self, _save: &mut Save, _index: usize) {
            // Increment IDX
        }

    }
}