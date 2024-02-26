pub mod regions {
    use eframe::egui::{self, Ui};

    use crate::{db::{map_name::map_name::MAP_NAME, regions::regions::REGIONS}, vm::vm::vm::ViewModel};

    pub fn regions(ui: &mut Ui, vm:&mut ViewModel) { 
        egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            let maps = &vm.slots[vm.index].regions_vm.region_groups;
            let regions = &mut vm.slots[vm.index].regions_vm.regions;
            
            for map in maps {
                ui.push_id(map.0, |ui| {
                    let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(MAP_NAME.lock().unwrap()[&map.0]);
                    ui.horizontal(|ui|{
                        ui.checkbox(&mut false, "");
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
}