pub mod regions_view_model {
    use std::collections::BTreeMap;

    use crate::{db::{map_name::map_name::MapName, regions::regions::{Region, ID_TO_REGION, REGIONS}}, save::common::save_slot::SaveSlot};

    #[derive(Clone)]
    pub struct RegionsViewModel  {
        pub region_groups: BTreeMap<MapName, Vec<Region>>,
        pub regions: BTreeMap<Region, (bool, bool, bool, bool)>, // (on/off, is_open_world, is_dungeon, is_boss)
    }

    impl Default for RegionsViewModel {
        fn default() -> Self {
            let mut region_groups: BTreeMap<MapName, Vec<Region>> = REGIONS.lock().unwrap().iter().map(|r| (r.1.2, Vec::new())).collect();
            let mut regions: BTreeMap<Region, (bool, bool, bool, bool)> = BTreeMap::new();

            for (region, (_,_, map, is_open_world, is_dungeon, is_boss)) in REGIONS.lock().unwrap().iter() {
                regions.insert(*region, (false, *is_open_world, *is_dungeon, *is_boss));
                region_groups.get_mut(&map).expect("").push(*region);
                region_groups.get_mut(&map).expect("").sort();
            }

            Self { region_groups, regions }
        }
    }

    impl RegionsViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut regions_vm = RegionsViewModel::default();

            for i in 0..slot.regions.unlocked_regions_count {
                let key = &slot.regions.unlocked_regions[i as usize];
                let is_invadeable_region = ID_TO_REGION.lock().unwrap().contains_key(key);
                
                if is_invadeable_region {
                    let region = ID_TO_REGION.lock().unwrap()[key];
                    regions_vm.regions.get_mut(&region).expect("").0 = true;
                }
            }

            regions_vm
        }
    }
}