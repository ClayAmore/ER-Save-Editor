use std::cmp::min;
use crate::{ 
    db::{
        accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME, armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, items::items, weapon_name::weapon_name::WEAPON_NAME
    }, 
    save::common::save_slot::{EquipProjectile, GaItem, GaItem2}, 
    util::regulation::Regulation, 
    vm::regulation::regulation_view_model::{
        RegulationItemViewModel, 
        WepType
    } 
};
use super::{InventoryGaitemType, InventoryItemType, InventoryItemViewModel, InventoryViewModel};

impl InventoryViewModel {
    pub fn add_to_inventory(&mut self, item: &RegulationItemViewModel) {
        // Check if inventory is full
        let held_inventory_full = self.storage[0].common_item_count == 2688;

        // Check if held inventory is full
        if held_inventory_full {
            self.log.insert(0, format!("Inventory is full."));
            return;
        }

        // Mark this section as changed so when the file is 
        // saved then it will write this section to the file
        self.changed = true;

        // Handle item add
        match item.item_type {
            InventoryItemType::WEAPON => {
                // Fetch weapon param to look up weapon type
                let weapon_param = Regulation::equip_weapon_params_map().get(&item.id);
                if weapon_param.is_none() {
                    self.log.insert(0, format!("Failed! Couldn't find weapon {}|{:#x} in weapon params.", item.id, item.id));
                    return;
                }

                // Retrieve weapon type and check if weapon type is projectile, default to regular weapon
                let wep_type = WepType::from(weapon_param.unwrap().data.wepType);
                if wep_type == WepType::Arrow || wep_type == WepType::Greatarrow || wep_type == WepType::Bolt || wep_type == WepType::BallistaBolt  {
                    if item.quantity.is_none() {
                        self.log.insert(0, format!("Failed! Quantity is 'None'."));
                        return;
                    }

                    // Add projectile
                    self.add_projectile(item.id, item.quantity.unwrap() as u32);

                }
                else {
                    // Add weapon
                    self.add_weapon(item.id, item.infusion, item.upgrade, item.affinity);
                }
            },
            InventoryItemType::ARMOR => {   
                self.add_armor(item.id);
            },
            InventoryItemType::ACCESSORY => {
                self.add_talisman(item.id);
            },
            InventoryItemType::ITEM => {
                match item.quantity {
                    Some(quantity) => {
                        if item.is_key_item {
                            self.add_key_item(item.id, quantity as u32);
                        }
                        else {
                            self.add_normal_item(item.id, quantity as u32);
                        }
                    },
                    None => self.log.insert(0, format!("Failed to add item. No quantity provided.")),
                };
            },
            InventoryItemType::AOW => {
                self.add_aow(item.id);
            },
            InventoryItemType::None => {
                self.log.insert(0, format!("Failed! Item type is 'None'."))
            },
        }
    }

    fn add_weapon(&mut self, id: u32, gem: Option<u32>, upgrade: Option<i16>, affinity: Option<i16>) {
        // If weapon has ash of war then handle adding the ash of war to the inventory
        let mut gem_gaitem_handle = u32::MAX;
        if gem.is_some() {
            let gem_id = gem.unwrap();
            match Regulation::equip_gem_param_map().get(&gem_id) {
                Some(gem_param) => { 
                    gem_gaitem_handle = self.add_aow(gem_param.id);
                },
                None => {
                    self.log.insert(0, format!("Failed! Cannot find ash of war with id {:#x} in the regaultion params.", gem_id));
                },
            }
        }

        // Check for upgrade
        let upgrade_level: u32 = match upgrade {
            Some(upgrade_level) => upgrade_level as u32,
            None => 0,
        };

        // Check for affinity
        let affinity_id = match affinity {
            Some(affinity_id) => affinity_id as u32,
            None => 0,
        };

        // Construct an item_id by combining the different configuration to the weapon
        // Weapon Id + Affinty Id + Upgrade level
        let item_id = id + upgrade_level + affinity_id;
        let gaitem_handle = self.generate_gaitem_handle_for_armament(InventoryGaitemType::WEAPON);

        // Add gaitem
        self.add_gaitem(GaItem {
            item_id,
            gaitem_handle,
            aow_gaitem_handle: gem_gaitem_handle,
            ..Default::default()
        }, false); 
        
        // Try to fetch weapon name from WEAPON_NAME db in order to include infusion in the name,
        // fallback to weapon name from regulation
        let weapon_name = match WEAPON_NAME.lock().unwrap().get(&((item_id/100)*100)) {
            Some(name) => if upgrade_level > 0 {format!("{} +{}", name, upgrade_level)} else {name.to_string()}
            None => {
                self.log.insert(0, format!("Failed to find name for weapon with id {}|{:#x}", ((item_id/100)*100), ((item_id/100)*100)));
                format!("Failed to find name for weapon with id {}|{:#x}", ((item_id/100)*100), ((item_id/100)*100))
            },
        };

        // Add item to storage
        self.add_to_storage_common_items(gaitem_handle, item_id, 1, 0, weapon_name.to_string(), InventoryGaitemType::WEAPON);

        // Add to gaitem data list if not presenet
        self.upsert_gaitem_data_list(item_id);

        self.log.insert(0, format!("> Added 1 {} to held inventory", weapon_name));

    }

    fn add_projectile(&mut self, id: u32, quantity: u32) {
        let item_id = id | InventoryItemType::WEAPON as u32;
        let gaitem_handle = self.generate_gaitem_handle_for_armament(InventoryGaitemType::WEAPON);
        let max_held;
        let max_in_storage;
        
        // Look up projectile in params
        let weapon_params_res = Regulation::equip_weapon_params_map().get(&id);
        if weapon_params_res.is_some() {
            let weapon_params = weapon_params_res.unwrap();
            max_held = weapon_params.data.maxArrowQuantity as u32;
            max_in_storage = 600;
        }
        else {
            self.log.insert(0, format!("Failed to fetch weapon param for {}|{:#x}. Add item failed!", id, id));
            println!("Failed to fetch weapon param for {}|{:#x}. Add item failed!", id, id);
            return;
        }

        // Fetch name
        let name = match WEAPON_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item projectile id {}|{:#x}", id, id));
                format!("Failed to find name for projectile with id {}|{:#x}", id, id)
            },
        };

        // Search for item in held inventory, retrieve position if found
        let held_index_res = self.storage[0].common_items.iter().position(|i| i.item_id == id);
        let is_already_held = held_index_res.is_some();

        // Add to held inventory is items is not meant to put straight to storage 
        let storage_quantity = 
        // Add item to held inventory, calculate how much of the item quantity will fit
        // rest of the quantity is transferred over to storage quantity
        quantity - ( if is_already_held {
            // Calculate allowed remaining quantity in held inventory
            let free_space = match self.get_free_space_for_item_quantity_in_storage(id,  0, true, false) {
                Ok(val) => val,
                Err(err) => {
                    println!("{err}");
                    0
                },
            };
            
            // If quantity exceedes the free space amount then use max_held instead 
            let amount = min(quantity, free_space);

            if amount != 0 {
                // Change item quantity
                self.increase_item_quantity(id, amount, 0, false);
    
                // Update log
                self.log.insert(0, format!("> Added {} {} to held inventory.", amount, name));
            }

            amount
        } else {
            // Add gaitem
            self.add_gaitem(GaItem {
                item_id: id,
                gaitem_handle,
                ..Default::default()
            }, false); 

            // If quantity exceedes the maximum held amount then use max_held instead 
            let amount = min(quantity, max_held);
            
            if amount != 0 {
                // Add new item to storage
                self.add_to_storage_common_items(gaitem_handle, item_id, amount, 0, name.to_string(), InventoryGaitemType::WEAPON);
                
                // Update log
                self.log.insert(0, format!("> Added {} {} to held inventory.", amount, name));
            }
            amount
        });
        
        // If there's still quantity left then put in storage box
        if storage_quantity > 0 {
            // Search for item in held inventory, retrieve position if found
            let box_index_res = self.storage[1].common_items.iter().position(|i| i.item_id == id);
            let is_already_in_box = box_index_res.is_some();

            // If quantity exceedes the maximum storage amount then use max_in_storage instead 
            if is_already_in_box {
                // Calculate allowed remaining quantity in held inventory
                let free_space = match self.get_free_space_for_item_quantity_in_storage(id,  1, true, false) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{err}");
                        0
                    },
                };
                
                // If quantity exceedes the free space amount then use max_held instead 
                let amount = min(quantity, free_space);

                if amount != 0 {
                    // Change item quantity
                    self.increase_item_quantity(id, amount, 1, false);
                    
                    // Update log
                    self.log.insert(0, format!("> Added {} {} to storage box.", amount, name));
                }
                
            } else {
                // Add gaitem
                self.add_gaitem(GaItem {
                    item_id: id,
                    gaitem_handle,
                    ..Default::default()
                }, false); 

                // If quantity exceedes the maximum held amount then use max_held instead 
                let amount = min(quantity, max_in_storage);

                if amount != 0 {
                    // Add new item to storage
                    self.add_to_storage_common_items(gaitem_handle, item_id, amount, 1, name.to_string(), InventoryGaitemType::WEAPON);
                    
                    // Update log
                    self.log.insert(0, format!("> Added {} {} to storage box.", amount, name));
                }
            }
        }
        
        // Add to gaitem data list if not presenet
        self.upsert_projectile_list(id);

    }

    fn add_normal_item(&mut self, id: u32, quantity: u32) {
        let item_id = id | InventoryItemType::ITEM as u32;
        let gaitem_handle = id | InventoryGaitemType::ITEM as u32;
        let max_held;
        let max_in_storage;

        // Fetch name
        let name = match ITEM_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item with id {}|{:#x}", item_id, item_id));
                format!("Failed to find name for item with id {}|{:#x}", item_id, item_id)
            },
        };

        let item_params_res = Regulation::equip_goods_param_map().get(&id);
        if item_params_res.is_some() {
            let item_params = item_params_res.unwrap();
            max_held = item_params.data.maxNum as u32;
            max_in_storage = item_params.data.maxRepositoryNum as u32;
        }
        else {
            self.log.insert(0, format!("Failed to fetch goods param for item {}|{:#x}. Add item failed!", id, id));
            println!("Failed to fetch goods param for item {}|{:#x}. Add item failed!", id, id);
            return;
        }

        // Check if item needs to go straight to storage. In the case of pots, it's easiest to only,
        // limit them to storage to avoid having more pots held than allowed in game.
        // Not the best implementation, but then again none of this code is.
        let straight_to_storage = items().get("Pots").unwrap().iter().any(|id| *id == item_id) ||
                                    items().get("Perfumes").unwrap().iter().any(|id| *id == item_id );

        // Search for item in held inventory, retrieve position if found
        let held_index_res = self.storage[0].common_items.iter().position(|i| i.ga_item_handle == gaitem_handle);
        let is_already_held = held_index_res.is_some();

        // Add to held inventory is items is not meant to put straight to storage 
        let storage_quantity = if ! straight_to_storage{
            // Add item to held inventory, calculate how much of the item quantity will fit
            // rest of the quantity is transferred over to storage quantity
                quantity - ( if is_already_held {
                    // Calculate allowed remaining quantity in held inventory
                    let free_space = match self.get_free_space_for_item_quantity_in_storage(id, 0, false, false) {
                        Ok(val) => val,
                        Err(err) => {
                            println!("{err}");
                            0
                        },
                    };

                    // If quantity exceedes the free space amount then use max_held instead 
                    let amount = min(quantity, free_space);

                    // Bail out early if there's no more space in inventory
                    if amount != 0 {
                        // Change item quantity
                        self.increase_item_quantity(id, free_space, 0, false);

                        // Update Log
                        self.log.insert(0, format!("> Added {} {} to held inventory", amount, name));
                    }
                    
                    amount
                } else {
                    // If quantity exceedes the maximum held amount then use max_held instead 
                    let amount = min(quantity, max_held);
                    
                    if amount != 0 {
                        // Add new item to storage
                        self.add_to_storage_common_items(gaitem_handle, id, amount, 0, name.to_string(), InventoryGaitemType::ITEM);
                        
                        // Update log
                        self.log.insert(0, format!("> Added {} {} to held inventory", amount, name));
                    }

                    amount
                })
        } else {quantity};
        
        // If there's still quantity left then put in storage box
        if storage_quantity > 0 {
            // Search for item in held inventory, retrieve position if found
            let box_index_res = self.storage[1].common_items.iter().position(|i| i.ga_item_handle == gaitem_handle);
            let is_already_in_box = box_index_res.is_some();

            if is_already_in_box {
                // Calculate allowed remaining quantity in held inventory
                let free_space = match self.get_free_space_for_item_quantity_in_storage(id, 1, false, false) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{err}");
                        0
                    },
                };
                
                // If quantity exceedes the free space amount then use max_held instead 
                let amount = min(quantity, free_space);

                if amount != 0 {
                    // Change item quantity
                    self.increase_item_quantity(id, amount, 1, false);

                    // Update log
                    self.log.insert(0, format!("> Added {} {} to storage box", amount, name));
                }

            } else {
                // If quantity exceedes the maximum held amount then use max_held instead 
                let amount = min(quantity, max_in_storage);
                
                // Add new item to storage
                self.add_to_storage_common_items(gaitem_handle, id, amount, 1, name.to_string(), InventoryGaitemType::ITEM);
                
                // Update log
                self.log.insert(0, format!("> Added {} {} to storage box", amount, name));
            }
        }
        
    }

    fn add_key_item(&mut self, id: u32, quantity: u32) {
        let item_id = id | InventoryItemType::ITEM as u32;
        let gaitem_handle = id | InventoryGaitemType::ITEM as u32;
        let max_held;
        let max_in_storage;

        // Fetch name
        let name = match ITEM_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item with id {}|{:#x}", item_id, item_id));
                format!("Failed to find name for item with id {}|{:#x}", item_id, item_id)
            },
        };

        let item_params_res = Regulation::equip_goods_param_map().get(&id);
        if item_params_res.is_some() {
            let item_params = item_params_res.unwrap();
            max_held = item_params.data.maxNum as u32;
            max_in_storage = item_params.data.maxRepositoryNum as u32;
        }
        else {
            self.log.insert(0, format!("Failed to fetch goods param for key item {}|{:#x}. Add item failed!", id, id));
            println!("Failed to fetch goods param for key item {}|{:#x}. Add item failed!", id, id);
            return;
        }

        // Search for item in held inventory, retrieve position if found
        let held_index_res = self.storage[0].key_items.iter().position(|i| i.ga_item_handle == gaitem_handle);
        let is_already_held = held_index_res.is_some();

        // Add to held inventory is items is not meant to put straight to storage 
        let storage_quantity =
        // Add item to held inventory, calculate how much of the item quantity will fit
        // rest of the quantity is transferred over to storage quantity
            quantity - ( if is_already_held {
                // Calculate allowed remaining quantity in held inventory
                let free_space = match self.get_free_space_for_item_quantity_in_storage(id, 0, false, true) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{err}");
                        0
                    },
                };

                // If quantity exceedes the free space amount then use max_held instead 
                let amount = min(quantity, free_space);

                if amount != 0 {
                    // Change item quantity
                    self.increase_item_quantity(id, amount, 0, true);
    
                    // Update Log
                    self.log.insert(0, format!("> Added {} {} to held inventory", amount, name));
                }

                amount
            } else {
                // If quantity exceedes the maximum held amount then use max_held instead 
                let amount = min(quantity, max_held);
                
                if amount != 0 {
                    // Add new item to storage
                    self.add_to_storage_key_items(gaitem_handle, id, amount, 0, name.to_string(), InventoryGaitemType::ITEM);
    
                    // Update Log
                    self.log.insert(0, format!("> Added {} {} to held inventory", amount, name));
                }
                amount
            });
        
        // If there's still quantity left then put in storage box
        if storage_quantity > 0 {
            // Search for item in held inventory, retrieve position if found
            let box_index_res = self.storage[1].key_items.iter().position(|i| i.ga_item_handle == gaitem_handle);
            let is_already_in_box = box_index_res.is_some();

            // If quantity exceedes the maximum storage amount then use max_in_storage instead 
            if is_already_in_box {
                // Calculate allowed remaining quantity in held inventory
                let free_space = match self.get_free_space_for_item_quantity_in_storage(id, 1, false, true) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{err}");
                        0
                    },
                };

                // To ensure not to exceed storage item quantity limit
                let amount = min(free_space, storage_quantity);

                if amount != 0 {
                    // Change item quantity
                    self.increase_item_quantity(id, amount, 1, true);
    
                    // Update Log
                    self.log.insert(0, format!("> Added {} {} to storage box", amount, name));
                }

            } else {
                // If quantity exceedes the maximum held amount then use max_held instead 
                let amount = min(quantity, max_in_storage);
                
                if amount != 0 {
                    // Add new item to storage
                    self.add_to_storage_key_items(gaitem_handle, id, amount, 1, name.to_string(), InventoryGaitemType::ITEM);
                    
                    // Update log
                    self.log.insert(0, format!("> Added {} {} to storage box", amount, name));
                }
            }
        }
    }

    fn add_aow(&mut self, id: u32) -> u32 {
        // Create item id and gaitem handle
        let item_id = id | InventoryItemType::AOW as u32;
        let gaitem_handle = self.generate_gaitem_handle_for_armament(InventoryGaitemType::AOW);

        // Add to gaitem map
        self.add_gaitem(GaItem {
            item_id,
            gaitem_handle,
            ..Default::default()
        }, true);

        // Fetch name
        let name = match AOW_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item with id {}|{:#x}", item_id, item_id));
                format!("Failed to find name for AOW with id {}|{:#x}", item_id, item_id)
            },
        };

        // Add to common items
        self.add_to_storage_common_items(gaitem_handle, id, 1, 0, name.to_string(), InventoryGaitemType::AOW);

        // Add to gaitem data list if not presenet
        self.upsert_gaitem_data_list(item_id);

        // Update log
        self.log.insert(0, format!("> Added {} {} to held inventory", 1, name));

        gaitem_handle
    }

    fn add_armor(&mut self, id: u32) {
        let item_id = id | InventoryItemType::ARMOR as u32;
        let gaitem_handle = self.generate_gaitem_handle_for_armament(InventoryGaitemType::ARMOR);

        self.add_gaitem(GaItem {
            item_id,
            gaitem_handle,
            ..Default::default()
        }, false);

        // Fetch name
        let name = match ARMOR_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item with id {}|{:#x}", item_id, item_id));
                format!("Failed to find name for Armor with id {}|{:#x}", item_id, item_id)
            },
        };

        self.add_to_storage_common_items(gaitem_handle, id, 1, 0, name.to_string(), InventoryGaitemType::ARMOR);
        
        // Update log
        self.log.insert(0, format!("> Added {} {} to held inventory", 1, name));
    }

    fn add_talisman(&mut self, id: u32) {
        let item_id = id | InventoryItemType::ACCESSORY as u32;
        let gaitem_handle = id | InventoryGaitemType::ACCESSORY as u32;

        // Fetch name
        let name = match ACCESSORY_NAME.lock().unwrap().get(&id) {
            Some(name) => format!("{}",name),            
            None => {
                self.log.insert(0, format!("Failed to find name for item with id {}|{:#x}", item_id, item_id));
                format!("Failed to find name for Talisman with id {}|{:#x}", item_id, item_id)
            },
        };

        self.add_to_storage_common_items(gaitem_handle, id, 1, 0, name.to_string(), InventoryGaitemType::ACCESSORY);

        // Update log
        self.log.insert(0, format!("> Added {} {} to held inventory", 1, name));
    }

    fn add_gaitem(&mut self, gaitem: GaItem, is_aow: bool) {
        match is_aow {
            true => {
                self.gaitem_map[self.next_aow_index] = gaitem;
                self.next_aow_index = self.next_aow_index + 1;
            },
            false => {
                self.gaitem_map[self.next_armament_or_armor_index] = gaitem;
                self.next_armament_or_armor_index = self.next_armament_or_armor_index + 1;
            },
        }
    }

    fn add_to_storage_common_items(&mut self, gaitem_handle: u32, id: u32, quantity: u32, storage_index: usize, name: String, r#type: InventoryGaitemType){
        // Determine equipindex offset based on storage type 0 = held, 1 = storage box
        let equip_index_offset = match storage_index { 0 => 0x180, 1 => 0x7F, _ => panic!("Index out of bounds!") };

        // Prepare new item indexes
        let storage = &mut self.storage[storage_index];
        let items = &mut storage.common_items;
        let index = storage.common_item_count;
        let equip_index = equip_index_offset + storage.common_item_count;
        let acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;

        // Add item to storage
        items[index as usize] = InventoryItemViewModel{
            ga_item_handle: gaitem_handle,
            item_id: id,
            equip_index: equip_index as u32,
            quantity: quantity,
            inventory_index: acquisiton_sort_order_index,
            item_name: name,
            r#type: r#type
        };

        // Storage box equip index starts as 0 on a new character. If items are put in the storage box
        // before placing something in it in game then the equip index will be off. This takes care of
        // that exception.
        let next_equip_index = if equip_index == 0 {
            equip_index + 1
        }
        else {
            self.storage[storage_index].next_equip_index + 1
        };

        // Increment storage indexes
        self.storage[storage_index].common_item_count = self.storage[storage_index].common_item_count + 1;
        self.storage[storage_index].next_equip_index = next_equip_index;
        self.storage[storage_index].next_acquisition_sort_order_index = self.storage[storage_index].next_acquisition_sort_order_index + 1;
    }

    fn add_to_storage_key_items(&mut self, gaitem_handle: u32, id: u32, quantity: u32, storage_index: usize, name: String, r#type: InventoryGaitemType) {
        // Determine equipindex offset based on storage type 0 = held, 1 = storage box
        let equip_index_offset = match storage_index { 0 => 0x180, 1 => 0x7F, _ => panic!("Index out of bounds!") };

        // Prepare new item indexes
        let storage = &mut self.storage[storage_index];
        let items = &mut storage.key_items;
        let index = storage.key_item_count;
        let equip_index = equip_index_offset + storage.key_item_count;
        let acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;

        // Add item to storage
        items[index as usize] = InventoryItemViewModel{
            ga_item_handle: gaitem_handle,
            item_id: id,
            equip_index: equip_index as u32,
            quantity: quantity,
            inventory_index: acquisiton_sort_order_index,
            item_name: name,
            r#type: r#type
        };

        // Increment storage indexes
        self.storage[storage_index].key_item_count = self.storage[storage_index].key_item_count + 1;
        self.storage[storage_index].next_equip_index = equip_index + 1;
        self.storage[storage_index].next_acquisition_sort_order_index = self.storage[storage_index].next_acquisition_sort_order_index + 1;
    }

    fn upsert_gaitem_data_list(&mut self, id: u32) {
        // Add item to gaitem data if not present 
        let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
        let gaitem_data_items = &mut self.gaitem_data.ga_items;
        if !gaitem_data_items.iter().any(|item| item.id == id) {
            gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                id: id,
                unk: 0,
                ..Default::default()
            };
            // Increment gaitem data index
            self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
        }
    }

    fn upsert_projectile_list(&mut self, id: u32) {
        // Add item to projectil list if not present 
        let projcetiles = &mut self.projectile_list.projectiles;
        if !projcetiles.iter().any(|projectile| projectile.projectile_id == id) {
            projcetiles.push(EquipProjectile{
                projectile_id: id,
                unk: 1,
            });
            // Increment gaitem data index
            self.projectile_list.projectile_count = self.projectile_list.projectile_count + 1;

            // Sort projectile list to keep it consistent with how it usually is in game
            self.projectile_list.projectiles.sort_by(|a,b| a.projectile_id.cmp(&b.projectile_id));
        }
    }

    fn generate_gaitem_handle_for_armament(&mut self, r#type: InventoryGaitemType) -> u32 {
        let gaitem_handle = r#type as u32 | self.next_gaitem_handle | (self.part_gaitem_handle as u32) << 16;
        self.next_gaitem_handle = self.next_gaitem_handle + 1;
        gaitem_handle
    }

    fn get_free_space_for_item_quantity_in_storage(&mut self, id: u32, storage_index: usize, is_projectile: bool, is_key_item: bool) -> Result<u32, String>{
        // Gaitem handle for items created from item id, projectile must be looked up
        let gaitem_handle = if !is_projectile {
            id | InventoryGaitemType::ITEM as u32
        } 
        else {
            let gaitem_map_res = self.gaitem_map.iter().filter(|gaitem| gaitem.item_id == id).map(|&gaitem| gaitem).collect::<Vec<GaItem>>();

            // Shouldn't happen. It's expected that there's at least one instance of 
            // the projectile in the gaitem map by this point
            if gaitem_map_res.is_empty() {
                return Err(format!("Failed! Couldn't find projectile with {}|{:#x} in gaitem map.!", id, id));
            }

            // Storage box requires there to be a second entry the projectiles in the 
            // gaitem map
            if storage_index == 1 && gaitem_map_res.len() == 1 {
                return Err(format!("Failed! Couldn't find projectile with {}|{:#x} in the storage box in gaitem map.!", id, id));
            }

            let gaitem = gaitem_map_res[storage_index];
            gaitem.gaitem_handle
        };

        // 0: Max amount of item in held storage
        // 1: Max amount of item in box storage
        let mut max: [u32; 2] = [0; 2];

        // Projectile limit is always the same
        if is_projectile {
            max[0] = 99;
            max[1] = 600;
        }
        // Look up item param to fetch storage limits
        else {
            let item_params_res = Regulation::equip_goods_param_map().get(&id);
            if item_params_res.is_some() {
                let item_params = item_params_res.unwrap();
                max[0] = item_params.data.maxNum as u32;
                max[1] = item_params.data.maxRepositoryNum as u32;
            }
            else {
                return Err(format!("Failed to determine storage limits for item {}|{:#x}. Add item failed!", id, id));
            }
        }

        // Look up item in storage box
        let item_res = match is_key_item {
            true => self.storage[storage_index].key_items.iter().find(|i| i.ga_item_handle == gaitem_handle),
            false => self.storage[storage_index].common_items.iter().find(|i| i.ga_item_handle == gaitem_handle),
        };

        if item_res.is_some() {
            // Fetch item from item list 
            let item = item_res.unwrap();
            let current_item_quantity = item.quantity;

            // Calculate how much free space for item is left in storage 
            let free_space = max[storage_index] - current_item_quantity;

            return Ok(free_space);
        }


        Err(format!("Failed to determine storage limits for item {}|{:#x}. Add item failed!", id, id))
    }
    
    fn increase_item_quantity(&mut self, id: u32, amount: u32, storage_index: usize, is_key_item: bool) {
        // Look up item in storage box
        let item_res = match is_key_item {
            true => self.storage[storage_index].key_items.iter_mut().find(|i| i.item_id == id),
            false => self.storage[storage_index].common_items.iter_mut().find(|i| i.item_id == id),
        };

        if item_res.is_some() {
            let item = item_res.unwrap();

            // Change item quantity
            item.quantity = item.quantity + amount;
        }
        else {
            self.log.insert(0, format!("Failed to update quantity for item {}|{:#x}. Failed to find item!", id, id));
        }
    }
}
