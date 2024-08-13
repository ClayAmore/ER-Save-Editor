use std::collections::BTreeMap;

use er_save_lib::SaveApi;

use crate::db::{map_name::MapName, regions::Region};

#[derive(Clone)]
pub struct RegionsViewModel {
    pub region_groups: BTreeMap<MapName, Vec<Region>>,
    pub regions: BTreeMap<Region, (bool, bool, bool, bool)>, // (on/off, is_open_world, is_dungeon, is_boss)
}

impl Default for RegionsViewModel {
    fn default() -> Self {
        let mut region_groups: BTreeMap<MapName, Vec<Region>> = Region::regions()
            .iter()
            .map(|r| (r.1 .2, Vec::new()))
            .collect();
        let mut regions: BTreeMap<Region, (bool, bool, bool, bool)> = BTreeMap::new();

        for (region, (_, _, map, is_open_world, is_dungeon, is_boss)) in Region::regions().iter() {
            regions.insert(*region, (false, *is_open_world, *is_dungeon, *is_boss));

            if let Some(region_groups) = region_groups.get_mut(&map) {
                region_groups.push(*region);
                region_groups.sort();
            }
        }

        Self {
            region_groups,
            regions,
        }
    }
}

impl RegionsViewModel {
    pub fn from_save(index: usize, save_api: &SaveApi) -> Self {
        let mut regions_vm = RegionsViewModel::default();

        let res = save_api.regions(index);

        match res {
            Ok(regions) => {
                for region in regions {
                    let invadeable_region = Region::from(*region);

                    if invadeable_region != Region::NonInvadeableRegion {
                        if let Some((is_on, _, _, _)) =
                            regions_vm.regions.get_mut(&invadeable_region)
                        {
                            *is_on = true;
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("{err}");
            }
        }

        regions_vm
    }
}
