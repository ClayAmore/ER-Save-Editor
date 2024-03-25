use std::collections::HashMap;

use crate::{db::{self, aows::aows, armors::armor_sets, items::items, talismans::talismans, weapons::weapons}, util::regulation::Regulation, vm::regulation::regulation_view_model::{GoodsType, RegulationItemViewModel, WepType}};

use super::{InventoryItemType, InventoryTypeRoute, InventoryViewModel};

impl InventoryViewModel {
    pub fn replace_bulk_items_selected_map(&mut self, item_type: InventoryTypeRoute) {
        match item_type {
            InventoryTypeRoute::KeyItems |
            InventoryTypeRoute::CommonItems => {
                self.bulk_items_selected.clear();
                for (_, items) in items() {
                    let item_group_map =  items.iter().map(|item| (*item, false)).collect::<HashMap<u32, bool>>();
                    self.bulk_items_selected.push(item_group_map);
                }
            },
            InventoryTypeRoute::Weapons => {
                self.bulk_items_selected.clear();
                for (_, weapons) in weapons() {
                    let weapon_group_map =  weapons.iter().map(|weapon| (*weapon, false)).collect::<HashMap<u32, bool>>();
                    self.bulk_items_selected.push(weapon_group_map);
                }
            },
            InventoryTypeRoute::Armors => {
                self.bulk_items_selected.clear();
                for (_, armor_sets) in armor_sets() {
                    let armor_set_map =  armor_sets.iter().map(|weapon| (*weapon, false)).collect::<HashMap<u32, bool>>();
                    self.bulk_items_selected.push(armor_set_map);
                }
            },
            InventoryTypeRoute::AshOfWar => {
                self.bulk_items_selected.clear();
                for (_, aows) in aows() {
                    let aow_map =  aows.iter().map(|aow| (*aow, false)).collect::<HashMap<u32, bool>>();
                    self.bulk_items_selected.push(aow_map);
                }
            },
            InventoryTypeRoute::Talismans => {
                self.bulk_items_selected.clear();
                for (_, talismans) in talismans() {
                    let talisman_map =  talismans.iter().map(|weapon| (*weapon, false)).collect::<HashMap<u32, bool>>();
                    self.bulk_items_selected.push(talisman_map);
                }
            },
        }
    }

    pub fn add_all_to_inventory(&mut self) {
        // Mark this section as changed so when the file is 
        // saved then it will write this section to the file
        self.changed = true;

        let items = match self.current_bulk_type_route {
            InventoryTypeRoute::KeyItems |
            InventoryTypeRoute::CommonItems => {
                let mut items: Vec<RegulationItemViewModel> = Vec::new();
                for (index, _) in db::items::items().iter().enumerate() {
                    for (item_id, selected) in self.bulk_items_selected[index].iter_mut() {
                        if *selected {
                            let item_param = Regulation::equip_goods_param_map().get(&(item_id^InventoryItemType::ITEM as u32)).unwrap();

                            let goods_type = GoodsType::from(item_param.data.goodsType);
                            let quantity = Some({
                                if self.bulk_items_max_quantity  {
                                    (item_param.data.maxRepositoryNum) as i16
                                }
                                else {
                                    1 as i16
                                }
                            });

                            items.push(RegulationItemViewModel {
                                id: item_param.id,
                                quantity: quantity,
                                is_key_item: goods_type == GoodsType::KeyItem,
                                item_type: InventoryItemType::ITEM,
                                max_held: item_param.data.maxNum,
                                max_storage: item_param.data.maxRepositoryNum,
                                name: item_param.name.to_string(),
                                ..Default::default()
                            });
                        }
                    }
                }
                items
            },
            InventoryTypeRoute::Weapons => {
                let mut items: Vec<RegulationItemViewModel> = Vec::new();
                for (index, _) in weapons().iter().enumerate() {
                    for (weapon_id, selected) in self.bulk_items_selected[index].iter_mut() {
                        if *selected {
                            let weapon_param = Regulation::equip_weapon_params_map().get(&weapon_id).unwrap();

                            let wep_type = WepType::from(weapon_param.data.wepType);
                            let is_projectile = wep_type == WepType::Arrow || wep_type == WepType::Greatarrow || wep_type == WepType::Bolt || wep_type == WepType::BallistaBolt;
                            let quantity = if is_projectile  {
                                Some(self.bulk_items_arrow_quantity as i16)
                            }
                            else {None};

                            let is_somber = weapon_param.data.reinforceTypeId != 0 && (
                                weapon_param.data.reinforceTypeId % 2200 == 0 ||
                                weapon_param.data.reinforceTypeId % 2400 == 0 ||
                                weapon_param.data.reinforceTypeId % 3200 == 0 ||
                                weapon_param.data.reinforceTypeId % 3300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8300 == 0 ||
                                weapon_param.data.reinforceTypeId % 8500 == 0);
                            let upgrade_level = if !is_projectile {
                                if is_somber {
                                    Some(((self.bulk_items_weapon_level as f32 + 0.5)/2.5) as i16)
                                } else {Some(self.bulk_items_weapon_level as i16)}
                            }else {
                                None
                            };

                            items.push(RegulationItemViewModel {
                                id: weapon_param.id,
                                quantity: quantity,
                                item_type: InventoryItemType::WEAPON,
                                upgrade: upgrade_level,
                                ..Default::default()
                            });
                        }
                    }
                }
                items
            },
            InventoryTypeRoute::Armors => {
                let mut items: Vec<RegulationItemViewModel> = Vec::new();
                for (index, _) in armor_sets().iter().enumerate() {
                    for (armor_id, selected) in self.bulk_items_selected[index].iter_mut() {
                        if *selected {
                            let armor_param = Regulation::equip_protectors_param_map().get(&(armor_id^0x10000000)).unwrap();

                            items.push(RegulationItemViewModel {
                                id: armor_param.id,
                                name: armor_param.name.to_string(),
                                item_type: InventoryItemType::ARMOR,
                                ..Default::default()
                            });
                        }
                    }
                }
                items
            },
            InventoryTypeRoute::AshOfWar => {
                let mut items: Vec<RegulationItemViewModel> = Vec::new();
                for (index, _) in aows().iter().enumerate() {
                    for (aow_id, selected) in self.bulk_items_selected[index].iter_mut() {
                        if *selected {
                            let aow_param = Regulation::equip_gem_param_map().get(&(aow_id^0x80000000)).unwrap();

                            items.push(RegulationItemViewModel {
                                id: aow_param.id,
                                name: aow_param.name.to_string(),
                                item_type: InventoryItemType::AOW,
                                ..Default::default()
                            });
                        }
                    }
                }
                items
            },
            InventoryTypeRoute::Talismans => {
                let mut items: Vec<RegulationItemViewModel> = Vec::new();
                for (index, _) in talismans().iter().enumerate() {
                    for (talisman_id, selected) in self.bulk_items_selected[index].iter_mut() {
                        if *selected {
                            let talisman_param = Regulation::equip_accessory_param_map().get(&(talisman_id^0x20000000)).unwrap();

                            items.push(RegulationItemViewModel {
                                id: talisman_param.id,
                                name: talisman_param.name.to_string(),
                                item_type: InventoryItemType::ACCESSORY,
                                ..Default::default()
                            });
                        }
                    }
                }
                items
            },
        };

        for item in items {
            self.add_to_inventory(&item);
        }
    }
}