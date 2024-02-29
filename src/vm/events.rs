pub mod events_view_model {
    use std::collections::BTreeMap;

    use crate::{db::{bosses::bosses::{Boss, BOSSES}, colosseums::colosseums::{Colosseum, COLOSSEUMS}, cookbooks::books::{Cookbook, COOKBOKS}, event_flags::event_flags::EVENT_FLAGS, graces::maps::{Grace, GRACES}, map_name::map_name::{MapName, MAP_NAME}, maps::maps::{Map, MAPS}, summoning_pools::summoning_pools::{SummoningPool, SUMMONING_POOLS}, whetblades::whetblades::{Whetblade, WHETBLADES}}, save::common::save_slot::SaveSlot, util::bit::bit::get_bit};

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
    pub struct EventsViewModel  {
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
                grace_groups: MAP_NAME.lock().unwrap().iter().map(|m| (*m.0, Vec::new())).collect::<BTreeMap<_,_>>(),
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
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut events_vm = EventsViewModel::default();

            let id_to_offset_lookup = EVENT_FLAGS.lock().unwrap();

            // Graces
            for (key, value) in GRACES.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.1];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.graces.insert(*key, on);
                events_vm.grace_groups.get_mut(&value.0).expect("").push(*key);
                events_vm.grace_groups.get_mut(&value.0).expect("").sort();
            }

            // Whetblades
            for (key, value) in WHETBLADES.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.whetblades.insert(*key, on);
            }

            // Cookbooks
            for (key, value) in COOKBOKS.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.cookbooks.insert(*key, on);
            }

            // Maps
            for (key, value) in MAPS.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.maps.insert(*key, on);
            }

            // Bosses
            for (key, value) in BOSSES.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.bosses.insert(*key, on);
            }

            // Summoning Pools
            for (key, value) in SUMMONING_POOLS.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.summoning_pools.insert(*key, on);
            }

            // Colosseums
            for (key, value) in COLOSSEUMS.lock().unwrap().iter() {
                let event_flag_info = id_to_offset_lookup[&value.0];
                let on = get_bit(slot.event_flags.flags[event_flag_info.0 as usize], event_flag_info.1);
                events_vm.colosseums.insert(*key, on);
            }

            events_vm
        }
    }
}