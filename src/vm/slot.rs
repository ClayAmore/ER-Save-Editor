pub mod slot_view_model {
    use crate::{save::save_slot::SaveSlot, vm::{events::events_view_model::EventsViewModel, general::general_view_model::GeneralViewModel, inventory::inventory::InventoryViewModel, stats::stats_view_model::StatsViewModel}};


    #[derive(Clone)]
    pub struct SlotViewModel {
        pub active: bool,
        pub general_vm : GeneralViewModel,
        pub stats_vm: StatsViewModel,
        pub inventory_vm: InventoryViewModel,
        pub events_vm: EventsViewModel,
    }

    impl Default for SlotViewModel {
        fn default() -> Self {
            Self { 
                active: Default::default(),
                general_vm: Default::default(),
                stats_vm: Default::default(),
                inventory_vm: Default::default(),
                events_vm: Default::default()
            }
        }
    }
    
    impl SlotViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            
            let active = true;
            
            let general_vm = GeneralViewModel::from_save(slot);
            let stats_vm = StatsViewModel::from_save(slot);
            let inventory_vm = InventoryViewModel::from_save(slot);
            let events_vm = EventsViewModel::from_save(slot);
             
            Self {
                active,
                general_vm,
                stats_vm,
                inventory_vm,
                events_vm
            }
        }
    }
}