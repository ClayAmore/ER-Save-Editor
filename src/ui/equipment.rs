// Todo: Rewrite this to work with only equip_index.
pub mod equipment {

    use eframe::egui::{self, Color32, Layout, Ui, Vec2};

    use crate::vm::{inventory::InventorySubTypeRoute, vm::vm::ViewModel};

    const EQUIPMENT_BOX_WIDTH: f32 = 110.; 

    pub fn equipment(ui: &mut Ui, vm: &mut ViewModel) {
        egui::SidePanel::right("equipment_list").show(ui.ctx(), |ui| {
            side_panel(ui, vm);
        });

        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                row(ui, vm, format!("Weapons - Right Hand"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..3 {
                            combo_box( ui, vm, index, InventorySubTypeRoute::WeaponRight);
                        }
                    });
                });
                row(ui, vm, format!("Weapons - Left Hand"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..3 {
                            combo_box( ui, vm, index, InventorySubTypeRoute::WeaponLeft);
                        }
                    });
                });
    
                row(ui, vm, format!("Arrows"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..2 {
                            combo_box(ui,vm, index, InventorySubTypeRoute::Arrow);
                        }
                    });
                });
                
    
                row(ui, vm, format!("Bolts"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..2 {
                            combo_box(ui, vm, index, InventorySubTypeRoute::Bolt);
                        }
                    });
                });
                
                row(ui, vm, format!("Armor"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        combo_box(ui, vm, 0, InventorySubTypeRoute::Head);
                        combo_box(ui, vm, 0, InventorySubTypeRoute::Body);
                        combo_box(ui, vm, 0, InventorySubTypeRoute::Arms);
                        combo_box(ui, vm, 0, InventorySubTypeRoute::Legs);
                    });
                });
                
                row(ui, vm, format!("Talismans"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..vm.slots[vm.index].equipment_vm.talisman_count as usize {
                            combo_box(ui, vm, index, InventorySubTypeRoute::Talisman);
                        }
                    });
                });
                
                row(ui, vm, format!("Quickslots"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..5 {
                            combo_box(ui, vm, index, InventorySubTypeRoute::QuickItem);
                        }
                    });
                });
                row(ui, vm, format!(""), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 5..10 {
                            combo_box(ui, vm, index, InventorySubTypeRoute::QuickItem);
                        }
                    });
                });
    
                row(ui, vm, format!("Pouch"), |ui, vm| {
                    group(ui, vm, |ui, vm| {
                        for index in 0..6 {
                            combo_box(ui, vm, index, InventorySubTypeRoute::Pouch);
                        }
                    });
                });
            });
        });
    }

    fn row(ui: &mut Ui, vm: &mut ViewModel, heading: String, f: fn(&mut Ui, &mut ViewModel) -> ()) {
        if !heading.is_empty() {
            ui.heading(egui::RichText::new(heading));
            ui.add_space(4.);
        };
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            f(ui, vm);
        });
        ui.add_space(16.);
    }

    fn group(ui: &mut Ui, vm: &mut ViewModel, f: fn(&mut Ui, &mut ViewModel) -> ()) {
        egui::Frame::none().show(ui, |ui| {
            ui.horizontal_wrapped(|ui|{
                f(ui, vm);
            });
        });
    }

    fn combo_box(ui: &mut Ui, vm: &mut ViewModel, index: usize, r#type: InventorySubTypeRoute) {
        let inventory_vm = &mut vm.slots[vm.index].inventory_vm;
        let equipment_vm = &mut vm.slots[vm.index].equipment_vm;
        ui.allocate_ui_with_layout(Vec2::new(EQUIPMENT_BOX_WIDTH, ui.available_height()), Layout::top_down(egui::Align::Center), |ui| {
            ui.group(|ui| {
                ui.add_space(8.);
                ui.label(match r#type {
                    InventorySubTypeRoute::None => egui::RichText::new(""),
                    InventorySubTypeRoute::WeaponLeft => egui::RichText::new(format!("WeaponLeft{}", index+1)).strong(),
                    InventorySubTypeRoute::WeaponRight => egui::RichText::new(format!("WeaponRight{}", index+1)).strong(),
                    InventorySubTypeRoute::Head => egui::RichText::new(format!("Head{}", index+1)).strong(),
                    InventorySubTypeRoute::Body => egui::RichText::new(format!("Body{}", index+1)).strong(),
                    InventorySubTypeRoute::Arms => egui::RichText::new(format!("Arms{}", index+1)).strong(),
                    InventorySubTypeRoute::Legs => egui::RichText::new(format!("Legs{}", index+1)).strong(),
                    InventorySubTypeRoute::Arrow => egui::RichText::new(format!("Arrow{}", index+1)).strong(),
                    InventorySubTypeRoute::Bolt => egui::RichText::new(format!("Bolt{}", index+1)).strong(),
                    InventorySubTypeRoute::Talisman => egui::RichText::new(format!("Talisman{}", index+1)).strong(),
                    InventorySubTypeRoute::QuickItem => egui::RichText::new(format!("Quickitem{}", index+1)).strong(),
                    InventorySubTypeRoute::Pouch => egui::RichText::new(format!("Pouch{}", index+1)).strong(),
                });
                ui.separator();
                ui.label(&format!("{:<32}", match r#type {
                    InventorySubTypeRoute::None => "",
                    InventorySubTypeRoute::WeaponLeft => &equipment_vm.left_hand_armaments[index].name,
                    InventorySubTypeRoute::WeaponRight => &equipment_vm.right_hand_armaments[index].name,
                    InventorySubTypeRoute::Head => &equipment_vm.head.name,
                    InventorySubTypeRoute::Body => &equipment_vm.chest.name,
                    InventorySubTypeRoute::Arms => &equipment_vm.arms.name,
                    InventorySubTypeRoute::Legs => &equipment_vm.legs.name,
                    InventorySubTypeRoute::Arrow => &equipment_vm.arrows[index].name,
                    InventorySubTypeRoute::Bolt => &equipment_vm.bolts[index].name,
                    InventorySubTypeRoute::Talisman => &equipment_vm.talismans[index].name,
                    InventorySubTypeRoute::QuickItem => &equipment_vm.quickitems[index].name,
                    InventorySubTypeRoute::Pouch => &equipment_vm.pouch[index].name,
                }));
                ui.separator();
                if ui.button("Change").clicked() {
                    inventory_vm.current_subtype_route = r#type;
                    equipment_vm.current_index = index;
                    inventory_vm.filter_with_subtype();
                    equipment_vm.current_equipped_items = match &inventory_vm.current_subtype_route {
                        InventorySubTypeRoute::None => {Vec::new()},
                        InventorySubTypeRoute::WeaponLeft |
                        InventorySubTypeRoute::WeaponRight => {
                            let mut weapons = Vec::from_iter(equipment_vm.left_hand_armaments.iter().map(|i| i.equip_index));
                            weapons.extend(equipment_vm.right_hand_armaments.iter().map(|i| i.equip_index));
                            weapons
                        },
                        InventorySubTypeRoute::Head => {vec![equipment_vm.head.equip_index]},
                        InventorySubTypeRoute::Body => {vec![equipment_vm.arms.equip_index]},
                        InventorySubTypeRoute::Arms => {vec![equipment_vm.chest.equip_index]},
                        InventorySubTypeRoute::Legs => {vec![equipment_vm.legs.equip_index]},
                        InventorySubTypeRoute::Arrow => {Vec::from_iter(equipment_vm.arrows.iter().map(|i| i.equip_index))},
                        InventorySubTypeRoute::Bolt => {Vec::from_iter(equipment_vm.bolts.iter().map(|i| i.equip_index))},
                        InventorySubTypeRoute::Talisman => {Vec::from_iter(equipment_vm.talismans.iter().map(|i| i.equip_index))},
                        InventorySubTypeRoute::QuickItem |
                        InventorySubTypeRoute::Pouch => {
                            let mut items = Vec::from_iter(equipment_vm.quickitems.iter().map(|i| i.equip_index));
                            items.extend(equipment_vm.pouch.iter().map(|i| i.equip_index));
                            items
                        },
                    };
                }
                ui.add_space(ui.available_height());
            });
        });
    } 

    fn side_panel(ui: &mut Ui, vm: &mut ViewModel) {
        let inventory_vm = &mut vm.slots[vm.index].inventory_vm;
        let equipment_vm = &mut vm.slots[vm.index].equipment_vm;
        let empty = Vec::new();
        ui.add_space(8.);
        match inventory_vm.current_subtype_route {
            InventorySubTypeRoute::None => {},
            InventorySubTypeRoute::WeaponLeft => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("WeaponLeft{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::WeaponRight => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("WeaponRight{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Head => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Head{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Body => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Body{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Arms => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Arms{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Legs => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Legs{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Arrow => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Arrow{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Bolt => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Bolt{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Talisman => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Talisman{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::QuickItem => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Quickitem{}", equipment_vm.current_index+1)).strong()); 
                });
            },
            InventorySubTypeRoute::Pouch => {
                ui.vertical_centered_justified(|ui| { 
                    ui.heading(egui::RichText::new(format!("Pouch{}", equipment_vm.current_index+1)).strong()); 
                });
            },
        };
        ui.add_space(8.);
        ui.horizontal(|ui|{
            let label = ui.label("Filter:");
            if ui.add(egui::widgets::TextEdit::singleline(&mut inventory_vm.filter_text)).labelled_by(label.id).changed() {
                inventory_vm.filter_with_subtype();
            };
        });
        ui.separator();
        ui.add_space(8.);
        let current_inventory_list = match &inventory_vm.current_subtype_route {
            InventorySubTypeRoute::None => {&empty},
            InventorySubTypeRoute::WeaponLeft => {&inventory_vm.storage[0].filtered_weapons},
            InventorySubTypeRoute::WeaponRight => {&inventory_vm.storage[0].filtered_weapons},
            InventorySubTypeRoute::Head => {&inventory_vm.storage[0].filtered_armors},
            InventorySubTypeRoute::Body => {&inventory_vm.storage[0].filtered_armors},
            InventorySubTypeRoute::Arms => {&inventory_vm.storage[0].filtered_armors},
            InventorySubTypeRoute::Legs => {&inventory_vm.storage[0].filtered_armors},
            InventorySubTypeRoute::Arrow => {&inventory_vm.storage[0].filtered_weapons},
            InventorySubTypeRoute::Bolt => {&inventory_vm.storage[0].filtered_weapons},
            InventorySubTypeRoute::Talisman => {&inventory_vm.storage[0].filtered_accessories},
            InventorySubTypeRoute::QuickItem |
            InventorySubTypeRoute::Pouch => {&inventory_vm.storage[0].filtered_items},
        };
        if current_inventory_list.is_empty() {
            ui.vertical_centered_justified(|ui| {
                ui.label("Empty!");
            });
        }
        else {
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(ui, 10., current_inventory_list.len(), |ui, row_range| {
                egui::Grid::new("availbale_equipment_list").striped(true).show(ui, |ui|{
                    for i in row_range {
                        let item = &current_inventory_list[i];
                        if item.item_id == inventory_vm.unarmed.item_id {continue;} // Skip showing "unarmed" weapon which is the weapon used when weapon slot is empty.
                        let item_name_ui = egui::RichText::new(item.item_name.to_string());
                        let is_current_item = item.equip_index == match &inventory_vm.current_subtype_route {
                            InventorySubTypeRoute::None => {0},
                            InventorySubTypeRoute::WeaponLeft => {
                                equipment_vm.left_hand_armaments[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::WeaponRight => {
                                equipment_vm.right_hand_armaments[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::Head => {
                                equipment_vm.head.equip_index
                            },
                            InventorySubTypeRoute::Body => {
                                equipment_vm.chest.equip_index
                            },
                            InventorySubTypeRoute::Arms => {
                                equipment_vm.arms.equip_index
                            },
                            InventorySubTypeRoute::Legs => {
                                equipment_vm.legs.equip_index
                            },
                            InventorySubTypeRoute::Arrow => {
                                equipment_vm.arrows[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::Bolt => {
                                equipment_vm.bolts[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::Talisman => {
                                equipment_vm.talismans[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::QuickItem => {
                                equipment_vm.quickitems[equipment_vm.current_index].equip_index
                            },
                            InventorySubTypeRoute::Pouch => {
                                equipment_vm.pouch[equipment_vm.current_index].equip_index
                            },
                        };
                        equipment_vm.quickitems[equipment_vm.current_index].equip_index;
                        let is_equipped_item = equipment_vm.current_equipped_items.iter().any(|id| *id == item.equip_index);
                        ui.label(if is_current_item {item_name_ui} else {item_name_ui});
                        if is_current_item{
                            if inventory_vm.current_subtype_route == InventorySubTypeRoute::Head && inventory_vm.naked_head.ga_item_handle == 0 {
                                ui.label("Equipped");
                            }   
                            else if inventory_vm.current_subtype_route == InventorySubTypeRoute::Body && inventory_vm.naked_head.ga_item_handle == 0 {
                                ui.label("Equipped");
                            }
                            else if inventory_vm.current_subtype_route == InventorySubTypeRoute::Arms && inventory_vm.naked_head.ga_item_handle == 0 {
                                ui.label("Equipped");
                            }
                            else if inventory_vm.current_subtype_route == InventorySubTypeRoute::Legs && inventory_vm.naked_head.ga_item_handle == 0 {
                                ui.label("Equipped");
                            }
                            else if ui.add_sized([100., 24.], egui::Button::new("Unequip").fill(Color32::LIGHT_RED)).clicked() {

                                // Mark this section as changed so when the file is 
                                // saved then it will write this section to the file
                                equipment_vm.changed = true;

                                match &inventory_vm.current_subtype_route {
                                    InventorySubTypeRoute::None => {},
                                    InventorySubTypeRoute::WeaponLeft => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.left_hand_armaments[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].gaitem_handle = inventory_vm.unarmed.ga_item_handle;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].id = inventory_vm.unarmed.item_id;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].equip_index = inventory_vm.unarmed.equip_index;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].name = "Unarmed".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::WeaponRight => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.right_hand_armaments[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].gaitem_handle = inventory_vm.unarmed.ga_item_handle;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].id = inventory_vm.unarmed.item_id;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].equip_index = inventory_vm.unarmed.equip_index;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].name = "Unarmed".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::Head => {
                                        if inventory_vm.naked_head.ga_item_handle != 0 {
                                            equipment_vm.head.gaitem_handle = inventory_vm.naked_head.ga_item_handle;
                                            equipment_vm.head.id = inventory_vm.naked_head.item_id;
                                            equipment_vm.head.equip_index = inventory_vm.naked_head.equip_index;
                                            equipment_vm.head.name = "Empty".to_string();
                                            equipment_vm.current_equipped_items[0] = 0;
                                        }
                                    },
                                    InventorySubTypeRoute::Body => {
                                        if inventory_vm.naked_body.ga_item_handle != 0 {
                                            equipment_vm.chest.gaitem_handle = inventory_vm.naked_body.ga_item_handle;
                                            equipment_vm.chest.id = inventory_vm.naked_body.item_id;
                                            equipment_vm.chest.equip_index = inventory_vm.naked_body.equip_index;
                                            equipment_vm.chest.name = "Empty".to_string();
                                            equipment_vm.current_equipped_items[0] = 0;
                                        }
                                    },
                                    InventorySubTypeRoute::Arms => {
                                        if inventory_vm.naked_arms.ga_item_handle != 0 {
                                            equipment_vm.arms.gaitem_handle = inventory_vm.naked_arms.ga_item_handle;
                                            equipment_vm.arms.id = inventory_vm.naked_arms.item_id;
                                            equipment_vm.arms.equip_index = inventory_vm.naked_arms.equip_index;
                                            equipment_vm.arms.name = "Empty".to_string();
                                            equipment_vm.current_equipped_items[0] = 0;
                                        }
                                    },
                                    InventorySubTypeRoute::Legs => {
                                        if inventory_vm.naked_legs.ga_item_handle != 0 {
                                            equipment_vm.legs.gaitem_handle = inventory_vm.naked_legs.ga_item_handle;
                                            equipment_vm.legs.id = inventory_vm.naked_legs.item_id;
                                            equipment_vm.legs.equip_index = inventory_vm.naked_legs.equip_index;
                                            equipment_vm.legs.name = "Empty".to_string();
                                            equipment_vm.current_equipped_items[0] = 0;
                                        }
                                    },
                                    InventorySubTypeRoute::Arrow => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.arrows[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.arrows[equipment_vm.current_index].gaitem_handle = 0;
                                        equipment_vm.arrows[equipment_vm.current_index].id = 0;
                                        equipment_vm.arrows[equipment_vm.current_index].equip_index = 0;
                                        equipment_vm.arrows[equipment_vm.current_index].name = "Empty".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::Bolt => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.bolts[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.bolts[equipment_vm.current_index].gaitem_handle = 0;
                                        equipment_vm.bolts[equipment_vm.current_index].id = 0;
                                        equipment_vm.bolts[equipment_vm.current_index].equip_index = 0;
                                        equipment_vm.bolts[equipment_vm.current_index].name = "Empty".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::Talisman => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.talismans[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.talismans[equipment_vm.current_index].gaitem_handle = 0;
                                        equipment_vm.talismans[equipment_vm.current_index].id = 0;
                                        equipment_vm.talismans[equipment_vm.current_index].equip_index = 0;
                                        equipment_vm.talismans[equipment_vm.current_index].name = "Empty".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::QuickItem => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.quickitems[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.quickitems[equipment_vm.current_index].gaitem_handle = 0;
                                        equipment_vm.quickitems[equipment_vm.current_index].id = 0;
                                        equipment_vm.quickitems[equipment_vm.current_index].equip_index = 0;
                                        equipment_vm.quickitems[equipment_vm.current_index].name = "Empty".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                    InventorySubTypeRoute::Pouch => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.pouch[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.pouch[equipment_vm.current_index].gaitem_handle = 0;
                                        equipment_vm.pouch[equipment_vm.current_index].id = 0;
                                        equipment_vm.pouch[equipment_vm.current_index].equip_index = 0;
                                        equipment_vm.pouch[equipment_vm.current_index].name = "Empty".to_string();
                                        equipment_vm.current_equipped_items[pos] = 0;
                                    },
                                }
                            };
                        }
                        else if is_equipped_item {
                            ui.add_sized([100., 24.], egui::Label::new("Equipped"));
                        }
                        else {
                            if ui.add_sized([100., 24.], egui::Button::new("Equip")).clicked() {
                                
                                // Mark this section as changed so when the file is 
                                // saved then it will write this section to the file
                                equipment_vm.changed = true;
                        
                                match &inventory_vm.current_subtype_route {
                                    InventorySubTypeRoute::None => {},
                                    InventorySubTypeRoute::WeaponLeft => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.left_hand_armaments[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.left_hand_armaments[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::WeaponRight => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.right_hand_armaments[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.right_hand_armaments[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Head => {
                                        equipment_vm.head.gaitem_handle = item.ga_item_handle;
                                        equipment_vm.head.id = item.item_id;
                                        equipment_vm.head.equip_index = item.equip_index;
                                        equipment_vm.head.name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[0] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Body => {
                                        equipment_vm.chest.gaitem_handle = item.ga_item_handle;
                                        equipment_vm.chest.id = item.item_id;
                                        equipment_vm.chest.equip_index = item.equip_index;
                                        equipment_vm.chest.name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[0] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Arms => {
                                        equipment_vm.arms.gaitem_handle = item.ga_item_handle;
                                        equipment_vm.arms.id = item.item_id;
                                        equipment_vm.arms.equip_index = item.equip_index;
                                        equipment_vm.arms.name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[0] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Legs => {
                                        equipment_vm.legs.gaitem_handle = item.ga_item_handle;
                                        equipment_vm.legs.id = item.item_id;
                                        equipment_vm.legs.equip_index = item.equip_index;
                                        equipment_vm.legs.name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[0] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Arrow => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.arrows[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.arrows[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.arrows[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.arrows[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.arrows[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                        
                                    },
                                    InventorySubTypeRoute::Bolt => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.bolts[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.bolts[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.bolts[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.bolts[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.bolts[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Talisman => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.talismans[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.talismans[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.talismans[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.talismans[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.talismans[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::QuickItem => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.quickitems[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.quickitems[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.quickitems[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.quickitems[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.quickitems[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                    InventorySubTypeRoute::Pouch => {
                                        let pos = equipment_vm.current_equipped_items.iter().position(|id| *id == equipment_vm.pouch[equipment_vm.current_index].equip_index).unwrap();
                                        equipment_vm.pouch[equipment_vm.current_index].gaitem_handle = item.ga_item_handle;
                                        equipment_vm.pouch[equipment_vm.current_index].id = item.item_id;
                                        equipment_vm.pouch[equipment_vm.current_index].equip_index = item.equip_index;
                                        equipment_vm.pouch[equipment_vm.current_index].name = item.item_name.to_string();
                                        equipment_vm.current_equipped_items[pos] = item.equip_index;
                                    },
                                };
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        }
    }
}