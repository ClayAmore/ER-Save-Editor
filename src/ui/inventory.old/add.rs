
use eframe::{egui::{self, Layout, Ui, Vec2}, epaint::Color32};
use crate::{db::{
    accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME, aows::aows, armor_name::armor_name::ARMOR_NAME, armors::armor_sets, item_name::item_name::ITEM_NAME, items::items, talismans::talismans, weapon_name::weapon_name::WEAPON_NAME, weapons::weapons}, ui::custom::checkbox::checkbox::{
        three_states_checkbox, State
    }, 
    util::regulation::Regulation, 
    vm::{
        inventory::{
            InventoryTypeRoute, InventoryViewModel
        }, 
        regulation::regulation_view_model::{
            GoodsType, RegulationViewModel, WepType
        }, 
            vm::vm::ViewModel
        }
    };

pub fn add(ui: &mut Ui, vm:&mut ViewModel) {
    let regulation_vm = &mut vm.regulation;
    let inventory_vm = &mut vm.slots[vm.index].inventory_vm;

    egui::TopBottomPanel::top("top_panel").show(ui.ctx(), |ui| {
        ui.add_space(6.);
        ui.columns(2, |uis| {
            let single_button = uis[0].add_sized([100.,40.], egui::Button::new("Single"));
            let bulk_button = uis[1].add_sized([100.,40.], egui::Button::new("Bulk"));
    
            if single_button.clicked() {inventory_vm.at_single_items = true;};
            if bulk_button.clicked() {inventory_vm.at_single_items = false;};
    
            if inventory_vm.at_single_items {single_button.highlight();}
            else {bulk_button.highlight();}
        });

        ui.add_space(6.);
        ui.columns(5,|uis| {
            let common_items = uis[0].add_sized([uis[0].available_width(), 40.], egui::Button::new("Items"));
            let weapons = uis[1].add_sized([uis[1].available_width(), 40.], egui::Button::new("Weapons"));
            let armors = uis[2].add_sized([uis[2].available_width(), 40.], egui::Button::new("Armors"));
            let ashofwar = uis[3].add_sized([uis[3].available_width(), 40.], egui::Button::new("Ash of War"));
            let talismans = uis[4].add_sized([uis[4].available_width(), 40.], egui::Button::new("Talismans"));

            if common_items.clicked() {
                // Update route for single item add
                inventory_vm.current_type_route = InventoryTypeRoute::CommonItems; regulation_vm.selected_item = Default::default();
                
                // Update route for bulk item add
                inventory_vm.current_bulk_type_route = InventoryTypeRoute::CommonItems;

                // Update lists for bulk items use
                inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::CommonItems);

                // Trigger filtering
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if weapons.clicked() {
                // Update route for single item add
                inventory_vm.current_type_route = InventoryTypeRoute::Weapons; regulation_vm.selected_item = Default::default();
                
                // Update route for bulk item add
                inventory_vm.current_bulk_type_route = InventoryTypeRoute::Weapons;

                // Update lists for bulk items use
                inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::Weapons);

                // Trigger filtering
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if armors.clicked() {
                // Update route for single item add
                inventory_vm.current_type_route = InventoryTypeRoute::Armors; regulation_vm.selected_item = Default::default();
                
                // Update route for bulk item add
                inventory_vm.current_bulk_type_route = InventoryTypeRoute::Armors;

                // Update lists for bulk items use
                inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::Armors);
                
                // Trigger filtering
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if ashofwar.clicked() {
                // Update route for single item add
                inventory_vm.current_type_route = InventoryTypeRoute::AshOfWar; regulation_vm.selected_item = Default::default();
                
                // Update route for bulk item add
                inventory_vm.current_bulk_type_route = InventoryTypeRoute::AshOfWar;

                // Update lists for bulk items use
                inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::AshOfWar);
                
                // Trigger filtering
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }
            if talismans.clicked() {
                // Update route for single item add
                inventory_vm.current_type_route = InventoryTypeRoute::Talismans; regulation_vm.selected_item = Default::default();
                
                // Update route for bulk item add
                inventory_vm.current_bulk_type_route = InventoryTypeRoute::Talismans;

                // Update lists for bulk items use
                inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::Talismans);
                
                // Trigger filtering
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            }

            // Highlight active
            match inventory_vm.at_single_items {
                true => {
                    match inventory_vm.current_type_route {
                        InventoryTypeRoute::CommonItems => {common_items.highlight();},
                        InventoryTypeRoute::KeyItems => {},
                        InventoryTypeRoute::Weapons => {weapons.highlight();},
                        InventoryTypeRoute::Armors => {armors.highlight();},
                        InventoryTypeRoute::AshOfWar => {ashofwar.highlight();},
                        InventoryTypeRoute::Talismans => {talismans.highlight();},
                    }
                },
                false => {
                    match inventory_vm.current_bulk_type_route {
                        InventoryTypeRoute::CommonItems => {common_items.highlight();},
                        InventoryTypeRoute::KeyItems => {},
                        InventoryTypeRoute::Weapons => {weapons.highlight();},
                        InventoryTypeRoute::Armors => {armors.highlight();},
                        InventoryTypeRoute::AshOfWar => {ashofwar.highlight();},
                        InventoryTypeRoute::Talismans => {talismans.highlight();},
                    }
                },
            }
        });

        ui.add_space(6.);

    });

    // Side Panel
    egui::SidePanel::left("item_choice").show(ui.ctx(), |ui| {
        if inventory_vm.at_single_items {
            // Single items list view
            single(ui, regulation_vm, inventory_vm);
        }
        else {
            // Bulk items list view
            bulk(ui, inventory_vm);
        }
    });

    // Central View (Item Customization)
    egui::CentralPanel::default().show(ui.ctx(), |ui| {
        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui|{
                ui.add_space(8.);
                ui.vertical(|ui|{
                    // Single Item customization view
                    if inventory_vm.at_single_items {
                        single_item_customization(ui,  inventory_vm, regulation_vm);
                    }
                    // Bulk Item customization view
                    else {
                        bulk_item_customization(ui,  inventory_vm);
                    }
                });
            });
        });
        ui.separator();
        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
            ui.push_id("log", |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .max_height(ui.available_height()-8.)
                    .show_rows(ui, 10., inventory_vm.log.len(), |ui, row_range|{
                    for i in row_range {
                        ui.label(egui::RichText::new(&inventory_vm.log[i]).monospace().size(10.));
                    }
                });
            });
        });
    });
}

fn single(ui: &mut Ui, regulation_vm: &mut RegulationViewModel, inventory_vm: &mut InventoryViewModel) {
    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui|{
            if ui.add(egui::TextEdit::singleline(&mut inventory_vm.filter_text)).labelled_by(ui.label("Filter:").id).changed() {
                regulation_vm.filter(&inventory_vm.current_type_route, &inventory_vm.filter_text);
            };
        });
        ui.separator();
        ui.add_space(8.);
        
        let row_height = 10.;
        match inventory_vm.current_type_route {
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
}

fn bulk(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {
    ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
        ui.add_space(8.);

        ui.horizontal(|ui| {
            let is_all_selected = inventory_vm.bulk_items_selected.iter().all(|map| map.values().all(|on|*on));
            let is_any_selected = inventory_vm.bulk_items_selected.iter().any(|map| map.values().any(|on|*on));
            let state = if is_all_selected {State::On} else if is_any_selected {State::InBetween} else {State::Off};
            if three_states_checkbox(ui, &state).clicked(){
                match state {
                    State::On => inventory_vm.bulk_items_selected.iter_mut().for_each(|map| map.values_mut().for_each(|selected| *selected = false)),
                    State::Off => inventory_vm.bulk_items_selected.iter_mut().for_each(|map| map.values_mut().for_each(|selected| *selected = true)),
                    State::InBetween => inventory_vm.bulk_items_selected.iter_mut().for_each(|map| map.values_mut().for_each(|selected| *selected = true)),
                }
            };
            ui.label("Select All");
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .max_height(ui.available_height()-8.)
            .show(ui, |ui|{
            match inventory_vm.current_bulk_type_route {
                InventoryTypeRoute::KeyItems |
                InventoryTypeRoute::CommonItems => {
                    for (index, (group_name, items)) in items().iter().enumerate() {
                        ui.horizontal(|ui| {
                            select_all_sub_group_checkbox(ui, inventory_vm, index);
                            ui.collapsing(group_name, |ui| {
                                for item in items {
                                    ui.checkbox(&mut inventory_vm.bulk_items_selected[index].get_mut(&item).unwrap(), ITEM_NAME.lock().unwrap()[&(item ^ 0x40000000)]);
                                }
                            });
                        });
                    }
                },
                InventoryTypeRoute::Weapons => {
                    for (index, (group_name, weapons)) in weapons().iter().enumerate() {
                        ui.horizontal(|ui| {
                            select_all_sub_group_checkbox(ui, inventory_vm, index);
                            ui.collapsing(group_name, |ui| {
                                for weapon in weapons {
                                    ui.checkbox(&mut inventory_vm.bulk_items_selected[index].get_mut(&weapon).unwrap(), WEAPON_NAME.lock().unwrap()[&weapon]);
                                }
                            });
                        });
                    }
                },
                InventoryTypeRoute::Armors => {
                    for (index, (group_name, armor_sets)) in armor_sets().iter().enumerate() {
                        ui.horizontal(|ui| {
                            select_all_sub_group_checkbox(ui, inventory_vm, index);
                            ui.collapsing(group_name, |ui| {
                                for armor in armor_sets {
                                    ui.checkbox(&mut inventory_vm.bulk_items_selected[index].get_mut(&armor).unwrap(), ARMOR_NAME.lock().unwrap()[&(armor ^ 0x10000000)]);
                                }
                            });
                        });
                    }
                },
                InventoryTypeRoute::AshOfWar => {
                    for (index, (_, aows)) in aows().iter().enumerate() {
                        ui.vertical(|ui| {
                            for aow in aows {
                                ui.checkbox(&mut inventory_vm.bulk_items_selected[index].get_mut(&aow).unwrap(), AOW_NAME.lock().unwrap()[&(aow ^ 0x80000000)]);
                            }
                        });
                    }
                },
                InventoryTypeRoute::Talismans => {
                    for (index, (_, talismans)) in talismans().iter().enumerate() {
                        ui.vertical(|ui| {
                            for talisman in talismans {
                                ui.checkbox(&mut inventory_vm.bulk_items_selected[index].get_mut(&talisman).unwrap(), ACCESSORY_NAME.lock().unwrap()[&(talisman ^ 0x20000000)]);
                            }
                        });
                    }
                },
            };
        });
    });
}

fn single_item_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel, regulation_vm: &mut RegulationViewModel) {
    if !regulation_vm.selected_item.name.is_empty() {
        egui::Frame::none().inner_margin(8.).show(ui, |ui|{
            ui.label(egui::RichText::new(regulation_vm.selected_item.name.to_string()).strong().heading().size(24.));
            ui.add_space(8.);

            match inventory_vm.current_type_route {
                InventoryTypeRoute::CommonItems | InventoryTypeRoute::KeyItems => {
                    let res = Regulation::equip_goods_param_map().get(&regulation_vm.selected_item.id);
                    if res.is_some() {
                        let item = res.unwrap();
                        let goods_type = GoodsType::from(item.data.goodsType);
                        let max_repository_num = if goods_type == GoodsType::KeyItem {item.data.maxNum} else {item.data.maxRepositoryNum};
                        let field = egui::DragValue::new(regulation_vm.selected_item.quantity.as_mut().unwrap()).clamp_range(1..=max_repository_num);
                        ui.horizontal(|ui|{
                            let label = ui.label("Quantity");
                            ui.add(field).labelled_by(label.id);
                        });
                    }
                }
                InventoryTypeRoute::Weapons => {
                    egui::Grid::new("grid").num_columns(2).spacing([8., 8.]).show(ui,|ui| {
                        let res = Regulation::equip_weapon_params_map().get(&regulation_vm.selected_item.id);
                        if res.is_some() {
                            let item = res.unwrap();
                            let wep_type = WepType::from(item.data.wepType);
                        
                            if wep_type == WepType::Arrow || wep_type == WepType::Greatarrow || wep_type == WepType::Bolt || wep_type == WepType::BallistaBolt  {
                                ui.horizontal(|ui|{
                                    let field = egui::DragValue::new(regulation_vm.selected_item.quantity.as_mut().unwrap()).clamp_range(1..=item.data.maxArrowQuantity);
                                    let label = ui.label("Quantity");
                                    ui.add(field).labelled_by(label.id);
                                });
                            }
                            else {
                                let max_upgrade = if item.data.reinforceTypeId != 0 &&(
                                    item.data.reinforceTypeId % 2200 == 0 ||
                                    item.data.reinforceTypeId % 2400 == 0 ||
                                    item.data.reinforceTypeId % 3200 == 0 ||
                                    item.data.reinforceTypeId % 3300 == 0 ||
                                    item.data.reinforceTypeId % 8300 == 0 ||
                                    item.data.reinforceTypeId % 8500 == 0) {10} else {25};
                                let field = egui::DragValue::new(regulation_vm.selected_item.upgrade.as_mut().unwrap())
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
                InventoryTypeRoute::Armors|
                InventoryTypeRoute::AshOfWar |
                InventoryTypeRoute::Talismans => {},
            };
        });
        
        egui::Frame::none().inner_margin(8.).show(ui, |ui|{
            ui.add_enabled_ui(true, |ui| {
                if ui.add_sized([ui.available_width(), 40.], egui::Button::new("Add")).clicked() {
                    inventory_vm.add_to_inventory(&regulation_vm.selected_item);
                }
            })
        });
    }
}

fn bulk_item_customization(ui: &mut Ui, inventory_vm: &mut InventoryViewModel) {
    egui::Frame::none().inner_margin(8.).show(ui, |ui|{
        ui.label(egui::RichText::new("Customize").strong().heading().size(24.));
        ui.add_space(6.);
        match inventory_vm.current_bulk_type_route {
            InventoryTypeRoute::CommonItems | InventoryTypeRoute::KeyItems => {
                ui.add(egui::Checkbox::new(&mut inventory_vm.bulk_items_max_quantity, "Max Quantity"));
            }
            InventoryTypeRoute::Weapons => {
                egui::Grid::new("bulk_items_customization").spacing(Vec2::new(6., 6.)).show(ui,|ui| {
                    let field = egui::DragValue::new(&mut inventory_vm.bulk_items_arrow_quantity).clamp_range(1..=99);
                    let label = ui.label("Projectile Quantity");
                    ui.add(field).labelled_by(label.id);
                    ui.end_row();

                    let label = ui.label("Weapon Level:");
                    ui.add(egui::DragValue::new(&mut inventory_vm.bulk_items_weapon_level).clamp_range(0..=25).custom_formatter(|val, _| {
                        let somber_upgrade_level: f64 = (val + 0.5)/2.5;
                        format!("Normal: +{}\t Somber: +{}", val as u32, somber_upgrade_level as u32)
                    })).labelled_by(label.id);
                    ui.end_row();
                });
            },
            InventoryTypeRoute::Armors|
            InventoryTypeRoute::AshOfWar |
            InventoryTypeRoute::Talismans => {},
        };
    });
        
    egui::Frame::none().inner_margin(8.).show(ui, |ui|{
        ui.add_enabled_ui(true, |ui| {
            if ui.add_sized([ui.available_width(), 40.], egui::Button::new("Add All")).clicked() {
                inventory_vm.add_all_to_inventory();
            }
        })
    });
}

fn select_all_sub_group_checkbox(ui:&mut Ui, inventory_vm: &mut InventoryViewModel, index: usize) {
    if inventory_vm.bulk_items_selected.is_empty() {return;}
    let is_all_selected = inventory_vm.bulk_items_selected[index].values().all(|on|*on);
    let is_any_selected = inventory_vm.bulk_items_selected[index].values().any(|on|*on);
    let state = if is_all_selected {State::On} else if is_any_selected {State::InBetween} else {State::Off};
    if three_states_checkbox(ui, &state).clicked(){
        match state {
            State::On => inventory_vm.bulk_items_selected[index].values_mut().for_each(|selected| *selected = false),
            State::Off => inventory_vm.bulk_items_selected[index].values_mut().for_each(|selected| *selected = true),
            State::InBetween => inventory_vm.bulk_items_selected[index].values_mut().for_each(|selected| *selected = true),
        }
    };
}