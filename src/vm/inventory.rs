pub mod inventory {
    use std::{cmp::Ordering, collections::BTreeMap};
    use strsim::sorensen_dice;

    use crate::{db::{ accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME,  armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, weapon_name::weapon_name::WEAPON_NAME}, save::common::save_slot::{EquipInventoryData, EquipInventoryItem, GaItem, SaveSlot}, vm::regulation::regulation_view_model::RegulationItemViewModel};

    #[derive(Clone)]
    pub enum InventoryRoute {
        None,
        Add,
        Browse,
    }

    #[derive(Clone)]
    pub enum InventoryTypeRoute {
        None,
        CommonItems,
        KeyItems,
        Weapons,
        Armors,
        AshOfWar,
        Talismans,
    }

    #[derive(Clone, Default, Copy)]
    pub enum InventoryItemType {
        #[default] None = -1,
        WEAPON = 0x0,
        ARMOR = 0x10000000,
        ACCESSORY = 0x20000000,
        ITEM = 0x40000000,
        AOW = 0x80000000,
    }
    impl From<u8> for InventoryItemType {
        fn from(value: u8) -> Self {
            match value {
                0x0 => InventoryItemType::WEAPON,
                0x10 => InventoryItemType::ARMOR,
                0x20 => InventoryItemType::ACCESSORY,
                0x40 => InventoryItemType::ITEM,
                0x80 => InventoryItemType::AOW,
                _ => InventoryItemType::None,
            }
        }
    }
    impl ToString for InventoryItemType {
        fn to_string(&self) -> String {
            match self {
                InventoryItemType::None => format!("None"),
                InventoryItemType::WEAPON => format!("WEAPON"),
                InventoryItemType::ARMOR => format!("ARMOR"),
                InventoryItemType::ACCESSORY => format!("ACCESSORY"),
                InventoryItemType::ITEM => format!("ITEM"),
                InventoryItemType::AOW => format!("AOW"),
            }
            
        }
    }

    pub enum InventoryGaitemType {
        UNKNOWN = -1,
        WEAPON = 0x80000000,
        ARMOR = 0x90000000,
        ACCESSORY = 0xa0000000,
        ITEM = 0xb0000000,
        AOW = 0xc0000000,
    }
    impl From<u32> for InventoryGaitemType {
        fn from(value: u32) -> Self {
            match value {
                x if x == InventoryGaitemType::WEAPON as u32 => InventoryGaitemType::WEAPON,
                x if x == InventoryGaitemType::ARMOR as u32 => InventoryGaitemType::ARMOR,
                x if x == InventoryGaitemType::ACCESSORY as u32 => InventoryGaitemType::ACCESSORY,
                x if x == InventoryGaitemType::ITEM as u32 => InventoryGaitemType::ITEM,
                x if x == InventoryGaitemType::AOW as u32 => InventoryGaitemType::AOW,
                _ => InventoryGaitemType::UNKNOWN,
            }
        }
    }
 
    #[derive(Clone)]
    pub struct InventoryItemViewModel  {
        pub ga_item_handle: u32,
        pub item_id: u32,
        pub item_name: String,
        pub quantity: i32,
        pub inventory_index: i32
    }

    impl Default for InventoryItemViewModel {
        fn default() -> Self {
            Self { 
                ga_item_handle: Default::default(), 
                item_id: Default::default(), 
                item_name: Default::default(),
                quantity: Default::default(), 
                inventory_index: Default::default() 
            }
        }
    }

    impl InventoryItemViewModel {
        pub fn from_save(item_info: &EquipInventoryItem, gaitem: &GaItem, gaitem_type: InventoryGaitemType) -> Self {
            let gaitem_handle = item_info.ga_item_handle as u32;
            let item_type_specific = match gaitem_type {
                InventoryGaitemType::WEAPON => {
                    let id = (gaitem.item_id / 100)*100;
                    (id, match WEAPON_NAME.lock().unwrap().get(&id) {
                        Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", id)},
                        None => format!("[UNKOWN_{}]", id),
                    })
                },
                InventoryGaitemType::ARMOR => {
                    let id = gaitem.item_id ^ InventoryItemType::ARMOR as u32;
                    (id, match ARMOR_NAME.lock().unwrap().get(&id) {
                        Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", id)},
                        None => format!("[UNKOWN_{}]", id),
                    })
                },
                InventoryGaitemType::ACCESSORY => {
                    let id = gaitem_handle ^ InventoryGaitemType::ACCESSORY as u32;
                    (id, match ACCESSORY_NAME.lock().unwrap().get(&id) {
                        Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", id)},
                        None => format!("[UNKOWN_{}]", id),
                    })
                },
                InventoryGaitemType::ITEM => {
                    let id = gaitem_handle ^ InventoryGaitemType::ITEM as u32;
                    (id, match ITEM_NAME.lock().unwrap().get(&id) {
                        Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", id)},
                        None => format!("[UNKOWN_{}]", id),
                    })
                },
                InventoryGaitemType::AOW => {
                    let id = gaitem.item_id ^ InventoryItemType::AOW as u32;
                    (id, match AOW_NAME.lock().unwrap().get(&id) {
                        Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", id)},
                        None => format!("[UNKOWN_{}]", id),
                    })
                },
                InventoryGaitemType::UNKNOWN => panic!("We shouldn't reach this!"),
            };            

            Self {
                ga_item_handle: item_info.ga_item_handle,
                item_id: item_type_specific.0,
                item_name: item_type_specific.1,
                quantity: item_info.quantity,
                inventory_index: item_info.inventory_index
            }
        }
    }

    #[derive(Clone)]
    pub struct InventoryStorage {
        pub common_items: Vec<InventoryItemViewModel>,
        pub key_items: Vec<InventoryItemViewModel>,
        pub weapons: Vec<InventoryItemViewModel>,
        pub armors: Vec<InventoryItemViewModel>,
        pub aows: Vec<InventoryItemViewModel>,
        pub projectiles: Vec<InventoryItemViewModel>,
        pub accessories: Vec<InventoryItemViewModel>,

        pub filtered_common_items: Vec<InventoryItemViewModel>,
        pub filtered_key_items: Vec<InventoryItemViewModel>,
        pub filtered_weapons: Vec<InventoryItemViewModel>,
        pub filtered_armors: Vec<InventoryItemViewModel>,
        pub filtered_aows: Vec<InventoryItemViewModel>,
        pub filtered_projectiles: Vec<InventoryItemViewModel>,
        pub filtered_accessories: Vec<InventoryItemViewModel>,

        pub common_item_count: i32,
        pub key_item_count: i32,
        pub next_acquisition_sort_order_index: i32,
    }

    impl Default for InventoryStorage {
        fn default() -> Self{
            Self{ 
                common_items: Default::default(),
                key_items: Default::default(),
                weapons: Default::default(),
                armors: Default::default(),
                aows: Default::default(),
                projectiles: Default::default(),
                accessories: Default::default(),

                filtered_common_items: Default::default(),
                filtered_key_items: Default::default(),
                filtered_weapons: Default::default(),
                filtered_armors: Default::default(),
                filtered_aows: Default::default(),
                filtered_projectiles: Default::default(),
                filtered_accessories: Default::default(),

                common_item_count: Default::default(),
                key_item_count: Default::default(),
                next_acquisition_sort_order_index: Default::default(),
            }
        }
    }

    #[derive(Clone)]
    pub struct InventoryViewModel  {
        pub at_storage_box: bool,
        pub current_route: InventoryRoute,
        pub current_type_route: InventoryTypeRoute,
        pub filter_text: String,
        pub storage: Vec<InventoryStorage>,
        pub infusions: Vec<(i32, String)>,
        pub gaitem_map: BTreeMap<u32, GaItem>,
        next_gaitem_index: u16,
        part_gaitem_handle: u8,
    }

    impl Default for InventoryViewModel {
        fn default() -> Self {
            Self {
                current_route: InventoryRoute::None,
                current_type_route: InventoryTypeRoute::None,
                filter_text: Default::default(),
                infusions: Default::default(),
                gaitem_map: Default::default(),
                next_gaitem_index: Default::default(),
                part_gaitem_handle: Default::default(),
                at_storage_box: Default::default(),
                storage: vec![InventoryStorage::default(); 2],
            }
        }
    }

    impl InventoryViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut inventory_vm = InventoryViewModel::default();

            inventory_vm.gaitem_map = slot.ga_items.iter()
                .filter(|g| g.gaitem_handle != 0)
                .map(|g| (g.gaitem_handle, *g))
                .collect();

            // Part of the gaitem handle. Should be the same value accross the whole list so we're just stroring it from the first element.
            inventory_vm.part_gaitem_handle = slot.ga_items[0].gaitem_handle.to_le_bytes()[2];

            // Find the next gaitem handle index by sorting the lower bits of the gaitem handle by ascending then picking the last and incrementing by 1.
            let mut ga_items = slot.ga_items.iter()
            .map(|g|{
                let bytes = g.gaitem_handle.to_le_bytes();
                let u16 = u16::from_le_bytes([bytes[0], bytes[1]]);
                u16
            }).collect::<Vec<u16>>();
            ga_items.sort_by(|a,b| a.cmp(&b));
            inventory_vm.next_gaitem_index = *ga_items.last().expect("Gaitem list appears to be empty! This shouldn't be the case.") + 1;

            inventory_vm.fill_stroage_type(&slot.equip_inventory_data, slot.equip_inventory_data.next_acquisition_sort_id ,0);
            inventory_vm.fill_stroage_type(&slot.storage_inventory_data, slot.storage_inventory_data.next_acquisition_sort_id ,1);
            
            inventory_vm
        }

        fn fill_stroage_type(&mut self, equip_inventory_data: &EquipInventoryData, next_acquisition_sort_id: i32 , inventory_storage_index: usize) {
            let inventory_storage = &mut self.storage[inventory_storage_index];
            for i in 0..equip_inventory_data.common_inventory_items_distinct_count {
                let equip_invenotry_item = &equip_inventory_data.common_items[i as usize];

                // Skip empty entries
                if equip_invenotry_item.ga_item_handle == u32::MAX || equip_invenotry_item.ga_item_handle == 0 { continue; };

                // Determine item type from gaitem_handle
                let inventory_gaitem_type = InventoryGaitemType::from(equip_invenotry_item.ga_item_handle as u32 & 0xf0000000);

                match inventory_gaitem_type {
                    InventoryGaitemType::WEAPON => {
                        let gaitem = self.gaitem_map[&equip_invenotry_item.ga_item_handle];
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &gaitem, InventoryGaitemType::WEAPON);
                        inventory_storage.weapons.push(inventory_item_vm);
                    },
                    InventoryGaitemType::ARMOR => {
                        let gaitem = self.gaitem_map[&equip_invenotry_item.ga_item_handle];
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &gaitem, InventoryGaitemType::ARMOR);
                        inventory_storage.armors.push( inventory_item_vm);
                    },
                    InventoryGaitemType::ACCESSORY => {
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &GaItem::default(), InventoryGaitemType::ACCESSORY);
                        inventory_storage.accessories.push( inventory_item_vm);
                    },
                    InventoryGaitemType::ITEM => {
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &GaItem::default(), InventoryGaitemType::ITEM);
                        inventory_storage.common_items.push( inventory_item_vm);
                    },
                    InventoryGaitemType::AOW => {
                        let gaitem = self.gaitem_map[&equip_invenotry_item.ga_item_handle];
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &gaitem, InventoryGaitemType::AOW);
                        inventory_storage.aows.push( inventory_item_vm);
                    },
                    InventoryGaitemType::UNKNOWN => {
                        // Unknown types will just be ignored
                    },
                }
            }

            for i in 0..equip_inventory_data.key_inventory_items_distinct_count {
                let equip_invenotry_item = &equip_inventory_data.key_items[i as usize];
                if equip_invenotry_item.ga_item_handle == u32::MAX || equip_invenotry_item.ga_item_handle == 0 { continue; };
                let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &GaItem::default(), InventoryGaitemType::ITEM);
                inventory_storage.key_items.push( inventory_item_vm);
            }

            inventory_storage.weapons.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.armors.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.common_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.key_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.aows.sort_by(|a, b| a.item_name.cmp(&b.item_name));

            inventory_storage.filtered_weapons = inventory_storage.weapons.clone();
            inventory_storage.filtered_armors = inventory_storage.armors.clone();
            inventory_storage.filtered_common_items = inventory_storage.common_items.clone();
            inventory_storage.filtered_key_items = inventory_storage.key_items.clone();
            inventory_storage.filtered_aows = inventory_storage.aows.clone();

            inventory_storage.common_item_count = equip_inventory_data.common_inventory_items_distinct_count;
            inventory_storage.key_item_count = equip_inventory_data.key_inventory_items_distinct_count;
            inventory_storage.next_acquisition_sort_order_index = next_acquisition_sort_id;

        }

        pub fn filter(&mut self) {
            for inventory_storage in &mut self.storage {
                inventory_storage.filtered_weapons = inventory_storage.weapons.iter()
                .filter(|i | {
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3    
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_armors = inventory_storage.armors.iter()
                .filter(|i | {
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_common_items = inventory_storage.common_items.iter()
                .filter(|i | {
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_key_items = inventory_storage.key_items.iter()
                .filter(|i | {
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_aows = inventory_storage.aows.iter()
                .filter(|i | {
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();

                inventory_storage.weapons.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.armors.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.common_items.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.key_items.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.aows.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
            }
        }
    
        pub fn add_item(&mut self, item: &RegulationItemViewModel) {
            match item.item_type {
                InventoryItemType::WEAPON => {
                    let gem_id = match item.infusion {
                        Some(gem_id) => gem_id,
                        None => -1,
                    } as u32;

                    let upgrade_level: u32 = match item.upgrade {
                        Some(upgrade_level) => upgrade_level as u32,
                        None => 0,
                    };

                    let affinity_id = match item.affinity {
                        Some(affinity_id) => affinity_id,
                        None => 0,
                    };

                    // Weapon Id
                    let weapon_id = item.id as u32 + affinity_id as u32;
                    let reinforced_weapon_id = item.id  as u32  + upgrade_level;
                    // Create gaitem handle for weapon
                    let weapon_gaitem_handle = u32::from_le_bytes([0, 0, self.part_gaitem_handle,  0x0]) + (self.next_gaitem_index as u32);
                    self.next_gaitem_index = self.next_gaitem_index + 1;

                    // Check if weapon should be infused with a gem
                    let gem_handle = if gem_id != u32::MAX {
                        // Check if gem is already in gaitem map
                        let gem = self.storage[0].aows.iter().find(|aow| {
                            aow.item_id == gem_id 
                        });
                        
                        // If gem is not in gaitem map then generate a new gem_gaitem_handle
                        let gem_handle = if gem.is_none() {
                            let gem_gaitem_handle = u32::from_le_bytes([0, 0,self.part_gaitem_handle,  0xC0]) + (self.next_gaitem_index as u32);
                            self.next_gaitem_index = self.next_gaitem_index + 1;

                            let next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index;
                            self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index+1;

                            // Create new item
                            let new_gem = InventoryItemViewModel { 
                                ga_item_handle: gem_gaitem_handle, 
                                item_id: gem_id, 
                                item_name: AOW_NAME.lock().unwrap()[&gem_id].to_string(), 
                                quantity: 1, 
                                inventory_index: next_acquisition_sort_order_index
                            };
                            
                            // Update gaitem_map with new gem
                            self.gaitem_map.insert(new_gem.ga_item_handle, GaItem{
                                gaitem_handle: new_gem.ga_item_handle,
                                item_id: new_gem.item_id,
                                ..Default::default()
                            });

                            // Update inventory with new gem
                            self.storage[0].aows.push(new_gem);

                            gem_gaitem_handle
                        }
                        else {
                            gem.unwrap().ga_item_handle
                        };
                        gem_handle
                    }
                    else {
                        u32::MAX
                    };

                    // Add weapon
                    let next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index;
                    self.storage[0].next_acquisition_sort_order_index = self.storage[0].next_acquisition_sort_order_index+1;

                    let new_weapon = InventoryItemViewModel { 
                        ga_item_handle: weapon_gaitem_handle, 
                        item_id: reinforced_weapon_id, 
                        item_name: WEAPON_NAME.lock().unwrap()[&weapon_id].to_string(), 
                        quantity: 1, 
                        inventory_index: next_acquisition_sort_order_index
                    };

                    // Update gaitem_map with new weapon
                    self.gaitem_map.insert(new_weapon.ga_item_handle, GaItem{
                        gaitem_handle: new_weapon.ga_item_handle,
                        item_id: new_weapon.item_id,
                        aow_gaitem_handle: gem_handle,
                        ..Default::default()
                    });

                    // Update inventory with weapon
                    self.storage[0].weapons.push(new_weapon);
                    self.filter();
                },
                InventoryItemType::ARMOR => {

                },
                InventoryItemType::ACCESSORY => {

                },
                InventoryItemType::ITEM => {

                },
                InventoryItemType::AOW => {

                },
                InventoryItemType::None => {

                },
            }
        }
    }
}