use eframe::{egui::{self, Layout, Ui}, epaint::Color32};
use crate::vm::{inventory::inventory::InventoryTypeRoute, regulation::regulation_view_model::WepType, vm::vm::ViewModel};

pub fn add_inventory(ui: &mut Ui, vm:&mut ViewModel) {
    let regulation_vm = &mut vm.regulation;
    let inventory_vm = &mut vm.slots[vm.index].inventory_vm;

    egui::TopBottomPanel::top("top_panel").show(ui.ctx(), |ui| {
        ui.add_space(6.);
        ui.columns(5,|uis| {
            let common_items = uis[0].add_sized([uis[0].available_width(), 40.], egui::Button::new("Items"));
            let weapons = uis[1].add_sized([uis[1].available_width(), 40.], egui::Button::new("Weapons"));
            let armors = uis[2].add_sized([uis[2].available_width(), 40.], egui::Button::new("Armors"));
            let ashofwar = uis[3].add_sized([uis[3].available_width(), 40.], egui::Button::new("Ash of War"));
            let talismans = uis[4].add_sized([uis[4].available_width(), 40.], egui::Button::new("Talismans"));

            if common_items.clicked() {
                inventory_vm.current_type_route = InventoryTypeRoute::CommonItems; regulation_vm.selected_item = Default::default();
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if weapons.clicked() {
                inventory_vm.current_type_route = InventoryTypeRoute::Weapons; regulation_vm.selected_item = Default::default();
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if armors.clicked() {
                inventory_vm.current_type_route = InventoryTypeRoute::Armors; regulation_vm.selected_item = Default::default();
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if ashofwar.clicked() {
                inventory_vm.current_type_route = InventoryTypeRoute::AshOfWar; regulation_vm.selected_item = Default::default();
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if talismans.clicked() {
                inventory_vm.current_type_route = InventoryTypeRoute::Talismans; regulation_vm.selected_item = Default::default();
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }

            // Highlight active 
            match inventory_vm.current_type_route {
                InventoryTypeRoute::None => {},
                InventoryTypeRoute::CommonItems => {common_items.highlight();},
                InventoryTypeRoute::KeyItems => {},
                InventoryTypeRoute::Weapons => {weapons.highlight();},
                InventoryTypeRoute::Armors => {armors.highlight();},
                InventoryTypeRoute::AshOfWar => {ashofwar.highlight();},
                InventoryTypeRoute::Talismans => {talismans.highlight();},
            }
        });

        ui.add_space(6.);

    });
    
    // Side Panel
    egui::SidePanel::left("item_choice").show(ui.ctx(), |ui| {
        ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
            ui.add_space(8.);
            ui.horizontal(|ui|{
                if ui.add(egui::widgets::TextEdit::singleline(&mut inventory_vm.filter_text)).labelled_by(ui.label("Filter:").id).changed() {
                    regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
                };
            });
            ui.separator();
            ui.add_space(8.);
            
            let row_height = 10.;
            match inventory_vm.current_type_route {
                InventoryTypeRoute::None =>  {
                },
                InventoryTypeRoute::CommonItems => {
                    egui::ScrollArea::vertical().max_height(ui.available_height()-8.).show_rows(ui, row_height, regulation_vm.filtered_goods.len(), |ui, row_range|{
                        for i in row_range {
                            let item = &regulation_vm.filtered_goods[i];
                            let mut text = egui::RichText::new(format!("{}", item.name));
                            if regulation_vm.selected_item.id == item.id {
                                text = egui::RichText::new(format!("{}", item.name)).strong().heading();
                            }
                            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                                if ui.add(egui::Button::new(text).fill(Color32::TRANSPARENT)).clicked() {
                                    regulation_vm.selected_item = item.clone();
                                };
                            });
                        }
                    });
                },
                InventoryTypeRoute::KeyItems => {
                }, 
                InventoryTypeRoute::Weapons => {
                    egui::ScrollArea::vertical().max_height(ui.available_height()-8.).show_rows(ui, row_height, regulation_vm.filtered_weapons.len(), |ui, row_range|{
                        for i in row_range {
                            let mut text = egui::RichText::new(format!("{}", &regulation_vm.filtered_weapons[i].name));
                            if regulation_vm.selected_item.id == regulation_vm.filtered_weapons[i].id {
                                text = egui::RichText::new(format!("{}", &regulation_vm.filtered_weapons[i].name)).strong().heading();
                            }
                            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                                if ui.add(egui::Button::new(text).fill(Color32::TRANSPARENT)).clicked() {
                                    regulation_vm.selected_item = regulation_vm.filtered_weapons[i].clone();
                                    regulation_vm.update_available_infusions();
                                    regulation_vm.update_available_affinities();
                                };
                            });
                        }
                    });
                },
                InventoryTypeRoute::Armors => {
                    egui::ScrollArea::vertical().max_height(ui.available_height()-8.).show_rows(ui, row_height, regulation_vm.filtered_protectors.len(), |ui, row_range|{
                        for i in row_range {
                            let item = &regulation_vm.filtered_protectors[i];
                            let mut text = egui::RichText::new(format!("{}", item.name));
                            if regulation_vm.selected_item.id == item.id {
                                text = egui::RichText::new(format!("{}", item.name)).strong().heading();
                            }
                            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                                if ui.add(egui::Button::new(text).fill(Color32::TRANSPARENT)).clicked() {
                                    regulation_vm.selected_item = item.clone();
                                };
                            });
                        }
                    });
                },
                InventoryTypeRoute::AshOfWar => {
                    egui::ScrollArea::vertical().max_height(ui.available_height()-8.).show_rows(ui, row_height, regulation_vm.filtered_gems.len(), |ui, row_range|{
                        for i in row_range {
                            let item = &regulation_vm.filtered_gems[i];
                            let mut text = egui::RichText::new(format!("{}", item.name));
                            if regulation_vm.selected_item.id == item.id {
                                text = egui::RichText::new(format!("{}", item.name)).strong().heading();
                            }
                            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                                if ui.add(egui::Button::new(text).fill(Color32::TRANSPARENT)).clicked() {
                                    regulation_vm.selected_item = item.clone();
                                };
                            });
                        }
                    });
                },
                InventoryTypeRoute::Talismans => {
                    egui::ScrollArea::vertical().max_height(ui.available_height()-8.).show_rows(ui, row_height, regulation_vm.filtered_accessories.len(), |ui, row_range|{
                        for i in row_range {
                            let item = &regulation_vm.filtered_accessories[i];
                            let mut text = egui::RichText::new(format!("{}", item.name));
                            if regulation_vm.selected_item.id == item.id {
                                text = egui::RichText::new(format!("{}", item.name)).strong().heading();
                            }
                            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                                if ui.add(egui::Button::new(text).fill(Color32::TRANSPARENT)).clicked() {
                                    regulation_vm.selected_item = item.clone();
                                };
                            });
                        }
                    });
                },
            };
        });
    });

    // Central View
    egui::CentralPanel::default().show(ui.ctx(), |ui| {
        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui|{
                ui.add_space(8.);
                ui.vertical(|ui|{
                    if !regulation_vm.selected_item.name.is_empty() {
                        egui::Frame::none().show(ui, |ui| {
                            ui.label(egui::RichText::new(regulation_vm.selected_item.name.to_string()).strong().heading().size(24.));
                            ui.add_space(8.);
                        });
                        match inventory_vm.current_type_route {
                            InventoryTypeRoute::None =>  {
                            },
                            InventoryTypeRoute::CommonItems | InventoryTypeRoute::KeyItems => {
                                let res = regulation_vm.full_goods.rows.iter().find(|p| p.id == regulation_vm.selected_item.id as i32);
                                if res.is_some() {
                                    let item = res.unwrap();
                                    let field = egui::widgets::DragValue::new(regulation_vm.selected_item.quantity.as_mut().unwrap()).clamp_range(1..=item.data.maxRepositoryNum);
                                    ui.horizontal(|ui|{
                                        let label = ui.label("Quantity");
                                        ui.add(field).labelled_by(label.id);
                                    });
                                }
                            }
                            InventoryTypeRoute::Weapons => {
                                egui::Grid::new("grid").num_columns(2).spacing([8., 8.]).show(ui,|ui| {
                                    let res = regulation_vm.full_weapons.rows.iter().find(|p| p.id == regulation_vm.selected_item.id as i32);
                                    if res.is_some() {
                                        let item = res.unwrap();
                                        let wep_type = WepType::from(item.data.wepType);
                                    
                                        if wep_type == WepType::Arrow || wep_type == WepType::Greatarrow || wep_type == WepType::Bolt || wep_type == WepType::BallistaBolt  {
                                            ui.horizontal(|ui|{
                                                let field = egui::widgets::DragValue::new(regulation_vm.selected_item.quantity.as_mut().unwrap()).clamp_range(1..=item.data.maxArrowQuantity);
                                                let label = ui.label("Quantity");
                                                ui.add(field).labelled_by(label.id);
                                            });
                                        }
                                        else {
                                            let max_upgrade = if item.data.materialSetId != 0 && item.data.materialSetId % 2200 == 0 {10} else {25};
                                            let field = egui::widgets::DragValue::new(regulation_vm.selected_item.upgrade.as_mut().unwrap())
                                            .clamp_range(0..=max_upgrade)
                                            .custom_formatter(|n, _| {
                                                format!("+{}", n)
                                            });
                                            let label = ui.add(egui::Label::new("Weapon Level:"));
                                            ui.add(field).labelled_by(label.id);
                                            ui.end_row();
    
                                            if regulation_vm.available_infusions.len() > 0 {
                                                ui.add(egui::Label::new("Infusion:"));
                                                if egui::ComboBox::new("infsuion", "")
                                                    .show_index(ui, &mut regulation_vm.selected_infusion, regulation_vm.available_infusions.len(), |i|{
                                                    regulation_vm.available_infusions[i].name.to_string()
                                                }).changed() {
                                                    regulation_vm.update_available_affinities();
                                                    regulation_vm.selected_item.infusion = Some(regulation_vm.available_infusions[regulation_vm.selected_infusion].id);
                                                };
                                                ui.end_row();
                                            }
    
                                            if regulation_vm.available_affinities.len() > 0 {
                                                ui.add(egui::Label::new("Affintiy:"));
                                                if egui::ComboBox::new("affinity", "")
                                                .show_index(ui, &mut regulation_vm.selected_affinity, regulation_vm.available_affinities.len(), |i|{
                                                    regulation_vm.available_affinities[i].to_string()
                                                }).changed() {
                                                    regulation_vm.selected_item.affinity = Some(regulation_vm.available_affinities[regulation_vm.selected_affinity].as_i16());
                                                };
                                                ui.end_row();
                                            }
                                        }
                                    }
                                });
                            },
                            InventoryTypeRoute::Armors => {
                            },
                            InventoryTypeRoute::AshOfWar => {
                            },
                            InventoryTypeRoute::Talismans => {
                            },
                        };
                        
                        egui::TopBottomPanel::bottom("add_button").show(ui.ctx(), |ui|{
                            egui::Frame::none().inner_margin(8.).show(ui, |ui|{
                                ui.add_enabled_ui(false, |ui| {
                                    if ui.add_sized([ui.available_width(), 40.], egui::Button::new("Add")).clicked() {
                                        inventory_vm.add_item(&regulation_vm.selected_item);
                                    }
                                })
                            });
                        });
                    }
                });
            });
        });
    });
}