pub mod inventory {
    use std::collections::HashMap;
    use crate::{db::{ accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME, armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, weapon_name::weapon_name::WEAPON_NAME}, save::common::save_slot::{EquipInventoryData, EquipInventoryItem, GaItem, SaveSlot}};

    #[derive(Clone)]
    pub enum InventoryRoute {
        None,
        CommonItems,
        KeyItems,
        Weapons,
        Armors,
        AshOfWar,
        Talismans,
    }

    pub enum InventoryItemType {
        _WEAPON = 0x0,
        ARMOR = 0x10000000,
        _ACCESSORY = 0x20000000,
        _ITEM = 0x40000000,
        AOW = 0x80000000,
    }

    pub enum InventoryGaitemType {
        WEAPON = 0x80000000,
        ARMOR = 0x90000000,
        ACCESSORY = 0xa0000000,
        ITEM = 0xb0000000,
        AOW = 0xc0000000,
    }

    impl TryFrom<u32> for InventoryGaitemType {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                x if x == InventoryGaitemType::WEAPON as u32 => Ok(InventoryGaitemType::WEAPON),
                x if x == InventoryGaitemType::ARMOR as u32 => Ok(InventoryGaitemType::ARMOR),
                x if x == InventoryGaitemType::ACCESSORY as u32 => Ok(InventoryGaitemType::ACCESSORY),
                x if x == InventoryGaitemType::ITEM as u32 => Ok(InventoryGaitemType::ITEM),
                x if x == InventoryGaitemType::AOW as u32 => Ok(InventoryGaitemType::AOW),
                _ => Err(()),
            }
        }
    }

 
    #[derive(Clone)]
    pub struct InventoryItemViewModel  {
        pub ga_item_handle: i32,
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
                    (id, String::from(WEAPON_NAME.lock().unwrap()[&id]))
                },
                InventoryGaitemType::ARMOR => {
                    let id = gaitem.item_id ^ InventoryItemType::ARMOR as u32;
                    (id, String::from(ARMOR_NAME.lock().unwrap()[&id]))
                },
                InventoryGaitemType::ACCESSORY => {
                    let id = gaitem_handle ^ InventoryGaitemType::ACCESSORY as u32;
                    (id, String::from(ACCESSORY_NAME.lock().unwrap()[&id]))
                },
                InventoryGaitemType::ITEM => {
                    let id = gaitem_handle ^ InventoryGaitemType::ITEM as u32;
                    (id, String::from(ITEM_NAME.lock().unwrap()[&id]))
                },
                InventoryGaitemType::AOW => {
                    let id = gaitem.item_id ^ InventoryItemType::AOW as u32;
                    (id, String::from(AOW_NAME.lock().unwrap()[&id]))
                },
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
                accessories: Default::default()
            }
        }
    }

    #[derive(Clone)]
    pub struct InventoryViewModel  {
        pub at_storage_box: bool,
        pub current_route: InventoryRoute,
        pub filter_text: String,
        pub storage: Vec<InventoryStorage>,
    }

    impl Default for InventoryViewModel {
        fn default() -> Self {
            Self {
                at_storage_box: Default::default(),
                current_route: InventoryRoute::None,
                filter_text: Default::default(),
                storage: vec![InventoryStorage::default(); 2]
            }
        }
    }

    impl InventoryViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut inventory_vm = InventoryViewModel::default();

            let gaitem_map: HashMap<i32, &GaItem> = slot.ga_items.iter().map(|g| (g.gaitem_handle, g)).collect();

            Self::fill_stroage_type(&slot.equip_inventory_data, &gaitem_map, &mut inventory_vm.storage[0]);
            Self::fill_stroage_type(&slot.storage_inventory_data, &gaitem_map, &mut inventory_vm.storage[1]);
            
            inventory_vm
        }

        fn fill_stroage_type(equip_inventory_data: &EquipInventoryData, gaitem_map: &HashMap<i32,&GaItem>, inventory_storage: &mut InventoryStorage) {
            for i in 0..equip_inventory_data.common_inventory_items_distinct_count {
                let equip_invenotry_item = &equip_inventory_data.common_items[i as usize];

                if equip_invenotry_item.ga_item_handle == -1 || equip_invenotry_item.ga_item_handle == 0 { continue; };

                let inventory_gaitem_type = InventoryGaitemType::try_from(equip_invenotry_item.ga_item_handle as u32 & 0xf0000000).expect("");

                match inventory_gaitem_type {
                    InventoryGaitemType::WEAPON => {
                        let gaitem = gaitem_map[&equip_invenotry_item.ga_item_handle];
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &gaitem, InventoryGaitemType::WEAPON);
                        inventory_storage.weapons.push(inventory_item_vm);
                    },
                    InventoryGaitemType::ARMOR => {
                        let gaitem = gaitem_map[&equip_invenotry_item.ga_item_handle];
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
                        let gaitem = gaitem_map[&equip_invenotry_item.ga_item_handle];
                        let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &gaitem, InventoryGaitemType::AOW);
                        inventory_storage.aows.push( inventory_item_vm);
                    },
                }
            }

            for i in 0..equip_inventory_data.key_inventory_items_distinct_count {
                let equip_invenotry_item = &equip_inventory_data.key_items[i as usize];
                if equip_invenotry_item.ga_item_handle == -1 || equip_invenotry_item.ga_item_handle == 0 { continue; };
                let inventory_item_vm = InventoryItemViewModel::from_save(equip_invenotry_item, &GaItem::default(), InventoryGaitemType::ITEM);
                inventory_storage.key_items.push( inventory_item_vm);
            }

            inventory_storage.weapons.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.armors.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.common_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.key_items.sort_by(|a, b| a.item_name.cmp(&b.item_name));
            inventory_storage.aows.sort_by(|a, b| a.item_name.cmp(&b.item_name));
        }


    }
}