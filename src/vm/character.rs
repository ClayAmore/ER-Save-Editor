use er_save_lib::{SaveApi, SaveApiError};

use super::{
    events::EventsViewModel, general::general_view_model::GeneralViewModel,
    inventory::inventory::InventoryViewModel, regions::RegionsViewModel, stats::StatsViewModel,
};

#[derive(Default)]
pub struct CharacterViewModel {
    pub general_vm: GeneralViewModel,
    pub stats_vm: StatsViewModel,
    // pub equipment_vm: EquipmentViewModel,
    pub inventory_vm: InventoryViewModel,
    pub events_vm: EventsViewModel,
    pub regions_vm: RegionsViewModel,
}

impl CharacterViewModel {
    pub fn from_save(save_api: &SaveApi, index: usize) -> Result<Self, SaveApiError> {
        let general_vm = GeneralViewModel::from_save(index, save_api);
        let stats_vm = StatsViewModel::from_save(index, save_api);
        // let equipment_vm = EquipmentViewModel::from_save(slot);
        let inventory_vm = InventoryViewModel::from_save(index, save_api)?;
        let events_vm = EventsViewModel::from_save(index, save_api);
        let regions_vm = RegionsViewModel::from_save(index, save_api);

        Ok(Self {
            general_vm,
            stats_vm,
            // equipment_vm,
            inventory_vm,
            events_vm,
            regions_vm,
        })
    }
}
