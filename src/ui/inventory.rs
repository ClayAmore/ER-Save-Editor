pub mod inventory {
    use std::cmp::Ordering;

    use eframe::{egui::{self, Layout, TextFormat, Ui}, epaint::{text::LayoutJob, Color32}};
    use egui_extras::{Column, TableBuilder};
    use crate::vm::{inventory::inventory::{InventoryItemViewModel, InventoryRoute}, vm::vm::ViewModel};
    use strsim::sorensen_dice;

    pub fn inventory(ui: &mut Ui, vm:&mut ViewModel) {
        egui::SidePanel::left("inventory_menu").show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let common_items = ui.add_sized([100., 40.], egui::Button::new("Common Items"));
                    let key_items = ui.add_sized([100., 40.], egui::Button::new("Key Items"));
                    let weapons = ui.add_sized([100., 40.], egui::Button::new("Weapons"));
                    let armors = ui.add_sized([100., 40.], egui::Button::new("Armors"));
                    let ashofwar = ui.add_sized([100., 40.], egui::Button::new("Ash of War"));
                    let talismans = ui.add_sized([100., 40.], egui::Button::new("Talismans"));
        
                    if common_items.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::CommonItems}
                    if key_items.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::KeyItems}
                    if weapons.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Weapons}
                    if armors.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Armors}
                    if ashofwar.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::AshOfWar}
                    if talismans.clicked() {vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Talismans}
        
                    // Highlight active 
                    match vm.slots[vm.index].inventory_vm.current_route {
                        InventoryRoute::None => {},
                        InventoryRoute::CommonItems => {common_items.highlight();},
                        InventoryRoute::KeyItems => {key_items.highlight();},
                        InventoryRoute::Weapons => {weapons.highlight();},
                        InventoryRoute::Armors => {armors.highlight();},
                        InventoryRoute::AshOfWar => {ashofwar.highlight();},
                        InventoryRoute::Talismans => {talismans.highlight();},
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                let mut empty: Vec<InventoryItemViewModel> = vec![];
                let inventory_vm = &mut vm.slots[vm.index].inventory_vm;
                let current_inventory_list = match inventory_vm.current_route {
                    InventoryRoute::None => &mut empty,
                    InventoryRoute::CommonItems => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].common_items,
                    InventoryRoute::KeyItems => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].key_items,
                    InventoryRoute::Weapons => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].weapons,
                    InventoryRoute::Armors => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].armors,
                    InventoryRoute::AshOfWar => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].aows,
                    InventoryRoute::Talismans => &mut inventory_vm.storage[inventory_vm.at_storage_box as usize].accessories,
                };
                
                // Sort by highest match
                current_inventory_list.sort_by(|a,b| {
                    if inventory_vm.filter_text.is_empty() {
                        return a.item_name.cmp(&b.item_name);
                    }
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &inventory_vm.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &inventory_vm.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });

                ui.columns(2, |uis| {
                    let equipped_button = uis[0].add_sized([100.,40.], egui::widgets::Button::new("Equipped"));
                    let storage_button = uis[1].add_sized([100.,40.], egui::widgets::Button::new("Storage Box"));

                    if equipped_button.clicked() {inventory_vm.at_storage_box = false;};
                    if storage_button.clicked() {inventory_vm.at_storage_box = true;};

                    if inventory_vm.at_storage_box {storage_button.highlight();}
                    else {equipped_button.highlight();}
                });
        
                ui.add_space(6.);
        
                ui.horizontal(|ui|{
                    let height = 20.;
                    ui.add_sized([ui.available_size().x,height], egui::widgets::TextEdit::singleline(&mut inventory_vm.filter_text));
                });
        
                let available_width = ui.available_width()/2.;
                let table = TableBuilder::new(ui)
                    .cell_layout(Layout::left_to_right(egui::Align::Center))
                    .striped(true)
                    .column(Column::auto())
                    .column(Column::exact(available_width))
                    .column(Column::remainder())
                    .column(Column::remainder());
        
                table.header(40.0, |mut header| {
                    header.col(|ui| {
                        let mut job = LayoutJob::default();
                        job.append("Item ID", 0., TextFormat{
                            color: Color32::BLACK,
                            ..Default::default()
                        });
                        ui.label(job);
                    });
                    header.col(|ui| {
                        let mut job = LayoutJob::default();
                        job.append("Item Name", 0., TextFormat{
                            color: Color32::BLACK,
                            ..Default::default()
                        });
                        ui.label(job);
                    });
                    header.col(|ui| {
                        let mut job = LayoutJob::default();
                        job.append("Quantity", 0., TextFormat{
                            color: Color32::BLACK,
                            ..Default::default()
                        });
                        ui.label(job);
                    });
                    header.col(|ui| {
                        let mut job = LayoutJob::default();
                        job.append("Inventory Index", 0., TextFormat{
                            color: Color32::BLACK,
                            ..Default::default()
                        });
                        ui.label(job);
                    });
                })
                .body(|mut body| {
                    for item in current_inventory_list.iter().filter(|i|{
                        if inventory_vm.filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&i.item_name.to_lowercase(), &inventory_vm.filter_text.to_lowercase());
                        distance > 0.3
                    }).collect::<Vec<&InventoryItemViewModel>>() {
                        body.row(24., |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}",item.item_id));
                            });
                            row.col(|ui| {
                                ui.add(egui::Label::new(item.item_name.to_string()).wrap(true));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}",item.quantity));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}",item.inventory_index));
                            });
                        });
                    }
                });
                })
            });
        
    }
}