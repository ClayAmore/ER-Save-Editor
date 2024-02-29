pub mod general_view_model {
    use crate::{save::save::save::Save, vm::{slot::slot_view_model::SlotViewModel, vm::vm::ViewModel}};

    #[derive(Default, Clone)]
    pub struct Character {
        pub active: bool,
        pub index: usize,
        pub name: String,
    }


    pub struct ImporterViewModel  {
        pub selected_from_index: usize,
        pub selected_to_index: usize,
        pub from_list: [Character; 0x10],
        pub to_list: [Character; 0x10],
        from_save: Save,
    }

    impl Default for ImporterViewModel {
        fn default() -> Self {
            Self { 
                selected_from_index: 0,
                selected_to_index: 0,
                from_list: Default::default(), 
                to_list: Default::default(),
                from_save: Save::default()
            }
        }
    }

    impl ImporterViewModel {
        pub fn new(from: Save, vm: &ViewModel) -> Self {
            let mut importer_view_model = ImporterViewModel::default();

            importer_view_model.from_save = from;

            for (i, active) in importer_view_model.from_save.save_type.active_slots().iter().enumerate() {
                if *active {                    
                    // Character Name
                    let character_name = importer_view_model.from_save.save_type.get_slot(i).player_game_data.character_name;
                    let mut character_name_trimmed: [u16; 0x10] = [0;0x10];
                    for (i, char) in character_name.iter().enumerate() {
                        if *char == 0 { break; }
                        character_name_trimmed[i] = *char;
                    }
                    let character_name: String = String::from_utf16(&character_name_trimmed).expect("");
                    importer_view_model.from_list[i].active = true;
                    importer_view_model.from_list[i].index = i;
                    importer_view_model.from_list[i].name = character_name;
                }
            }

            for i in 0..0xA {
                if vm.profile_summary[i].active {                    
                    importer_view_model.to_list[i].active = true;
                    importer_view_model.to_list[i].index = i;
                    importer_view_model.to_list[i].name = vm.slots[i].general_vm.character_name.to_string();
                }
            }

            importer_view_model
        }

        pub fn import_character(&mut self, to_save: &mut Save, vm: &mut ViewModel) {
            // Save Slot
            to_save.save_type.set_slot(self.selected_to_index, self.from_save.save_type.get_slot(self.selected_from_index));
            
            // Profile Summary
            to_save.save_type.set_profile_summary(self.selected_to_index, self.from_save.save_type.get_profile_summary(self.selected_from_index));

            // Refresh view model
            vm.slots[self.selected_to_index] = SlotViewModel::from_save(to_save.save_type.get_slot(self.selected_to_index));
        }
    }
}