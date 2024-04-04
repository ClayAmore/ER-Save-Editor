pub mod regions_view_model {
    use std::{collections::BTreeMap, path::PathBuf};

    use rfd::FileDialog;

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
        fn region_file_dialog() -> FileDialog {
            FileDialog::new()
                .add_filter("TXT", &["txt", "Cheat Engine Exported Region File"])
                .add_filter("*", &["*", "All files"])
                .set_directory("./")
        }

        pub fn open_region_file_dialog() -> Option<PathBuf> {
            Self::region_file_dialog()
                .pick_file()
        }
    
        pub fn save_region_file_dialog() -> Option<PathBuf> {
            Self::region_file_dialog()
                .save_file()
        }

        pub fn from_save(slot:& SaveSlot, enabled_ids: Option<&Vec<u32>>) -> Self {
            let mut regions_vm = RegionsViewModel::default();
            // Allows an override to unlocked_regions
            let unlocked_regions: (usize, &Vec<u32>) = match enabled_ids {
                Some(ids) => {
                    (ids.len(), ids)
                },
                None => {
                    (slot.regions.unlocked_regions_count.try_into().unwrap(), &slot.regions.unlocked_regions)

                }
            };
            for i in 0..unlocked_regions.0 {
                let key = &unlocked_regions.1[i as usize];
                let is_invadeable_region = ID_TO_REGION.lock().unwrap().contains_key(key);
                
                if is_invadeable_region {
                    let region = ID_TO_REGION.lock().unwrap()[key];
                    regions_vm.regions.get_mut(&region).expect("").0 = true;
                }
            }

            regions_vm
        }

        pub fn to_string(&self) -> String {
            let mut text = String::from("");

            for (region, (activated, _, _,_)) in self.regions.iter() {
                if *activated {
                    // Matches the format provided by a CE table for Elden Ring
                    let (region_id, _,_,_,_,_) = &REGIONS.lock().unwrap()[region];
                    text.push_str(&region_id.to_string());
                    text.push('\n');
                }
            }

            return text;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_save_with_enabled_ids() {
        let mut slot = crate::save::common::save_slot::SaveSlot::default();
        slot.regions.unlocked_regions = vec![];

        let key = 6101000;
        let enabled_ids = vec![key];
        let regions_vm = &mut crate::vm::regions::regions_view_model::RegionsViewModel::from_save(&slot, Some(&enabled_ids));

        let region = crate::db::regions::regions::ID_TO_REGION.lock().unwrap()[&key];

        assert!(regions_vm.regions.get_mut(&region).expect("").0, "Region 6101000 should be unlocked");
        assert_eq!(regions_vm.regions.values().filter(|&&(is_active, _, _, _)| is_active).count(), 1, "Only one region should be unlocked");
    }
}
        