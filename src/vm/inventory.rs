pub mod inventory {
    use std::cmp::{min, Ordering};
    use strsim::sorensen_dice;

    use crate::{db::{ accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME,  armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, weapon_name::weapon_name::WEAPON_NAME}, save::common::save_slot::{EquipInventoryData, EquipInventoryItem, GaItem, GaItem2, GaItemData, SaveSlot}, util::regulation::Regulation, vm::regulation::regulation_view_model::{RegulationItemViewModel, WepType}};

    #[derive(Clone)]
    pub enum InventoryRoute {
        None,
        Add,
        Browse,
    }

    #[derive(Clone)]
    pub enum InventoryTypeRoute {
        CommonItems,
        KeyItems,
        Weapons,
        Armors,
        AshOfWar,
        Talismans,
    }
    
    #[derive(PartialEq, Clone)]
    pub enum InventorySubTypeRoute {
        None,
        WeaponLeft,
        WeaponRight,
        Head,
        Body,
        Arms,
        Legs,
        Arrow,
        Bolt,
        Talisman,
        QuickItem,
        Pouch
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

    #[derive(Default, Clone, PartialEq)]
    pub enum InventoryGaitemType {
        #[default] EMPTY = -1,
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
                _ => InventoryGaitemType::EMPTY,
            }
        }
    }
 
    #[derive(Default, Clone)]
    pub struct InventoryItemViewModel  {
        pub ga_item_handle: u32,
        pub item_id: u32,
        pub item_name: String,
        pub quantity: u32,
        pub inventory_index: u32,
        pub equip_index: u32,
        pub r#type: InventoryGaitemType,
    }

    impl InventoryItemViewModel {
        pub fn from_save(item_info: &EquipInventoryItem, equip_index: u32, gaitem: &GaItem, gaitem_type: InventoryGaitemType) -> Self {
            let gaitem_handle = item_info.ga_item_handle;
            let item_type_specific = match gaitem_type {
                InventoryGaitemType::WEAPON => {
                    let id = (gaitem.item_id / 100)*100;
                    (gaitem.item_id, match WEAPON_NAME.lock().unwrap().get(&id) {
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
                InventoryGaitemType::EMPTY => panic!("We shouldn't reach this!"),
            };            

            Self {
                ga_item_handle: item_info.ga_item_handle,
                item_id: item_type_specific.0,
                item_name: item_type_specific.1,
                quantity: item_info.quantity,
                inventory_index: item_info.inventory_index,
                equip_index: equip_index,
                r#type: gaitem_type
            }
        }
    }

    #[derive(Clone)]
    pub struct InventoryStorage {
        pub common_items: Vec<InventoryItemViewModel>,
        pub key_items: Vec<InventoryItemViewModel>,

        pub filtered_items: Vec<InventoryItemViewModel>,
        pub filtered_key_items: Vec<InventoryItemViewModel>,
        pub filtered_weapons: Vec<InventoryItemViewModel>,
        pub filtered_armors: Vec<InventoryItemViewModel>,
        pub filtered_aows: Vec<InventoryItemViewModel>,
        pub filtered_projectiles: Vec<InventoryItemViewModel>,
        pub filtered_accessories: Vec<InventoryItemViewModel>,

        pub common_item_count: u32,
        pub key_item_count: u32,
        pub next_acquisition_sort_order_index: u32,
        pub next_equip_index: u32,
    }

    impl Default for InventoryStorage {
        fn default() -> Self{
            Self{ 
                common_items: Default::default(),
                key_items: Default::default(),

                filtered_items: Default::default(),
                filtered_key_items: Default::default(),
                filtered_weapons: Default::default(),
                filtered_armors: Default::default(),
                filtered_aows: Default::default(),
                filtered_projectiles: Default::default(),
                filtered_accessories: Default::default(),

                common_item_count: Default::default(),
                key_item_count: Default::default(),
                next_acquisition_sort_order_index: Default::default(),
                next_equip_index: Default::default(),
            }
        }
    }

    #[derive(Clone)]
    pub struct InventoryViewModel  {
        pub at_storage_box: bool,
        pub current_route: InventoryRoute,
        pub current_type_route: InventoryTypeRoute,
        pub current_subtype_route: InventorySubTypeRoute,
        pub filter_text: String,
        pub storage: Vec<InventoryStorage>,
        pub infusions: Vec<(i32, String)>,
        pub gaitem_map: Vec<GaItem>,
        pub gaitem_data: GaItemData,
        pub unarmed: InventoryItemViewModel,
        pub naked_head: InventoryItemViewModel,
        pub naked_body: InventoryItemViewModel,
        pub naked_arms: InventoryItemViewModel,
        pub naked_legs: InventoryItemViewModel,
        pub log: Vec<String>,

        next_gaitem_handle: u32,
        part_gaitem_handle: u8,
        next_ash_of_war_gaitem_index: usize,
        next_armament_or_armor_index: usize,
    }

    impl Default for InventoryViewModel {
        fn default() -> Self {
            Self {
                current_route: InventoryRoute::None,
                current_type_route: InventoryTypeRoute::CommonItems,
                current_subtype_route: InventorySubTypeRoute::None,
                filter_text: Default::default(),
                infusions: Default::default(),
                at_storage_box: Default::default(),
                gaitem_map: Default::default(),
                gaitem_data: Default::default(),
                unarmed: Default::default(),
                naked_head: Default::default(),
                naked_body: Default::default(),
                naked_arms: Default::default(),
                naked_legs: Default::default(),
                storage: vec![InventoryStorage::default(); 2],
                
                next_gaitem_handle: Default::default(),
                part_gaitem_handle: Default::default(),
                next_ash_of_war_gaitem_index: Default::default(),
                next_armament_or_armor_index: Default::default(),
                log: Default::default(),
            }
        }
    }

    impl InventoryViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut inventory_vm = InventoryViewModel::default();

            // Gaitem_map
            inventory_vm.gaitem_map = slot.ga_items.clone();

            // Gaitem_data
            inventory_vm.gaitem_data = slot.ga_item_data.clone();

            // Find the next gaitem_handle used when adding new weapon, armors or ashes of war
            inventory_vm.gaitem_map.iter().enumerate().for_each(| (index, gaitem)| {
                if (gaitem.gaitem_handle & 0xF0000000) == InventoryGaitemType::AOW as u32 {
                    inventory_vm.next_ash_of_war_gaitem_index = index;
                }
                if (gaitem.gaitem_handle & 0xFFFF) > (inventory_vm.next_gaitem_handle) {
                    inventory_vm.next_gaitem_handle = gaitem.gaitem_handle & 0xFFFF;
                    inventory_vm.next_armament_or_armor_index = index;
                }
            });
            inventory_vm.part_gaitem_handle = ((inventory_vm.gaitem_map[0].gaitem_handle >> 16) & 0xFF) as u8;
            
            
            inventory_vm.next_gaitem_handle = inventory_vm.next_gaitem_handle + 1;
            inventory_vm.next_ash_of_war_gaitem_index = inventory_vm.next_ash_of_war_gaitem_index + 1;
            inventory_vm.next_armament_or_armor_index = inventory_vm.next_armament_or_armor_index + 1;

            inventory_vm.fill_stroage_type(&slot.equip_inventory_data, slot.equip_inventory_data.next_acquisition_sort_id, slot.equip_inventory_data.next_equip_index,0);
            inventory_vm.fill_stroage_type(&slot.storage_inventory_data, slot.storage_inventory_data.next_acquisition_sort_id, slot.storage_inventory_data.next_equip_index ,1);
            
            inventory_vm
        }

        fn fill_stroage_type(&mut self, equip_inventory_data: &EquipInventoryData, next_acquisition_sort_id: u32, next_equip_index: u32, inventory_storage_index: usize) {
            let inventory_storage = &mut self.storage[inventory_storage_index];

            for (index, item) in equip_inventory_data.common_items.iter().enumerate() {

                // Determine item type from gaitem_handle
                let inventory_gaitem_type = InventoryGaitemType::from(item.ga_item_handle & 0xf0000000);

                // Equip_index
                let equip_index = (index as u32) + 0x180;

                match inventory_gaitem_type {
                    InventoryGaitemType::WEAPON => {
                        let gaitem = self.gaitem_map.iter().find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle).unwrap();
                        let inventory_item_vm = InventoryItemViewModel::from_save(&item, equip_index, &gaitem, InventoryGaitemType::WEAPON);
                        if inventory_item_vm.item_id == 110000 && self.unarmed.item_id != 110000 {self.unarmed = inventory_item_vm.clone();}
                        inventory_storage.common_items.push(inventory_item_vm.clone());
                        inventory_storage.filtered_weapons.push(inventory_item_vm);

                    },
                    InventoryGaitemType::ARMOR => {
                        let gaitem = self.gaitem_map.iter().find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle).unwrap();
                        let inventory_item_vm = InventoryItemViewModel::from_save(&item, equip_index, &gaitem, InventoryGaitemType::ARMOR);
                        if inventory_item_vm.item_id == 10000 {self.naked_head = inventory_item_vm.clone();}
                        else if inventory_item_vm.item_id == 10100 {self.naked_body = inventory_item_vm.clone();}
                        else if inventory_item_vm.item_id == 10200 {self.naked_arms = inventory_item_vm.clone();}
                        else if inventory_item_vm.item_id == 10300 {self.naked_legs = inventory_item_vm.clone();}
                        inventory_storage.common_items.push(inventory_item_vm.clone());
                        inventory_storage.filtered_armors.push( inventory_item_vm);
                    },
                    InventoryGaitemType::ACCESSORY => {
                        let inventory_item_vm = InventoryItemViewModel::from_save(&item, equip_index, &GaItem::default(), InventoryGaitemType::ACCESSORY);
                        inventory_storage.common_items.push(inventory_item_vm.clone());
                        inventory_storage.filtered_accessories.push( inventory_item_vm);
                    },
                    InventoryGaitemType::ITEM => {
                        let inventory_item_vm = InventoryItemViewModel::from_save(&item, equip_index, &GaItem::default(), InventoryGaitemType::ITEM);
                        inventory_storage.common_items.push(inventory_item_vm.clone());
                        inventory_storage.filtered_items.push( inventory_item_vm);
                    },
                    InventoryGaitemType::AOW => {
                        let gaitem = self.gaitem_map.iter().find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle).unwrap();
                        let inventory_item_vm = InventoryItemViewModel::from_save(&item, equip_index, &gaitem, InventoryGaitemType::AOW);
                        inventory_storage.common_items.push(inventory_item_vm.clone());
                        inventory_storage.filtered_aows.push( inventory_item_vm);
                    },
                    InventoryGaitemType::EMPTY => {
                        inventory_storage.common_items.push(InventoryItemViewModel::default());
                    },
                }
            }

            for key_item in equip_inventory_data.key_items.iter() {
                let inventory_item_vm = InventoryItemViewModel::from_save(key_item, 0, &GaItem::default(), InventoryGaitemType::ITEM);
                inventory_storage.key_items.push(inventory_item_vm);
            }

            inventory_storage.filtered_weapons.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.filtered_armors.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.filtered_accessories.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.filtered_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.filtered_key_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.filtered_aows.sort_by(|a, b| a.item_name.cmp(&b.item_name));

            inventory_storage.common_item_count = equip_inventory_data.common_inventory_items_distinct_count;
            inventory_storage.key_item_count = equip_inventory_data.key_inventory_items_distinct_count;
            inventory_storage.next_acquisition_sort_order_index = next_acquisition_sort_id;
            inventory_storage.next_equip_index = next_equip_index;

        }

        pub fn filter(&mut self) {
            for inventory_storage in &mut self.storage {
                inventory_storage.filtered_weapons = inventory_storage.common_items.iter()
                .filter(|i | {
                    if i.r#type != InventoryGaitemType::WEAPON {return false;}
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3    
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_armors = inventory_storage.common_items.iter()
                .filter(|i | {
                    if i.r#type != InventoryGaitemType::ARMOR {return false;}
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_accessories = inventory_storage.common_items.iter()
                .filter(|i | {
                    if i.r#type != InventoryGaitemType::ACCESSORY {return false;}
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_items = inventory_storage.common_items.iter()
                .filter(|i | {
                    if i.r#type != InventoryGaitemType::ITEM {return false;}
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_key_items = inventory_storage.key_items.iter()
                .filter(|i | {
                    if i.quantity == 0 { return false; }
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();
                inventory_storage.filtered_aows = inventory_storage.common_items.iter()
                .filter(|i | {
                    if i.r#type != InventoryGaitemType::AOW {return false;}
                    if self.filter_text.is_empty() { return true; }
                    let distance = sorensen_dice(&i.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    distance > 0.3
                }).map(|i| i.clone()).collect();

                inventory_storage.filtered_weapons.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.filtered_armors.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.filtered_accessories.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.filtered_items.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.filtered_key_items.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
                inventory_storage.filtered_aows.sort_by(|a, b| {
                    if self.filter_text.is_empty() {return a.item_name.cmp(&b.item_name);}
                    let distance_a = sorensen_dice(&a.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    let distance_b = sorensen_dice(&b.item_name.to_lowercase(), &self.filter_text.to_lowercase());
                    if distance_a < distance_b { return Ordering::Greater; }
                    else if distance_a > distance_b { return Ordering::Less; }
                    return Ordering::Equal;
                });
            }
        }

        pub fn filter_with_subtype(&mut self) {
            self.filter();
            match self.current_subtype_route {
                InventorySubTypeRoute::None => {},
                InventorySubTypeRoute::WeaponLeft => {
                    self.current_type_route = InventoryTypeRoute::Weapons;
                    self.storage[0].filtered_weapons = self.storage[0].filtered_weapons
                    .iter()
                    .filter(|g| 
                        Regulation::equip_weapon_params_map().get(&((g.item_id/100)*100))
                        .is_some_and(|g| g.data.leftHandEquipable())
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::WeaponRight => {
                    self.current_type_route = InventoryTypeRoute::Weapons;
                    self.storage[0].filtered_weapons = self.storage[0].filtered_weapons
                    .iter()
                    .filter(|g| 
                        Regulation::equip_weapon_params_map().get(&((g.item_id/100)*100))
                        .is_some_and(|g| g.data.rightHandEquipable())
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Head => {
                    self.current_type_route = InventoryTypeRoute::Armors;
                    self.storage[0].filtered_armors = self.storage[0].filtered_armors
                    .iter()
                    .filter(|g| 
                        Regulation::equip_protectors_param_map().get(&g.item_id)
                        .is_some_and(|g| g.data.equipModelCategory == 5)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Body => {
                    self.current_type_route = InventoryTypeRoute::Armors;
                    self.storage[0].filtered_armors = self.storage[0].filtered_armors
                    .iter()
                    .filter(|g| 
                        Regulation::equip_protectors_param_map().get(&g.item_id)
                        .is_some_and(|g| g.data.equipModelCategory == 2)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Arms => {
                    self.current_type_route = InventoryTypeRoute::Armors;
                    self.storage[0].filtered_armors = self.storage[0].filtered_armors
                    .iter()
                    .filter(|g| 
                        Regulation::equip_protectors_param_map().get(&g.item_id)
                        .is_some_and(|g| g.data.equipModelCategory == 1)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Legs => {
                    self.current_type_route = InventoryTypeRoute::Armors;
                    self.storage[0].filtered_armors = self.storage[0].filtered_armors
                    .iter()
                    .filter(|g| 
                        Regulation::equip_protectors_param_map().get(&g.item_id)
                        .is_some_and(|g| g.data.equipModelCategory == 6)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Arrow => {
                    self.current_type_route = InventoryTypeRoute::Weapons;
                    self.storage[0].filtered_weapons = self.storage[0].filtered_weapons
                    .iter()
                    .filter(|g| 
                        Regulation::equip_weapon_params_map().get(&g.item_id)
                        .is_some_and(|g| WepType::from(g.data.wepType) == WepType::Arrow || WepType::from(g.data.wepType) == WepType::Greatarrow)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Bolt => {
                    self.current_type_route = InventoryTypeRoute::Weapons;
                    self.storage[0].filtered_weapons = self.storage[0].filtered_weapons
                    .iter()
                    .filter(|g| 
                        Regulation::equip_weapon_params_map().get(&g.item_id)
                        .is_some_and(|g| WepType::from(g.data.wepType) == WepType::Bolt || WepType::from(g.data.wepType) == WepType::BallistaBolt)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
                InventorySubTypeRoute::Talisman => {
                    self.current_type_route = InventoryTypeRoute::Talismans;
                },
                InventorySubTypeRoute::QuickItem | 
                InventorySubTypeRoute::Pouch => {
                    self.current_type_route = InventoryTypeRoute::CommonItems;
                    self.storage[0].filtered_items = self.storage[0].filtered_items
                    .iter()
                    .filter(|g| 
                        Regulation::equip_goods_param_map().get(&g.item_id)
                        .is_some_and(|g| g.data.goodsType == 0)
                    )
                    .map(|i| i.clone())
                    .collect();
                },
            }
        }
    
        pub fn add_to_inventory(&mut self, item: &RegulationItemViewModel) {
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

                    // Add gaitem
                    self.gaitem_map[self.next_armament_or_armor_index] = GaItem {
                        gaitem_handle: gaitem_handle,
                        item_id: item_id,
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
                        item_id: item_id,
                        equip_index: items_next_equip_index as u32,
                        quantity: match item.quantity { Some(quantity) => quantity as u32, None => 1, },
                        inventory_index: items_next_acquisiton_sort_order_index,
                        item_name: item.name.to_string(),
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
                    self.next_ash_of_war_gaitem_index = self.next_ash_of_war_gaitem_index + 1;
                    
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
                        item_id: item_id,
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
                        item_id: item_id,
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
                        item_id: item_id,
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
                    let gaitem_handle = u32::from_be_bytes([0xB0, 0x0, item_id.to_be_bytes()[2], item_id.to_be_bytes()[3]]);

                    if item.is_key_item {
                        // Add item to common inventory
                        let held_index_res = self.storage[0].key_items.iter().position(|item| item.item_id == item_id);
                        let box_index_res = self.storage[1].key_items.iter().position(|item| item.item_id == item_id);

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
                            let key_items_next_equip_index = 0x180 + self.storage[0].key_item_count;
                            let key_items_next_acquisiton_sort_order_index = self.storage[0].next_acquisition_sort_order_index;
                            let available_quantity = item.max_held as u32;
                            self.storage[0].key_items[key_items_next_index as usize] = InventoryItemViewModel{
                                ga_item_handle: gaitem_handle,
                                item_id: item_id,
                                equip_index: key_items_next_equip_index,
                                quantity: min(quantity, available_quantity),
                                inventory_index: key_items_next_acquisiton_sort_order_index,
                                item_name: item.name.to_string(),
                                r#type: InventoryGaitemType::ITEM
                            };

                            // Increment indexes for common items
                            self.storage[0].key_item_count = self.storage[0].key_item_count + 1;
                            self.storage[0].next_equip_index = self.storage[0].next_equip_index + 1;
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
                                    item_id: item_id,
                                    equip_index: key_items_next_equip_index,
                                    quantity: min(remaining_quantity as u32, available_quantity),
                                    inventory_index: key_items_next_acquisiton_sort_order_index,
                                    item_name: item.name.to_string(),
                                    r#type: InventoryGaitemType::ITEM
                                };

                                // Increment indexes for common items
                                self.storage[1].key_item_count = self.storage[1].key_item_count + 1;
                                self.storage[1].next_equip_index = self.storage[1].next_equip_index + 1;
                                self.storage[1].next_acquisition_sort_order_index = self.storage[1].next_acquisition_sort_order_index + 1;
                            };
                        }
                    }
                    else {
                        // Add item to common inventory
                        let held_index_res = self.storage[0].common_items.iter().position(|item| item.item_id == item_id);
                        let box_index_res = self.storage[1].common_items.iter().position(|item| item.item_id == item_id);
                        
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
                                item_id: item_id,
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
                        });

                        if remaining_quantity > 0 {
                            if box_index_res.is_some() {
                                let index = box_index_res.unwrap();
                                let box_item = &mut self.storage[1].common_items[index];
                                let available_quantity = (item.max_storage as u32) - box_item.quantity;
                                box_item.quantity = box_item.quantity + available_quantity;
                            }
                            else {
                                let common_items_next_index = self.storage[1].common_item_count;
                                let common_items_next_equip_index = 0x180 + self.storage[1].common_item_count;
                                let common_items_next_acquisiton_sort_order_index = self.storage[1].next_acquisition_sort_order_index;
                                let available_quantity = item.max_storage as u32;
                                self.storage[1].common_items[common_items_next_index as usize] = InventoryItemViewModel{
                                    ga_item_handle: gaitem_handle,
                                    item_id: item_id,
                                    equip_index: common_items_next_equip_index,
                                    quantity: min(remaining_quantity as u32, available_quantity),
                                    inventory_index: common_items_next_acquisiton_sort_order_index,
                                    item_name: item.name.to_string(),
                                    r#type: InventoryGaitemType::ITEM
                                };
            
                                // Increment indexes for common items
                                self.storage[1].common_item_count = self.storage[1].common_item_count + 1;
                                self.storage[1].next_equip_index = self.storage[1].next_equip_index + 1;
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
                        item_id: item_id,
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
                        item_id: item_id,
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
            self.log.insert(0, format!("> Added {} {}", match item.quantity { Some(n) => n, None => 1 }, item.name))
        }
    }
}
