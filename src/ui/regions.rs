use std::collections::BTreeMap;

use eframe::egui::{self, Ui};

use crate::{
    db::{map_name::MapName, regions::Region},
    vm::vm::ViewModel,
};

use super::custom::checkbox::{three_states_checkbox, State};

/// Draws the 'Regions' view.
pub fn regions(ui: &mut Ui, vm: &mut ViewModel) {
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            let maps = &vm.characters[vm.index].regions_vm.region_groups;
            let regions = &mut vm.characters[vm.index].regions_vm.regions;
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

            for (map_name, region_group) in maps {
                // Skip DLC regions if not activated
                if !vm.dlc && map_name == &MapName::RealmofShadow {
                    continue;
                }

                ui.push_id(map_name, |ui| {
                    let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(
                        if let Some(map_name_str) = MapName::map_names().get(map_name) {
                            map_name_str
                        } else {
                            "Uknown Region!"
                        },
                    );
                    ui.horizontal(|ui| {
                        let mut state = State::Off;
                        if region_group.iter().all(|g| regions[&g].0) {
                            state = State::On;
                        } else if region_group.iter().any(|g| regions[&g].0) {
                            state = State::InBetween;
                        }

                        if three_states_checkbox(ui, &state).clicked() {
                            match state {
                                State::Off => region_group
                                    .iter()
                                    .for_each(|g| regions.get_mut(g).expect("").0 = true),
                                State::On => region_group
                                    .iter()
                                    .for_each(|g| regions.get_mut(g).expect("").0 = false),
                                State::InBetween => region_group
                                    .iter()
                                    .for_each(|g| regions.get_mut(g).expect("").0 = true),
                            }
                        }

                        collapsing.show(ui, |ui| {
                            for region in region_group {
                                if let Some((_, name_str, _, _, _, _)) =
                                    Region::regions().get(region)
                                {
                                    if let Some((is_on, _, _, _)) = regions.get_mut(region) {
                                        ui.checkbox(is_on, name_str.to_string());
                                    }
                                }
                            }
                        });
                    })
                });
            }
        });
}

fn select_all_checkbox<T>(
    ui: &mut Ui,
    map: &mut BTreeMap<T, (bool, bool, bool, bool)>,
    label: &str,
) {
    let mut state = State::Off;
    if map.values().all(|(on, _, _, _)| *on) {
        state = State::On;
    } else if map.values().any(|(on, _, _, _)| *on) {
        state = State::InBetween;
    }

    ui.horizontal(|ui| {
        if three_states_checkbox(ui, &state).clicked() {
            match state {
                State::Off => map.values_mut().for_each(|(on, _, _, _)| *on = true),
                State::On => map.values_mut().for_each(|(on, _, _, _)| *on = false),
                State::InBetween => map.values_mut().for_each(|(on, _, _, _)| *on = true),
            }
        }
        ui.label(label);
    });
}

fn select_open_world_checkbox<T>(
    ui: &mut Ui,
    map: &mut BTreeMap<T, (bool, bool, bool, bool)>,
    label: &str,
) {
    let mut state = State::Off;
    if map
        .values()
        .filter(|(_, is_open_world, _, _)| *is_open_world)
        .all(|(on, _, _, _)| *on)
    {
        state = State::On;
    } else if map
        .values()
        .filter(|(_, is_open_world, _, _)| *is_open_world)
        .any(|(on, _, _, _)| *on)
    {
        state = State::InBetween;
    }

    ui.horizontal(|ui| {
        if three_states_checkbox(ui, &state).clicked() {
            match state {
                State::Off => map
                    .values_mut()
                    .filter(|(_, is_open_world, _, _)| *is_open_world)
                    .for_each(|(on, _, _, _)| *on = true),
                State::On => map
                    .values_mut()
                    .filter(|(_, is_open_world, _, _)| *is_open_world)
                    .for_each(|(on, _, _, _)| *on = false),
                State::InBetween => map
                    .values_mut()
                    .filter(|(_, is_open_world, _, _)| *is_open_world)
                    .for_each(|(on, _, _, _)| *on = true),
            }
        }
        ui.label(label);
    });
}

fn select_dungeon_checkbox<T>(
    ui: &mut Ui,
    map: &mut BTreeMap<T, (bool, bool, bool, bool)>,
    label: &str,
) {
    let mut state = State::Off;
    if map
        .values()
        .filter(|(_, _, is_dungeon, _)| *is_dungeon)
        .all(|(on, _, _, _)| *on)
    {
        state = State::On;
    } else if map
        .values()
        .filter(|(_, _, is_dungeon, _)| *is_dungeon)
        .any(|(on, _, _, _)| *on)
    {
        state = State::InBetween;
    }

    ui.horizontal(|ui| {
        if three_states_checkbox(ui, &state).clicked() {
            match state {
                State::Off => map
                    .values_mut()
                    .filter(|(_, _, is_dungeon, _)| *is_dungeon)
                    .for_each(|(on, _, _, _)| *on = true),
                State::On => map
                    .values_mut()
                    .filter(|(_, _, is_dungeon, _)| *is_dungeon)
                    .for_each(|(on, _, _, _)| *on = false),
                State::InBetween => map
                    .values_mut()
                    .filter(|(_, _, is_dungeon, _)| *is_dungeon)
                    .for_each(|(on, _, _, _)| *on = true),
            }
        }
        ui.label(label);
    });
}

fn select_bosses_checkbox<T>(
    ui: &mut Ui,
    map: &mut BTreeMap<T, (bool, bool, bool, bool)>,
    label: &str,
) {
    let mut state = State::Off;
    if map
        .values()
        .filter(|(_, _, _, is_boss)| *is_boss)
        .all(|(on, _, _, _)| *on)
    {
        state = State::On;
    } else if map
        .values()
        .filter(|(_, _, _, is_boss)| *is_boss)
        .any(|(on, _, _, _)| *on)
    {
        state = State::InBetween;
    }

    ui.horizontal(|ui| {
        if three_states_checkbox(ui, &state).clicked() {
            match state {
                State::Off => map
                    .values_mut()
                    .filter(|(_, _, _, is_boss)| *is_boss)
                    .for_each(|(on, _, _, _)| *on = true),
                State::On => map
                    .values_mut()
                    .filter(|(_, _, _, is_boss)| *is_boss)
                    .for_each(|(on, _, _, _)| *on = false),
                State::InBetween => map
                    .values_mut()
                    .filter(|(_, _, _, is_boss)| *is_boss)
                    .for_each(|(on, _, _, _)| *on = true),
            }
        }
        ui.label(label);
    });
}
