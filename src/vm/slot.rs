pub mod slot_view_model {
    use crate::{save::common::save_slot::SaveSlot, vm::{equipment::equipment_view_model::EquipmentViewModel, events::events_view_model::EventsViewModel, general::general_view_model::GeneralViewModel, inventory::InventoryViewModel, regions::regions_view_model::RegionsViewModel, stats::stats_view_model::StatsViewModel}};


    #[derive(Default, Clone)]
    pub struct SlotViewModel {
        pub active: bool,
        pub general_vm : GeneralViewModel,
        pub stats_vm: StatsViewModel,
        pub equipment_vm: EquipmentViewModel,
        pub inventory_vm: InventoryViewModel,
        pub events_vm: EventsViewModel,
        pub regions_vm: RegionsViewModel,
    }
    
    impl SlotViewModel {
        pub fn from_save(slot:&SaveSlot) -> Self {
            let active = true;
            
            let general_vm = GeneralViewModel::from_save(slot);
            let stats_vm = StatsViewModel::from_save(slot);
            let equipment_vm = EquipmentViewModel::from_save(slot);
            let inventory_vm = InventoryViewModel::from_save(slot);
            let events_vm = EventsViewModel::from_save(slot);
            let regions_vm = RegionsViewModel::from_save(slot);
             
            Self {
                active,
                general_vm,
                stats_vm,
                equipment_vm,
                inventory_vm,
                events_vm,
                regions_vm,
            }
        }
    }
}