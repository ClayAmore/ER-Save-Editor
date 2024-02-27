pub mod vm{
    use crate::{save::save::save::Save, vm::{profile_summary::slot_view_model::ProfileSummaryViewModel, slot::slot_view_model::SlotViewModel}};
    
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

        pub fn _update_save(&self, save: &mut Save) {
            // Update Steam ID
            self._update_steam_id(save);

            // Character name
            self._update_character_name(save);
            
        }

        fn _update_steam_id(&self, save: &mut Save) {
            let steam_id =  self.steam_id.parse::<u64>().expect("");
            save.user_data_10.steam_id = steam_id;
            for i in 0..0xA {
                if save.user_data_10.active_slot[i] {
                    save.save_slots[i].steam_id = steam_id;
                }
            }
        }

        fn _update_character_name(&self, save: &mut Save) {
            for i in 0..0xA {
                if save.user_data_10.active_slot[i] {
                    let mut character_name: [u16; 0x10] = [0; 0x10];
                    let mut character_name2: [u16; 0x11] = [0; 0x11];

                    for (j, char) in self.slots[i].general_vm.character_name.chars().enumerate() {
                        character_name[j] = char as u16;
                        character_name2[j] = char as u16;
                    }
                    save.save_slots[i].player_game_data.character_name.copy_from_slice(&character_name);
                    save.user_data_10.profile_summary[i].character_name.copy_from_slice(&character_name2);
                }
            }
        }
    }
}