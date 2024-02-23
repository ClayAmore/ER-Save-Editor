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
    }
}