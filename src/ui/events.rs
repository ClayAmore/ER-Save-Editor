pub mod events {

    use std::collections::BTreeMap;

    use eframe::egui::{self, Ui};
    use crate::{db::{bosses::bosses::{Boss, BOSSES}, colosseums::colosseums::{Colosseum, COLOSSEUMS}, cookbooks::books::{Cookbook, COOKBOKS}, graces::maps::GRACES, map_name::map_name::{MapName, MAP_NAME}, maps::maps::{Map, MAPS}, summoning_pools::summoning_pools::{SummoningPool, SUMMONING_POOLS}, whetblades::whetblades::{Whetblade, WHETBLADES}}, ui::custom::checkbox::checkbox::{three_states_checkbox, State}, vm::{events::events_view_model::EventsRoute, vm::vm::ViewModel}};

    pub fn events(ui: &mut Ui, vm:&mut ViewModel) {
        egui::SidePanel::left("inventory_menu").show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let sites_of_grace = ui.add_sized([100., 40.], egui::Button::new("Sites Of Grace"));
                    let whetblades = ui.add_sized([100., 40.], egui::Button::new("Whetblades"));
                    let cookboks = ui.add_sized([100., 40.], egui::Button::new("Cookbooks"));
                    let maps = ui.add_sized([100., 40.], egui::Button::new("Maps"));
                    let bosses = ui.add_sized([100., 40.], egui::Button::new("Bosses"));
                    let summoning_pools = ui.add_sized([100., 60.], egui::Button::new("Summoning\nPools"));
                    let colosseums = ui.add_sized([100., 40.], egui::Button::new("Colosseums"));
        
                    if sites_of_grace.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::SitesOfGrace}
                    if whetblades.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Whetblades}
                    if cookboks.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Cookboks}
                    if maps.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Maps}
                    if bosses.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Bosses}
                    if summoning_pools.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::SummoningPools}
                    if colosseums.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Colosseums}
        
                    // Highlight active 
                    match vm.slots[vm.index].events_vm.current_route {
                        EventsRoute::None => {},
                        EventsRoute::SitesOfGrace => {sites_of_grace.highlight();},
                        EventsRoute::Whetblades => {whetblades.highlight();},
                        EventsRoute::Cookboks => {cookboks.highlight();},
                        EventsRoute::Maps => {maps.highlight();},
                        EventsRoute::Bosses => {bosses.highlight();},
                        EventsRoute::SummoningPools => {summoning_pools.highlight();},
                        EventsRoute::Colosseums => {colosseums.highlight();},
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .auto_shrink(false)
            .show(ui, |ui| {
                match vm.slots[vm.index].events_vm.current_route {
                    EventsRoute::None => {},
                    EventsRoute::SitesOfGrace => {graces(ui, vm);},
                    EventsRoute::Whetblades => {whetblades(ui, vm);},
                    EventsRoute::Cookboks => {cookbooks(ui, vm);},
                    EventsRoute::Maps => {maps(ui, vm);},
                    EventsRoute::Bosses => {bosses(ui, vm);},
                    EventsRoute::SummoningPools => {summoning_pools(ui, vm);},
                    EventsRoute::Colosseums => {colosseums(ui, vm);},
                }
            });
        });
        
    }

    fn graces(ui: &mut Ui, vm:&mut ViewModel) {
        ui.vertical(|ui| {
            let maps = &vm.slots[vm.index].events_vm.grace_groups;
            let graces = &mut vm.slots[vm.index].events_vm.graces;
            select_all_checkbox(ui, graces, "All Graces");
            for map in maps {
                ui.push_id(map.0, |ui| {
                    let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(MAP_NAME.lock().unwrap()[&map.0]);
                    ui.horizontal(|ui|{
                        let mut state = State::Off;
                        if map.1.iter().all(|g| graces[&g]) {
                            state = State::On;
                        }
                        else if map.1.iter().any(|g| graces[&g]) {
                            state = State::InBetween;
                        }

                        if three_states_checkbox(ui, &state).clicked() {
                            match state {
                                State::Off => map.1.iter().for_each(|g| *graces.get_mut(g).expect("") = true),
                                State::On => map.1.iter().for_each(|g| *graces.get_mut(g).expect("") = false),
                                State::InBetween => map.1.iter().for_each(|g| *graces.get_mut(g).expect("") = true),
                            }
                        }

                        collapsing.show(ui, |ui| {
                            for grace in map.1 {
                                let grace_info: (MapName, u32, &str) = GRACES.lock().unwrap()[&grace];
                                let on = graces.get_mut(grace).expect("");
                                ui.checkbox(on, grace_info.2.to_string());
                            }
                        });
                    });
                });
            }
        });
    }

    fn whetblades(ui: &mut Ui, vm:&mut ViewModel) {
        let whetblades = &mut vm.slots[vm.index].events_vm.whetblades;
        select_all_checkbox::<Whetblade>(ui, whetblades, "All Whetblades");
        for (whetblade, on) in whetblades {
            let whetblade_info: (u32,&str) = WHETBLADES.lock().unwrap()[&whetblade];
            ui.checkbox(on, whetblade_info.1.to_string());
        }
    }

    fn cookbooks(ui: &mut Ui, vm:&mut ViewModel) {
        let cookbooks = &mut vm.slots[vm.index].events_vm.cookbooks;
        select_all_checkbox::<Cookbook>(ui, cookbooks, "All Cookbooks");
        for (cookbook, on) in cookbooks {
            let cookbook_info: (u32,&str) = COOKBOKS.lock().unwrap()[&cookbook];
            ui.checkbox(on, cookbook_info.1.to_string());
        }
    }

    fn maps(ui: &mut Ui, vm:&mut ViewModel) {
        let maps = &mut vm.slots[vm.index].events_vm.maps;
        select_all_checkbox::<Map>(ui, maps, "All Maps");
        for (map, on) in maps {
            let map_info: (u32,&str) = MAPS.lock().unwrap()[&map];
            ui.checkbox(on, map_info.1.to_string());
        }
    }

    fn bosses(ui: &mut Ui, vm:&mut ViewModel) {
        let bosses = &mut vm.slots[vm.index].events_vm.bosses;
        select_all_checkbox::<Boss>(ui, bosses, "All Bosses");
        for (boss, on) in bosses {
            let boss_info: (u32,&str) = BOSSES.lock().unwrap()[&boss];
            ui.checkbox(on, boss_info.1.to_string());
        }
    }

    fn summoning_pools(ui: &mut Ui, vm:&mut ViewModel) {
        let summoning_pools = &mut vm.slots[vm.index].events_vm.summoning_pools;
        select_all_checkbox::<SummoningPool>(ui, summoning_pools, "All Summoning Pools");
        for (summoning_pool, on) in summoning_pools {
            let summoning_pool_info: (u32,&str) = SUMMONING_POOLS.lock().unwrap()[&summoning_pool];
            ui.checkbox(on, summoning_pool_info.1.to_string());
        }
    }

    fn colosseums(ui: &mut Ui, vm:&mut ViewModel) {
        let colosseums = &mut vm.slots[vm.index].events_vm.colosseums;
        select_all_checkbox::<Colosseum>(ui, colosseums, "All Colusseums");
        for (colosseum, on) in colosseums {
            let colosseum_info: (u32,&str) = COLOSSEUMS.lock().unwrap()[&colosseum];
            ui.checkbox(on, colosseum_info.1.to_string());
        }
    }

    fn select_all_checkbox<T>(ui: &mut Ui, map: &mut BTreeMap<T, bool>, label: &str) {
        let mut state = State::Off;
        if map.values().all(|w|*w) {
            state = State::On;
        }
        else if map.values().any(|w|*w) {
            state = State::InBetween;
        }

        ui.horizontal(|ui| {
            if three_states_checkbox(ui, &state).clicked() {
                match state {
                    State::Off => map.values_mut().for_each(|w| *w = true),
                    State::On => map.values_mut().for_each(|w| *w = false),
                    State::InBetween => map.values_mut().for_each(|w| *w = true),
                }
            }
            ui.label(label);
        });
        ui.separator();
    }
}