use crate::{
    ui::custom::selectable::SelectableButton,
    vm::{inventory::inventory::InventoryRoute, vm::ViewModel},
};
use eframe::egui::{self, Ui};

use super::{add::add::add, browse::browse_inventory};

pub fn inventory(ui: &mut Ui, vm: &mut ViewModel) {
    let inventory_vm = &mut vm.characters[vm.index].inventory_vm;
    egui::SidePanel::left("inventory_menu").show(ui.ctx(), |ui| {
        egui::ScrollArea::vertical()
            .id_source("inventory_item_type_menu")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if ui
                        .add_sized(
                            [120., 60.],
                            SelectableButton::new(
                                inventory_vm.current_route == InventoryRoute::Add,
                                "Add\n(WIP)",
                            ),
                        )
                        .clicked()
                    {
                        inventory_vm.current_route = InventoryRoute::Add;
                    };

                    if ui
                        .add_sized(
                            [120., 40.],
                            SelectableButton::new(
                                inventory_vm.current_route == InventoryRoute::Browse,
                                "Browse",
                            ),
                        )
                        .clicked()
                    {
                        inventory_vm.current_route = InventoryRoute::Browse
                    };

                    // if add_items.clicked() {
                    //     vm.slots[vm.index].inventory_vm.filter();
                    //     vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Add
                    // }
                    // if add_items.hovered() {
                    //     egui::popup::show_tooltip(ui.ctx(), add_items.id, |ui|{
                    //         ui.label(egui::RichText::new("Warning: This is an experimental feature that is still being worked on. Use with catution.").size(8.0).color(Color32::PLACEHOLDER));
                    //     });
                    // }
                    // if browse_items.clicked() {
                    //     // vm.characters[vm.index].inventory_vm.filter();
                    //     // vm.regulation.filter(
                    //     //     &vm.slots[vm.index].inventory_vm.current_type_route,
                    //     //     &vm.slots[vm.index].inventory_vm.filter_text,
                    //     // );
                    // }
                })
            });
    });

    let current_route = inventory_vm.current_route.clone();
    egui::CentralPanel::default().show(ui.ctx(), |ui| match current_route {
        InventoryRoute::None => {
            ui.label("Empty");
        }
        InventoryRoute::Add => {
            add(ui, vm);
        }
        InventoryRoute::Browse => {
            browse_inventory(ui, vm);
        }
    });
}
