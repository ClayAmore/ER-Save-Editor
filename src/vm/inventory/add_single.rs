use std::cmp::min;
use crate::{ 
    db::{items::items, weapon_name::weapon_name::WEAPON_NAME}, 
    save::common::save_slot::{GaItem, GaItem2}, 
    util::regulation::Regulation, 
    vm::regulation::regulation_view_model::RegulationItemViewModel 
};
use super::{InventoryGaitemType, InventoryItemType, InventoryItemViewModel, InventoryViewModel};

impl InventoryViewModel {
    pub fn add_to_inventory(&mut self, item: &RegulationItemViewModel) {
        // Moved outside of the match scop in order to be used in the log at the end of the function
        // (This needs a major rewrite)
        let mut weapon_name = String::new();
        match item.item_type {
            InventoryItemType::WEAPON => {
                // If weapon has ash of war then handle adding the ash of war to the inventory
                let gem_gaitem_handle = match item.infusion {
                    Some(gem_id) => {
                        let gem_res = Regulation::equip_gem_param_map().get(&gem_id);
                        match gem_res {
                            Some(gem_param) => {
                                let next_gaitem_handle = self.next_gaitem_handle;
                                self.add_to_inventory(&RegulationItemViewModel{
                                    id: gem_param.id,
                                    name: gem_param.name.to_string(),
                                    max_held: 1,
                                    max_storage: 1,
                                    item_type: InventoryItemType::AOW,
                                    ..Default::default()
                                });
                                u32::from_be_bytes([0xC0, self.part_gaitem_handle, next_gaitem_handle.to_be_bytes()[2], next_gaitem_handle.to_be_bytes()[3]])
                            },
                            None => {
                                println!("Cannot find ash of war {} in the regaultion params", gem_id);
                                u32::MAX
                            },
                        }
                    },
                    None => u32::MAX,
                } as u32;

                // Check for upgrade
                let upgrade_level: u32 = match item.upgrade {
                    Some(upgrade_level) => upgrade_level as u32,
                    None => 0,
                };

                // Check for affinity
                let affinity_id = match item.affinity {
                    Some(affinity_id) => affinity_id as u32,
                    None => 0,
                };
                
                // Construct an item_id by combining the different configuration to the weapon
                // Weapon Id + Affinty Id + Upgrade level
                let item_id = item.id + upgrade_level + affinity_id;
                let gaitem_handle = u32::from_be_bytes([0x80, self.part_gaitem_handle, self.next_gaitem_handle.to_be_bytes()[2], self.next_gaitem_handle.to_be_bytes()[3]]);

                // Try to fetch weapon name from WEAPON_NAME db in order to include infusion in the name,
                // fallback to weapon name from regulation
                weapon_name = match WEAPON_NAME.lock().unwrap().get(&((item_id/100)*100)) {
                    Some(name) => if upgrade_level > 0 {format!("{} +{}", name, upgrade_level)} else {name.to_string()},
                    None => if upgrade_level > 0 {format!("{} +{}", item.name, upgrade_level)} else {item.name.to_string()},
                };

                // Add gaitem
                self.gaitem_map[self.next_armament_or_armor_index] = GaItem {
                    gaitem_handle: gaitem_handle,
                    item_id: item.id,
                    aow_gaitem_handle: gem_gaitem_handle,
                    ..Default::default()
                };
                
                // Add item to common inventory
                let storage = &mut self.storage[0];
                let items = &mut storage.common_items;
                let items_next_index = storage.common_item_count;
                let items_next_equip_index = 0x180 + storage.common_item_count;
                let items_next_acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;
                items[items_next_index as usize] = InventoryItemViewModel{
                    ga_item_handle: gaitem_handle,
                    item_id: item.id,
                    equip_index: items_next_equip_index as u32,
                    quantity: match item.quantity { Some(quantity) => quantity as u32, None => 1, },
                    inventory_index: items_next_acquisiton_sort_order_index,
                    item_name: weapon_name.to_string(),
                    r#type: InventoryGaitemType::WEAPON
                };
                
                // Add item to gaitem data if not present 
                let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                let gaitem_data_items = &mut self.gaitem_data.ga_items;
                if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                    gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                        id: item_id,
                        unk: 0,
                        reinforce_type: items_next_equip_index,
                        ..Default::default()
                    };
                    // Increment gaitem data index
                    self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                }
                
                // Increment indexes for gaitem map
                self.next_gaitem_handle = self.next_gaitem_handle + 1;
                self.next_armament_or_armor_index = self.next_armament_or_armor_index + 1;
                
                // Increment indexes for common items
                self.storage[0].common_item_count = self.storage[0].common_item_count + 1;
                self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
                self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;

            },
            InventoryItemType::ARMOR => {
                let item_id = item.id | InventoryItemType::ARMOR as u32;
                let gaitem_handle = u32::from_be_bytes([0x90, self.part_gaitem_handle, self.next_gaitem_handle.to_be_bytes()[2], self.next_gaitem_handle.to_be_bytes()[3]]);

                // Add gaitem
                self.gaitem_map[self.next_armament_or_armor_index] = GaItem {
                    gaitem_handle: gaitem_handle,
                    item_id: item.id,
                    ..Default::default()
                };

                // Add item to common inventory
                let storage = &mut self.storage[0];
                let common_items = &mut storage.common_items;
                let common_items_next_index = storage.common_item_count;
                let common_items_next_equip_index = 0x180 + storage.common_item_count;
                let common_items_next_acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;
                common_items[common_items_next_index as usize] = InventoryItemViewModel{
                    ga_item_handle: gaitem_handle,
                    item_id: item.id,
                    equip_index: common_items_next_equip_index,
                    quantity: 1,
                    inventory_index: common_items_next_acquisiton_sort_order_index,
                    item_name: item.name.to_string(),
                    r#type: InventoryGaitemType::ARMOR
                };

                // Add item to gaitem data 
                let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                let gaitem_data_items = &mut self.gaitem_data.ga_items;
                if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                    gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                        id: item_id,
                        ..Default::default()
                    };
                    // Increment gaitem data index
                    self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                }

                // Increment indexes for gaitem map
                self.next_gaitem_handle = self.next_gaitem_handle + 1;
                self.next_armament_or_armor_index = self.next_armament_or_armor_index + 1;
                
                // Increment indexes for common items
                self.storage[0].common_item_count = self.storage[0].common_item_count + 1;
                self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
                self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;
            },
            InventoryItemType::ACCESSORY => {
                let item_id = item.id | InventoryItemType::ACCESSORY as u32;
                let gaitem_handle = u32::from_be_bytes([0xA0, 0x0, item_id.to_be_bytes()[2], item_id.to_be_bytes()[3]]);

                // Add item to common inventory
                let storage = &mut self.storage[0];
                let common_items = &mut storage.common_items;
                let common_items_next_index = storage.common_item_count;
                let common_items_next_equip_index = 0x180 + storage.common_item_count;
                let common_items_next_acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;
                common_items[common_items_next_index as usize] = InventoryItemViewModel{
                    ga_item_handle: gaitem_handle,
                    item_id: item.id,
                    equip_index: common_items_next_equip_index,
                    quantity: 1,
                    inventory_index: common_items_next_acquisiton_sort_order_index,
                    item_name: item.name.to_string(),
                    r#type: InventoryGaitemType::ACCESSORY
                };

                // Add item to gaitem data 
                let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                let gaitem_data_items = &mut self.gaitem_data.ga_items;
                if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                    gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                        id: item_id,
                        ..Default::default()
                    };
                    // Increment gaitem data index
                    self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                }

                // Increment indexes for common items
                self.storage[0].common_item_count = self.storage[0].common_item_count + 1;
                self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
                self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;
            },
            InventoryItemType::ITEM => {
                let item_id = item.id | InventoryItemType::ITEM as u32;
                let gaitem_handle = item.id | InventoryGaitemType::ITEM as u32;

                // Check if item needs to go straight to storage. In the case of pots, it's easiest to only,
                // limit them to storage to avoid having more pots held than allowed in game.
                // Not the best implementation, but then again none of this code is.
                let straight_to_storage = items().get("Pots").unwrap().iter().any(|id| *id == item_id) ||
                                            items().get("Perfumes").unwrap().iter().any(|id| *id == item_id );

                if item.is_key_item {
                    // Add item to common inventory
                    let held_index_res = self.storage[0].key_items.iter().position(|i| i.item_id == item.id);
                    let box_index_res = self.storage[1].key_items.iter().position(|i| i.item_id == item.id);

                    let item_exists = held_index_res.is_some() || box_index_res.is_some();

                    // Add item to gaitem data if not present
                    if !item_exists {
                        let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                        let gaitem_data_items = &mut self.gaitem_data.ga_items;
                        if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                            gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                                id: item_id,
                                ..Default::default()
                            };
                            // Increment gaitem data index
                            self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                        }
                    }

                    // Handle item adding based on wether item is already in inventory or not
                    let quantity = item.quantity.unwrap() as u32;
                    let remaining_quantity = (quantity as i32) - (
                    if held_index_res.is_some() {
                        let index = held_index_res.unwrap();
                        let held_item = &mut self.storage[0].key_items[index];
                        let available_quantity = (item.max_held as u32) - held_item.quantity;
                        held_item.quantity = held_item.quantity + available_quantity;
                        available_quantity as i32
                    }
                    else {
                        let key_items_next_index = self.storage[0].key_item_count;
                        let key_items_next_acquisiton_sort_order_index = self.storage[0].next_acquisition_sort_order_index;
                        let available_quantity = item.max_held as u32;
                        self.storage[0].key_items[key_items_next_index as usize] = InventoryItemViewModel{
                            ga_item_handle: gaitem_handle,
                            item_id: item.id,
                            equip_index: 0,
                            quantity: min(quantity, available_quantity),
                            inventory_index: key_items_next_acquisiton_sort_order_index,
                            item_name: item.name.to_string(),
                            r#type: InventoryGaitemType::ITEM
                        };

                        // Increment indexes for common items
                        self.storage[0].key_item_count = self.storage[0].key_item_count + 1;
                        self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;
                        available_quantity as i32
                    });

                    if remaining_quantity > 0 {
                        if box_index_res.is_some() {
                            let index = box_index_res.unwrap();
                            let box_item = &mut self.storage[1].key_items[index];
                            let available_quantity = (item.max_storage as u32) - box_item.quantity;
                            box_item.quantity = box_item.quantity + available_quantity;
                        }
                        else {
                            let key_items_next_index = self.storage[1].key_item_count;
                            let key_items_next_equip_index = 0x180 + self.storage[1].key_item_count;
                            let key_items_next_acquisiton_sort_order_index = self.storage[1].next_acquisition_sort_order_index;
                            let available_quantity = item.max_storage as u32;
                            self.storage[1].key_items[key_items_next_index as usize] = InventoryItemViewModel{
                                ga_item_handle: gaitem_handle,
                                item_id: item.id,
                                equip_index: key_items_next_equip_index,
                                quantity: min(remaining_quantity as u32, available_quantity),
                                inventory_index: key_items_next_acquisiton_sort_order_index,
                                item_name: item.name.to_string(),
                                r#type: InventoryGaitemType::ITEM
                            };

                            // Increment indexes for common items
                            self.storage[1].key_item_count = self.storage[1].key_item_count + 1;
                            self.storage[1].next_acquisition_sort_order_index = self.storage[1].next_acquisition_sort_order_index + 1;
                        };
                    }
                }
                else {
                    // Add item to common inventory
                    let held_index_res = self.storage[0].common_items.iter().position(|i| i.item_id == item.id);
                    let box_index_res = self.storage[1].common_items.iter().position(|i| i.item_id == item.id);
                    
                    let item_exists = held_index_res.is_some() || box_index_res.is_some();

                    // Add item to gaitem data if not present
                    if !item_exists {
                        let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                        let gaitem_data_items = &mut self.gaitem_data.ga_items;
                        if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                            gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                                id: item_id,
                                ..Default::default()
                            };
                            // Increment gaitem data index
                            self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                        }
                    }

                    // Handle item adding based on wether item is already in inventory or not
                    let quantity = item.quantity.unwrap() as u32;
                    let remaining_quantity = (quantity as i32) - (
                    if !straight_to_storage {
                        if held_index_res.is_some() {
                            let index = held_index_res.unwrap();
                            let held_item = &mut self.storage[0].common_items[index];
                            let available_quantity = (item.max_held as u32) - held_item.quantity;
                            held_item.quantity = held_item.quantity + available_quantity;
                            available_quantity as i32
                        }
                        else {
                            let common_items_next_index = self.storage[0].common_item_count;
                            let common_items_next_equip_index = 0x180 + self.storage[0].common_item_count;
                            let common_items_next_acquisiton_sort_order_index = self.storage[0].next_acquisition_sort_order_index;
                            let available_quantity = item.max_held as u32;
                            self.storage[0].common_items[common_items_next_index as usize] = InventoryItemViewModel{
                                ga_item_handle: gaitem_handle,
                                item_id: item.id,
                                equip_index: common_items_next_equip_index,
                                quantity: min(quantity, available_quantity),
                                inventory_index: common_items_next_acquisiton_sort_order_index,
                                item_name: item.name.to_string(),
                                r#type: InventoryGaitemType::ITEM
                            };
        
                            // Increment indexes for common items
                            self.storage[0].common_item_count = self.storage[0].common_item_count + 1;
                            self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
                            self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;
                            available_quantity as i32
                        }
                    } else{0});

                    if remaining_quantity > 0 {
                        if box_index_res.is_some() {
                            let index = box_index_res.unwrap();
                            let box_item = &mut self.storage[1].common_items[index];
                            let available_quantity = (item.max_storage as u32) - box_item.quantity;
                            box_item.quantity = box_item.quantity + available_quantity;
                        }
                        else {
                            let common_items_next_index = self.storage[1].common_item_count;
                            let common_items_next_equip_index = if self.storage[1].next_equip_index == 0 {
                                // In case it's a fresh character.
                                0x7F + self.storage[1].common_item_count
                            } else {
                                // If it's not a fresh character then equip index should be right
                                self.storage[1].next_equip_index
                            };
                            let common_items_next_acquisiton_sort_order_index = self.storage[1].next_acquisition_sort_order_index;
                            let available_quantity = item.max_storage as u32;
                            self.storage[1].common_items[common_items_next_index as usize] = InventoryItemViewModel{
                                ga_item_handle: gaitem_handle,
                                item_id: item.id,
                                equip_index: common_items_next_equip_index,
                                quantity: min(remaining_quantity as u32, available_quantity),
                                inventory_index: common_items_next_acquisiton_sort_order_index,
                                item_name: item.name.to_string(),
                                r#type: InventoryGaitemType::ITEM
                            };
        
                            // Increment indexes for common items
                            self.storage[1].common_item_count = self.storage[1].common_item_count + 1;
                            self.storage[1].next_equip_index = common_items_next_equip_index + 1;
                            self.storage[1].next_acquisition_sort_order_index = self.storage[1].next_acquisition_sort_order_index + 1;
                        };
                    }
                }
            },
            InventoryItemType::AOW => {
                let item_id = item.id | InventoryItemType::AOW as u32;
                let gaitem_handle = u32::from_be_bytes([0xC0, self.part_gaitem_handle, self.next_gaitem_handle.to_be_bytes()[2], self.next_gaitem_handle.to_be_bytes()[3]]);

                // Add gaitem
                self.gaitem_map[self.next_ash_of_war_gaitem_index] = GaItem {
                    gaitem_handle: gaitem_handle,
                    item_id: item.id,
                    ..Default::default()
                };

                // Add item to common inventory
                let storage = &mut self.storage[0];
                let common_items = &mut storage.common_items;
                let common_items_next_index = storage.common_item_count;
                let common_items_next_equip_index = 0x180 + storage.common_item_count;
                let common_items_next_acquisiton_sort_order_index = storage.next_acquisition_sort_order_index;
                common_items[common_items_next_index as usize] = InventoryItemViewModel{
                    ga_item_handle: gaitem_handle,
                    item_id: item.id,
                    equip_index: common_items_next_equip_index,
                    quantity: 1,
                    inventory_index: common_items_next_acquisiton_sort_order_index,
                    item_name: item.name.to_string(),
                    r#type: InventoryGaitemType::AOW
                };

                // Add item to gaitem data 
                let next_gaitem_data_index = self.gaitem_data.distinct_aquired_items_count;
                let gaitem_data_items = &mut self.gaitem_data.ga_items;
                if !gaitem_data_items.iter().any(|item| item.id == item_id) {
                    gaitem_data_items[next_gaitem_data_index as usize] = GaItem2{
                        id: item_id,
                        unk: 0xFFFFFF01,
                        reinforce_type: common_items_next_equip_index,
                        ..Default::default()
                    };
                    // Increment gaitem data index
                    self.gaitem_data.distinct_aquired_items_count = self.gaitem_data.distinct_aquired_items_count + 1;
                }

                // Increment indexes for gaitem map
                self.next_gaitem_handle = self.next_gaitem_handle + 1;
                self.next_ash_of_war_gaitem_index = self.next_ash_of_war_gaitem_index + 1;
                
                // Increment indexes for common items
                self.storage[0].common_item_count = self.storage[0].common_item_count + 1;
                self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
                self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index + 1;
            },
            InventoryItemType::None => {

            },
        }
        
        self.log.insert(0, format!("> Added {} {}", match item.quantity { Some(n) => n, None => 1 }, 
            if item.item_type == InventoryItemType::WEAPON { &weapon_name } else { &item.name }
        ))
    }
}