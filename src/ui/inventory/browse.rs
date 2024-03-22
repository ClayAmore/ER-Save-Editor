use eframe::{egui::{self, Margin, TextFormat, Ui}, epaint::{text::LayoutJob, Color32}};
use crate::vm::{inventory::InventoryTypeRoute, vm::vm::ViewModel};

pub fn browse_inventory(ui: &mut Ui, vm:&mut ViewModel) {
    let inventory_vm = &mut vm.slots[vm.index].inventory_vm;
    
    ui.columns(2, |uis| {
        let equipped_button = uis[0].add_sized([100.,40.], egui::widgets::Button::new("Equipped"));
        let storage_button = uis[1].add_sized([100.,40.], egui::widgets::Button::new("Storage Box"));

        if equipped_button.clicked() {inventory_vm.at_storage_box = false;};
        if storage_button.clicked() {inventory_vm.at_storage_box = true;};

        if inventory_vm.at_storage_box {storage_button.highlight();}
        else {equipped_button.highlight();}
    });

    ui.add_space(6.);

    ui.columns(6,|uis| {
        let common_items = uis[0].add_sized([uis[0].available_width(), 40.], egui::Button::new("Common Item"));
        let key_items = uis[1].add_sized([uis[1].available_width(), 40.], egui::Button::new("Key Item"));
        let weapons = uis[2].add_sized([uis[2].available_width(), 40.], egui::Button::new("Weapons"));
        let armors = uis[3].add_sized([uis[3].available_width(), 40.], egui::Button::new("Armors"));
        let ashofwar = uis[4].add_sized([uis[4].available_width(), 40.], egui::Button::new("Ash of War"));
        let talismans = uis[5].add_sized([uis[5].available_width(), 40.], egui::Button::new("Talismans"));

        if common_items.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::CommonItems}
        if key_items.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::KeyItems}
        if weapons.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::Weapons}
        if armors.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::Armors}
        if ashofwar.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::AshOfWar}
        if talismans.clicked() {inventory_vm.current_type_route = InventoryTypeRoute::Talismans}

        // Highlight active 
        match inventory_vm.current_type_route {
            InventoryTypeRoute::CommonItems => {common_items.highlight();},
            InventoryTypeRoute::KeyItems => {key_items.highlight();},
            InventoryTypeRoute::Weapons => {weapons.highlight();},
            InventoryTypeRoute::Armors => {armors.highlight();},
            InventoryTypeRoute::AshOfWar => {ashofwar.highlight();},
            InventoryTypeRoute::Talismans => {talismans.highlight();},
        }
    });

    ui.add_space(6.);

    ui.horizontal(|ui|{
        let height = 20.;
        let label = ui.label("Filter: ");
        if ui.add_sized([ui.available_size().x,height], egui::widgets::TextEdit::singleline(&mut inventory_vm.filter_text)).labelled_by(label.id).changed() {
            inventory_vm.filter();
        };
    });

    let mut frame = egui::Frame::none();
    frame.inner_margin = Margin { top: 8., left: 0., bottom: 8., right: 0. };
    frame.show(ui,|ui| {
        egui::Grid::new("browse_header").spacing([16., 16.]).min_col_width(ui.available_width()/4.).striped(true).show(ui, |ui| {
            // Table Header
            let mut job = LayoutJob::default();
            job.append("Item ID", 0., TextFormat{
                color: Color32::BLACK,
                ..Default::default()
            });
            ui.label(job);
    
            let mut job = LayoutJob::default();
            job.append("Item Name", 0., TextFormat{
                color: Color32::BLACK,
                ..Default::default()
                });
            ui.label(job);
    
            let mut job = LayoutJob::default();
            job.append("Quantity", 0., TextFormat{
                color: Color32::BLACK,
                ..Default::default()
                });
            ui.label(job);
    
            let mut job = LayoutJob::default();
            job.append("Acquisition Sort ID", 0., TextFormat{
                color: Color32::BLACK,
                ..Default::default()
            });
            ui.label(job);
            ui.end_row();
        });
    });
        
    let current_inventory_list = match inventory_vm.current_type_route {
        InventoryTypeRoute::CommonItems => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_items,
        InventoryTypeRoute::KeyItems => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_key_items,
        InventoryTypeRoute::Weapons => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_weapons,
        InventoryTypeRoute::Armors => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_armors,
        InventoryTypeRoute::AshOfWar => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_aows,
        InventoryTypeRoute::Talismans => &inventory_vm.storage[inventory_vm.at_storage_box as usize].filtered_accessories,
    };
    egui::ScrollArea::vertical().show_rows(ui, 10., current_inventory_list.len(), |ui, row_range| {
        egui::Grid::new("browse_body").spacing([8., 8.]).min_col_width(ui.available_width()/4.).striped(true).show(ui, |ui| {
            for i in row_range {
                let item = &current_inventory_list[i];
                ui.label(format!("{}",item.item_id));
                ui.add(egui::Label::new(item.item_name.to_string()).wrap(true));
                ui.label(format!("{}",item.quantity));
                ui.label(format!("{}",item.inventory_index));
                ui.end_row();
            }
        });
    });
}