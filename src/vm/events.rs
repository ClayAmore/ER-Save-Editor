use std::collections::BTreeMap;

use er_save_lib::SaveApi;

use crate::db::{
    bosses::Boss, colosseums::Colosseum, cookbooks::Cookbook, graces::maps::Grace,
    map_name::MapName, maps::Map, summoning_pools::SummoningPool, whetblades::Whetblade,
};

#[derive(Clone)]
pub enum EventsRoute {
    None,
    SitesOfGrace,
    Whetblades,
    Cookboks,
    Maps,
    Bosses,
    SummoningPools,
    Colosseums,
}

#[derive(Clone)]
pub struct EventsViewModel {
    pub current_route: EventsRoute,
    pub grace_groups: BTreeMap<MapName, Vec<Grace>>,
    pub graces: BTreeMap<Grace, bool>,
    pub whetblades: BTreeMap<Whetblade, bool>,
    pub cookbooks: BTreeMap<Cookbook, bool>,
    pub maps: BTreeMap<Map, bool>,
    pub bosses: BTreeMap<Boss, bool>,
    pub summoning_pools: BTreeMap<SummoningPool, bool>,
    pub colosseums: BTreeMap<Colosseum, bool>,
}

impl Default for EventsViewModel {
    fn default() -> Self {
        Self {
            current_route: EventsRoute::None,
            grace_groups: MapName::map_names()
                .iter()
                .map(|m| (*m.0, Vec::new()))
                .collect::<BTreeMap<_, _>>(),
            graces: Default::default(),
            whetblades: Default::default(),
            cookbooks: Default::default(),
            maps: Default::default(),
            bosses: Default::default(),
            summoning_pools: Default::default(),
            colosseums: Default::default(),
        }
    }
}

impl EventsViewModel {
    pub fn from_save(index: usize, save_api: &SaveApi) -> Self {
        let mut events_vm = EventsViewModel::default();

        // let id_to_offset_lookup = EVENT_FLAGS.lock().unwrap();

        // Graces
        for (key, (map_name, event_id, _)) in Grace::graces().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.graces.insert(*key, is_on);
                    events_vm
                        .grace_groups
                        .get_mut(&map_name)
                        .expect("")
                        .push(*key);
                    events_vm.grace_groups.get_mut(&map_name).expect("").sort();
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        // Whetblades
        for (key, (event_id, _)) in Whetblade::whetblades().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.whetblades.insert(*key, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        // Cookbooks
        for (key, (event_id, _)) in Cookbook::cookbooks().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.cookbooks.insert(*key, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        // Maps
        for (key, (event_id, _)) in Map::maps().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.maps.insert(*key, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        // Bosses
        for (boss, (event_id, _)) in Boss::bosses().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.bosses.insert(*boss, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            };
        }

        // Summoning Pools
        for (key, (event_id, _)) in SummoningPool::summoning_pools().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.summoning_pools.insert(*key, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            };
        }

        // Colosseums
        for (key, (event_id, _)) in Colosseum::colusseums().iter() {
            let res = save_api.get_event_flag(*event_id, index);
            match res {
                Ok(is_on) => {
                    events_vm.colosseums.insert(*key, is_on);
                }
                Err(err) => {
                    println!("{err}");
                }
            };
        }

        events_vm
    }
}
