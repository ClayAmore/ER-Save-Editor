use std::{ops::Deref, rc::Rc};

use eframe::egui::{self, Color32, Layout, Ui};

use crate::vm::inventory::{add::add::SelectedItem, inventory::InventoryViewModel};

/// Draws the list for adding single items
pub(crate) fn single(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {
    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui
                .add(egui::TextEdit::singleline(&mut inventory_vm.filter_text))
                .labelled_by(ui.label("Filter:").id)
                .changed()
            {
                inventory_vm.add_vm.to_route(
                    inventory_vm.add_vm.current_sub_type_route.clone(),
                    &inventory_vm.filter_text,
                );
            };
        });
        ui.separator();
        ui.add_space(6.);

        let row_height = 10.;
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 8.)
            .show_rows(
                ui,
                row_height,
                inventory_vm.add_vm.current_list.len(),
                |ui, row_range| {
                    for i in row_range {
                        let item = inventory_vm.add_vm.current_list[i].clone();
                        let mut text = egui::RichText::new(format!(
                            "{}",
                            item.as_ref().borrow().item_name.as_ref().borrow()
                        ));
                        if let Some(header) = &inventory_vm.add_vm.selected_header {
                            if Rc::ptr_eq(header, &item) {
                                text = egui::RichText::new(format!(
                                    "{}",
                                    item.as_ref().borrow().item_name.as_ref().borrow()
                                ))
                                .strong()
                                .heading();
                            }
                        }
                        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                            if ui
                                .add(egui::Button::new(text).fill(Color32::TRANSPARENT))
                                .clicked()
                            {
                                inventory_vm.add_vm.select_item(item.clone());
                                inventory_vm.add_vm.selected_infusion = 0;
                            };
                        });
                    }
                },
            );
    });
}

/// Draws the customization view for adding a single item
pub(crate) fn single_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {
    if inventory_vm.add_vm.selected_header.is_none() {
        return;
    }

    egui::Frame::none().inner_margin(8.).show(ui, |ui| {
        ui.label(
            egui::RichText::new(
                inventory_vm
                    .add_vm
                    .selected_header
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .item_name
                    .as_ref()
                    .borrow()
                    .to_string(),
            )
            .strong()
            .heading()
            .size(24.),
        );
        ui.add_space(8.);

        match inventory_vm.add_vm.selected_item.clone() {
            // No customization needed for these item categories
            SelectedItem::None
            | SelectedItem::Armor(_)
            | SelectedItem::Accessory(_)
            | SelectedItem::Aow(_) => {
                return;
            }

            // Draw item customization
            SelectedItem::Item(item) => {
                let field = egui::DragValue::new(&mut inventory_vm.add_vm.selected_item_quantity)
                    .clamp_range(1..=item.as_ref().borrow().param.maxRepositoryNum);
                ui.horizontal(|ui| {
                    let label = ui.label("Quantity");
                    ui.add(field).labelled_by(label.id);
                });
            }

            // Draw weapon customization
            SelectedItem::Weapon(item) => {
                let weapon_type = item.as_ref().borrow().weapon_type();
                ui.label(egui::RichText::new(weapon_type.to_string()).size(10.));
                ui.add_space(8.);

                // Draw max quantity drag value if there's any
                let max_quantity = item.as_ref().borrow().param.maxArrowQuantity;
                if max_quantity > 1 {
                    let field =
                        egui::DragValue::new(&mut inventory_vm.add_vm.selected_item_quantity)
                            .clamp_range(1..=item.as_ref().borrow().param.maxArrowQuantity);
                    ui.horizontal(|ui| {
                        let label = ui.label("Quantity");
                        ui.add(field).labelled_by(label.id);
                    });
                }

                // Weapon customization grid
                egui::Grid::new("grid")
                    .num_columns(2)
                    .spacing([8., 8.])
                    .show(ui, |ui| {
                        // Determine max upgrade if any
                        let reinforce_type_id = item.as_ref().borrow().param.materialSetId;
                        let max_upgrade = if reinforce_type_id == -1 {
                            None
                        } else if reinforce_type_id == 2200 {
                            Some(10)
                        } else {
                            Some(25)
                        };

                        // Draw max upgrade drag value if there's any
                        if let Some(max_upgrade) = max_upgrade {
                            let field = egui::DragValue::new(
                                &mut inventory_vm.add_vm.selected_weapon_level,
                            )
                            .clamp_range(0..=max_upgrade)
                            .custom_formatter(|n, _| format!("+{}", n));
                            let label = ui.add(egui::Label::new("Weapon Level:"));
                            ui.add(field).labelled_by(label.id);
                            ui.end_row();
                        }

                        // Draw available ash of war Infusions combo box if there's any
                        if item.as_ref().borrow().is_infusable() {
                            ui.label("Ash of War:");
                            if inventory_vm.add_vm.available_infusions.len() > 0 {
                                if egui::ComboBox::new("infsuion", "")
                                    .show_index(
                                        ui,
                                        &mut inventory_vm.add_vm.selected_infusion,
                                        inventory_vm.add_vm.available_infusions.len(),
                                        |i| {
                                            inventory_vm.add_vm.available_infusions[i]
                                                .as_ref()
                                                .borrow()
                                                .item_name
                                                .as_ref()
                                                .borrow()
                                                .to_string()
                                        },
                                    )
                                    .changed()
                                {
                                    inventory_vm.add_vm.infusion_changed();
                                };
                            }
                            ui.end_row();
                        }

                        // Draw available affinity combo box if there's any
                        if inventory_vm.add_vm.selected_infusion != 0 {
                            ui.label("Affinity:");
                            egui::ComboBox::new("affinity", "").show_index(
                                ui,
                                &mut inventory_vm.add_vm.selected_affinity,
                                inventory_vm.add_vm.available_affinities.len(),
                                |i| inventory_vm.add_vm.available_affinities[i].to_string(),
                            );
                        }
                    });
            }
        }
    });

    egui::Frame::none().inner_margin(8.).show(ui, |ui| {
        ui.add_enabled_ui(true, |ui| {
            if ui
                .add_sized([ui.available_width(), 40.], egui::Button::new("Add"))
                .clicked()
            {
                // inventory_vm.add_to_inventory(&regulation_vm.selected_item);
            }
        })
    });
}

pub(crate) fn single_item_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {}

pub(crate) fn single_weapon_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {}

pub(crate) fn single_projectile_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {}
