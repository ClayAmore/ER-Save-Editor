pub mod regions_view_model {
    use std::{collections::BTreeMap, fs::File, io::Write, path::PathBuf};

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
                .add_filter("regions", &["regions", "Save Editor Exported Region File"])
                .add_filter("*", &["*", "Cheat Engine Exported Region File"])
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

        // Provides a string in the same format that is provided by The Grand Archives: Elden Ring -
        // Invasion Regions Export.
        // This string provides activated regions in the format of numbers for region ids with new lines
        // delimiting the entries
        pub fn as_string(&self) -> String {
            let mut text = String::from("");

            for (region, (activated, _, _,_)) in self.regions.iter() {
                if *activated {
                    let (region_id, _,_,_,_,_) = &REGIONS.lock().unwrap()[region];
                    text.push_str(&region_id.to_string());
                    text.push('\n');
                }
            }

            return text;
        }

        pub fn write_regions(&self, path: PathBuf) {
            let path_with_extension = if path.extension().unwrap_or_default() != "regions" {
                path.with_extension("regions")
            } else {
                path
            };
            let mut f = File::create(path_with_extension).expect("");
            let region_string =  &self.as_string();
            
            f.write_all(&region_string.as_bytes()).expect("Failed to write file");
        }

        pub fn set_active_regions(&mut self, active_region_ids: &Vec<u32>){
            let active_invasion_region_count = active_region_ids.len();
            *self = Self::default();

            if active_invasion_region_count == 0 {
                return;
            }
            for i in 0..active_region_ids.len() {
                let key = &active_region_ids[i as usize];
                
                if ID_TO_REGION.lock().unwrap().contains_key(key) {
                    let region = ID_TO_REGION.lock().unwrap()[key];
                    self.regions.get_mut(&region).expect("").0 = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::regions_view_model::RegionsViewModel;

    #[test]
    fn test_as_string() {
        let region_vm = RegionsViewModel::default();
        let region_as_string = region_vm.as_string();

        assert_eq!(region_as_string, "", "Should not contain any enabled regions")
    }

    #[test]
    fn test_write_regions() {
        let regions_vm = &crate::vm::regions::regions_view_model::RegionsViewModel::default();
        let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");

        regions_vm.write_regions(temp_file.path().to_path_buf());
        let file_contents = std::fs::read_to_string(temp_file.path()).expect("Failed to read temporary file");
        assert_eq!(file_contents, "");
    }

    #[test]
    fn test_set_active_regions() {

        let key = 6101000;
        let regions_vm = &mut crate::vm::regions::regions_view_model::RegionsViewModel::default();

        let region = crate::db::regions::regions::ID_TO_REGION.lock().unwrap()[&key];

        assert!(!regions_vm.regions.get_mut(&region).expect("").0, "Region 6101000 should be locked");
        regions_vm.set_active_regions(&vec![key]);
        assert!(regions_vm.regions.get_mut(&region).expect("").0, "Region 6101000 should be unlocked");
        assert_eq!(regions_vm.regions.values().filter(|&&(is_active, _, _, _)| is_active).count(), 1, "Only one region should be unlocked");
        regions_vm.set_active_regions(&vec![]);
        assert!(!regions_vm.regions.get_mut(&region).expect("").0, "Region 6101000 should be locked");
    }
}
        