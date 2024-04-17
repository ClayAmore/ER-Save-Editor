pub mod regions {
    use std::{collections::BTreeMap, fs};

    use eframe::egui::{self, Ui};

    use crate::{db::{map_name::map_name::MAP_NAME, regions::regions::REGIONS}, ui::custom::checkbox::checkbox::{three_states_checkbox, State}, vm::regions::regions_view_model::RegionsViewModel, App};

    pub fn regions(ui: &mut Ui, app: &mut App) { 
        egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let steam_id_presence = !app.vm.steam_id.is_empty();
                
                import_file(ui, steam_id_presence, app);
                export_file(ui, steam_id_presence, app);
            });
            ui.separator();

            let maps = &app.vm.slots[app.vm.index].regions_vm.region_groups;
            let regions = &mut app.vm.slots[app.vm.index].regions_vm.regions;
            ui.horizontal(|ui| {
                select_all_checkbox(ui, regions, "All Regions");
                ui.separator();
                select_open_world_checkbox(ui, regions, "Open World");
                ui.separator();
                select_dungeon_checkbox(ui, regions, "Dungeons");
                ui.separator();
                select_bosses_checkbox(ui, regions, "Bosses");
            });
            ui.separator();
            
            for map in maps {
                ui.push_id(map.0, |ui| {
                    let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(MAP_NAME.lock().unwrap()[&map.0]);
                    ui.horizontal(|ui|{
                        let mut state = State::Off;
                        if map.1.iter().all(|g| regions[&g].0) {
                            state = State::On;
                        }
                        else if map.1.iter().any(|g| regions[&g].0) {
                            state = State::InBetween;
                        }

                        if three_states_checkbox(ui, &state).clicked() {
                            match state {
                                State::Off => map.1.iter().for_each(|g| regions.get_mut(g).expect("").0 = true),
                                State::On => map.1.iter().for_each(|g| regions.get_mut(g).expect("").0 = false),
                                State::InBetween => map.1.iter().for_each(|g| regions.get_mut(g).expect("").0 = true),
                            }
                        }

                        collapsing.show(ui, |ui| {
                            for region in map.1 {
                                let region_info = REGIONS.lock().unwrap()[&region];
                                let on = &mut regions.get_mut(region).expect("").0;
                                ui.checkbox(on, region_info.1.to_string());
                            }
                        });
                    })
                });
            }
        });
    }

    fn import_file(ui: &mut Ui, steam_id_presence: bool, app: &mut App) {
        let import_button = egui::widgets::Button::new(
            egui::RichText::new(
                format!(
                    "{} Import Regions",
                    egui_phosphor::regular::DOWNLOAD_SIMPLE
                )
            )
        );

        if ui.add_enabled(steam_id_presence, import_button).clicked() {
            let files = RegionsViewModel::open_region_file_dialog().expect("File dialog didn't produce any files");
            let region_text = fs::read_to_string(files).expect("Could not read string from file");
            let region_ids: Vec<u32> = get_ids(region_text);

            app.vm.slots[app.vm.index].regions_vm.set_active_regions(
                &region_ids
            );
        }

    }

    fn get_ids(region_text: String) -> Vec<u32> {
        region_text
        .split_whitespace() // Split the content by whitespace (newlines, spaces, etc.)
        .filter_map(|s| s.parse::<u32>().ok()) // Parse each piece as an i32, filtering out any errors
        .collect()
    }

    fn export_file(ui: &mut Ui, steam_id_presence: bool, app: &mut App) {

        let export_button = egui::widgets::Button::new(
            egui::RichText::new(
                format!("{} Export Regions", egui_phosphor::regular::DOWNLOAD_SIMPLE)
            )
        );

        if ui.add_enabled(steam_id_presence, export_button).clicked() {
            let path = RegionsViewModel::save_region_file_dialog().expect("Should provide a path to a new or existing file");
            app.vm.slots[app.vm.index].regions_vm.write_regions(path);
        }
    }
    
    fn select_all_checkbox<T>(ui: &mut Ui, map: &mut BTreeMap<T, (bool, bool, bool, bool)>, label: &str) {
        let mut state = State::Off;
        if map.values().all(|(on,_,_,_)| *on) {
            state = State::On;
        }
        else if map.values().any(|(on,_,_,_)| *on) {
            state = State::InBetween;
        }

        ui.horizontal(|ui| {
            if three_states_checkbox(ui, &state).clicked() {
                match state {
                    State::Off => map.values_mut().for_each(|(on,_,_,_)| *on = true),
                    State::On => map.values_mut().for_each(|(on,_,_,_)| *on = false),
                    State::InBetween => map.values_mut().for_each(|(on,_,_,_)| *on = true),
                }
            }
            ui.label(label);
        });
    }

    
    fn select_open_world_checkbox<T>(ui: &mut Ui, map: &mut BTreeMap<T, (bool, bool, bool, bool)>, label: &str) {
        let mut state = State::Off;
        if map.values().filter(|(_, is_open_world,_,_)|*is_open_world).all(|(on,_,_,_)| *on) {
            state = State::On;
        }
        else if map.values().filter(|(_, is_open_world,_,_)|*is_open_world).any(|(on,_,_,_)| *on) {
            state = State::InBetween;
        }

        ui.horizontal(|ui| {
            if three_states_checkbox(ui, &state).clicked() {
                match state {
                    State::Off => map.values_mut().filter(|(_, is_open_world,_,_)|*is_open_world).for_each(|(on,_,_,_)| *on = true),
                    State::On => map.values_mut().filter(|(_, is_open_world,_,_)|*is_open_world).for_each(|(on,_,_,_)| *on = false),
                    State::InBetween => map.values_mut().filter(|(_, is_open_world,_,_)|*is_open_world).for_each(|(on,_,_,_)| *on = true),
                }
            }
            ui.label(label);
        });
    }
    
    fn select_dungeon_checkbox<T>(ui: &mut Ui, map: &mut BTreeMap<T, (bool, bool, bool, bool)>, label: &str) {
        let mut state = State::Off;
        if map.values().filter(|(_,_, is_dungeon,_)| *is_dungeon).all(|(on,_,_,_)| *on) {
            state = State::On;
        }
        else if map.values().filter(|(_,_, is_dungeon,_)| *is_dungeon).any(|(on,_,_,_)| *on) {
            state = State::InBetween;
        }

        ui.horizontal(|ui| {
            if three_states_checkbox(ui, &state).clicked() {
                match state {
                    State::Off => map.values_mut().filter(|(_,_, is_dungeon,_)| *is_dungeon).for_each(|(on,_,_,_)| *on = true),
                    State::On => map.values_mut().filter(|(_,_, is_dungeon,_)| *is_dungeon).for_each(|(on,_,_,_)| *on = false),
                    State::InBetween => map.values_mut().filter(|(_,_, is_dungeon,_)| *is_dungeon).for_each(|(on,_,_,_)| *on = true),
                }
            }
            ui.label(label);
        });
    }
    
    fn select_bosses_checkbox<T>(ui: &mut Ui, map: &mut BTreeMap<T, (bool, bool, bool, bool)>, label: &str) {
        let mut state = State::Off;
        if map.values().filter(|(_,_,_, is_boss)| *is_boss).all(|(on,_,_,_)| *on) {
            state = State::On;
        }
        else if map.values().filter(|(_,_,_, is_boss)| *is_boss).any(|(on,_,_,_)| *on) {
            state = State::InBetween;
        }

        ui.horizontal(|ui| {
            if three_states_checkbox(ui, &state).clicked() {
                match state {
                    State::Off => map.values_mut().filter(|(_,_,_, is_boss)| *is_boss).for_each(|(on,_,_,_)| *on = true),
                    State::On => map.values_mut().filter(|(_,_,_, is_boss)| *is_boss).for_each(|(on,_,_,_)| *on = false),
                    State::InBetween => map.values_mut().filter(|(_,_,_, is_boss)| *is_boss).for_each(|(on,_,_,_)| *on = true),
                }
            }
            ui.label(label);
        });
    }
}