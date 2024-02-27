pub mod regions {
    use std::collections::BTreeMap;

    use eframe::egui::{self, Ui};

    use crate::{db::{map_name::map_name::MAP_NAME, regions::regions::REGIONS}, ui::custom::checkbox::checkbox::{three_states_checkbox, State}, vm::vm::vm::ViewModel};

    pub fn regions(ui: &mut Ui, vm:&mut ViewModel) { 
        egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            let maps = &vm.slots[vm.index].regions_vm.region_groups;
            let regions = &mut vm.slots[vm.index].regions_vm.regions;
            select_all_checkbox(ui, regions, "All Regions");
            
            for map in maps {
                ui.push_id(map.0, |ui| {
                    let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(MAP_NAME.lock().unwrap()[&map.0]);
                    ui.horizontal(|ui|{
                        let mut state = State::Off;
                        if map.1.iter().all(|g| regions[&g]) {
                            state = State::On;
                        }
                        else if map.1.iter().any(|g| regions[&g]) {
                            state = State::InBetween;
                        }

                        if three_states_checkbox(ui, &state).clicked() {
                            match state {
                                State::Off => map.1.iter().for_each(|g| *regions.get_mut(g).expect("") = true),
                                State::On => map.1.iter().for_each(|g| *regions.get_mut(g).expect("") = false),
                                State::InBetween => map.1.iter().for_each(|g| *regions.get_mut(g).expect("") = true),
                            }
                        }

                        collapsing.show(ui, |ui| {
                            for region in map.1 {
                                let region_info = REGIONS.lock().unwrap()[&region];
                                let on = regions.get_mut(region).expect("");
                                ui.checkbox(on, region_info.1.to_string());
                            }
                        });
                    })
                });
            }
        });
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