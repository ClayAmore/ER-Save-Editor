pub mod inventory {
    use eframe::egui::{self, Ui};
    use crate::ui::inventory::{add::add_inventory, browse::browse_inventory};
    use crate::vm::{inventory::inventory::InventoryRoute, vm::vm::ViewModel};

    pub fn inventory(ui: &mut Ui, vm:&mut ViewModel) {
        egui::SidePanel::left("inventory_menu").show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("inventory_item_type_menu")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let add_items = ui.add_sized([120., 60.], egui::Button::new("Add\n(WORK IN PROGRESS)"));
                    let browse_items = ui.add_sized([120., 40.], egui::Button::new("Browse"));

                    if add_items.clicked() {
                        vm.slots[vm.index].inventory_vm.filter();
                        vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Add
                    }
                    if browse_items.clicked() {
                        vm.slots[vm.index].inventory_vm.filter();
                        vm.regulation.filter(&vm.slots[vm.index].inventory_vm.current_type_route, &vm.slots[vm.index].inventory_vm.filter_text);
                        vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Browse
                    }
                    
                    // Highlight active 
                    match vm.slots[vm.index].inventory_vm.current_route {
                        InventoryRoute::None => {},
                        InventoryRoute::Add => {add_items.highlight();},
                        InventoryRoute::Browse => {browse_items.highlight();},
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ui.ctx(), |ui|{
            match vm.slots[vm.index].inventory_vm.current_route {
                InventoryRoute::None => {ui.label("Empty");},
                InventoryRoute::Add => {add_inventory(ui, vm);},
                InventoryRoute::Browse => {browse_inventory(ui, vm);},
            }
        });
    }
}