use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use er_save_lib::{
    EquipParamAccessory::EquipParamAccessory, EquipParamGem::EquipParamGem,
    EquipParamGoods::EquipParamGoods, EquipParamProtector::EquipParamProtector,
    EquipParamWeapon::EquipParamWeapon, ItemType, SaveApi, SaveApiError,
};
use strsim::sorensen_dice;

use super::{affinity::Affinity, filter::ParamFilter, item_param::ItemParam};

#[derive(Default, PartialEq)]
pub(crate) enum AddTypeRoute {
    #[default]
    Single,
    Bulk,
}

#[derive(Default)]
pub(crate) struct ParamHeader {
    pub(crate) item_id: Rc<RefCell<u32>>,
    pub(crate) item_name: Rc<RefCell<String>>,
    pub(crate) is_dlc_item: Rc<RefCell<bool>>,
    pub(crate) item_type: ItemType,
}

#[derive(Default, Clone)]
pub(crate) enum SelectedItem {
    #[default]
    None,
    Weapon(Rc<RefCell<ItemParam<EquipParamWeapon>>>),
    Armor(Rc<RefCell<ItemParam<EquipParamProtector>>>),
    Accessory(Rc<RefCell<ItemParam<EquipParamAccessory>>>),
    Item(Rc<RefCell<ItemParam<EquipParamGoods>>>),
    Aow(Rc<RefCell<ItemParam<EquipParamGem>>>),
}

#[derive(Default)]
pub(crate) struct AddViewModel {
    // Navigation
    pub(crate) current_type_route: AddTypeRoute,
    pub(crate) current_sub_type_route: ItemType,

    // Selected
    pub(crate) selected_header: Option<Rc<RefCell<ParamHeader>>>,
    pub(crate) selected_item: SelectedItem,

    // Add Adjustments
    pub(crate) selected_item_quantity: i16,
    pub(crate) selected_weapon_level: u8,
    pub(crate) selected_gem: u8,
    pub(crate) selected_infusion: usize,
    pub(crate) selected_affinity: usize,

    // Params
    pub(crate) items: Vec<Rc<RefCell<ItemParam<EquipParamGoods>>>>,
    pub(crate) weapons: Vec<Rc<RefCell<ItemParam<EquipParamWeapon>>>>,
    pub(crate) aows: Vec<Rc<RefCell<ItemParam<EquipParamGem>>>>,
    pub(crate) armors: Vec<Rc<RefCell<ItemParam<EquipParamProtector>>>>,
    pub(crate) talismans: Vec<Rc<RefCell<ItemParam<EquipParamAccessory>>>>,

    // List used in view
    pub(crate) current_list: Vec<Rc<RefCell<ParamHeader>>>,
    pub(crate) available_infusions: Vec<Rc<RefCell<ParamHeader>>>,
    pub(crate) available_affinities: Vec<Affinity>,
}

impl AddViewModel {
    pub(crate) fn from_save(save_api: &SaveApi) -> Result<Self, SaveApiError> {
        let items = Self::prepare_items::<EquipParamGoods>(save_api, &ItemType::Item)?;
        let weapons = Self::prepare_items::<EquipParamWeapon>(save_api, &ItemType::Weapon)?;
        let aows = Self::prepare_items::<EquipParamGem>(save_api, &ItemType::Aow)?;
        let armors = Self::prepare_items::<EquipParamProtector>(save_api, &ItemType::Armor)?;
        let talismans = Self::prepare_items::<EquipParamAccessory>(save_api, &ItemType::Accessory)?;

        Ok(Self {
            items,
            weapons,
            aows,
            armors,
            talismans,
            ..Default::default()
        })
    }

    fn prepare_items<P: er_save_lib::param_trait::Param>(
        save_api: &SaveApi,
        item_type: &ItemType,
    ) -> Result<Vec<Rc<RefCell<ItemParam<P>>>>, SaveApiError> {
        Ok(save_api
            .get_param::<P>()?
            .rows
            .into_iter()
            .map(|(item_id, param)| {
                let item_id = item_id as u32;
                Rc::new(RefCell::new(ItemParam::<P> {
                    header: Rc::new(RefCell::new(ParamHeader {
                        item_id: Rc::new(RefCell::new(item_id)),
                        item_name: Rc::new(RefCell::new(
                            if let Some(item_name) = SaveApi::get_item_name(
                                if item_type == &ItemType::Aow {
                                    (item_id / 100) as u32
                                } else {
                                    item_id as u32
                                },
                                item_type,
                            ) {
                                item_name
                            } else {
                                format!("Unk_{}", item_id)
                            },
                        )),
                        is_dlc_item: Rc::new(RefCell::new(SaveApi::is_dlc_item(
                            item_id,
                            &ItemType::Item,
                        ))),
                        item_type: item_type.clone(),
                    })),
                    param: param,
                }))
            })
            .collect())
    }

    pub(crate) fn to_route(&mut self, item_type: ItemType, filter_text: impl Into<String>) {
        self.selected_header = None;
        self.selected_item = SelectedItem::None;
        self.current_list = match item_type {
            ItemType::None => Vec::new(),
            ItemType::Weapon => {
                // Filter out infused weapons
                let weapons: Vec<Rc<RefCell<ItemParam<EquipParamWeapon>>>> = self
                    .weapons
                    .iter()
                    .filter(ParamFilter::weapons_not_infused)
                    .map(|item| item.clone())
                    .collect();
                Self::init_list(weapons.iter(), filter_text)
            }
            ItemType::Armor => Self::init_list(self.armors.iter(), filter_text),
            ItemType::Accessory => Self::init_list(self.talismans.iter(), filter_text),
            ItemType::Item => Self::init_list(self.items.iter(), filter_text),
            ItemType::Aow => {
                // Filter out nameless aows
                let aows: Vec<Rc<RefCell<ItemParam<EquipParamGem>>>> = self
                    .aows
                    .iter()
                    .filter(ParamFilter::aows_nameless)
                    .map(|item| item.clone())
                    .collect();
                Self::init_list(aows.iter(), filter_text)
            }
        };
        self.current_list.sort_by(|a, b| {
            a.as_ref()
                .borrow()
                .item_name
                .as_ref()
                .borrow()
                .cmp(&b.as_ref().borrow().item_name.as_ref().borrow())
        });
        self.current_sub_type_route = item_type;
    }

    pub(crate) fn init_list<P: er_save_lib::param_trait::Param>(
        iter: std::slice::Iter<Rc<RefCell<ItemParam<P>>>>,
        filter_text: impl Into<String>,
    ) -> Vec<Rc<RefCell<ParamHeader>>> {
        let filter_text = filter_text.into();
        iter.filter(|param| {
            if filter_text.is_empty() {
                return true;
            }
            let distance = sorensen_dice(
                &param
                    .as_ref()
                    .borrow()
                    .header
                    .as_ref()
                    .borrow()
                    .item_name
                    .as_ref()
                    .borrow()
                    .to_lowercase(),
                &filter_text.to_lowercase(),
            );

            distance > 0.3
        })
        .map(|param| param.as_ref().borrow().header.clone())
        .collect()
    }

    pub(crate) fn select_item(&mut self, param_header: Rc<RefCell<ParamHeader>>) {
        self.selected_header = Some(param_header.clone());
        match param_header.as_ref().borrow().item_type {
            ItemType::None => self.selected_item = SelectedItem::None,
            ItemType::Weapon => {
                let weapon = Self::find_param_entry(&param_header, &mut self.weapons.iter());
                self.available_infusions = self
                    .aows
                    .iter()
                    .filter(|aow| {
                        weapon
                            .as_ref()
                            .borrow()
                            .weapon_type()
                            .can_mount(&aow.as_ref().borrow().param)
                    })
                    .filter(|aow| {
                        *aow.as_ref()
                            .borrow()
                            .header
                            .as_ref()
                            .borrow()
                            .item_id
                            .as_ref()
                            .borrow()
                            > 9999
                    })
                    .map(|aow| aow.as_ref().borrow().header.clone())
                    .collect();
                self.available_infusions.insert(
                    0,
                    Rc::new(RefCell::new(ParamHeader {
                        item_name: Rc::new(RefCell::new(String::from("-- None --"))),
                        item_type: ItemType::None,
                        ..Default::default()
                    })),
                );
                self.available_infusions.sort_by(|a, b| {
                    a.as_ref()
                        .borrow()
                        .item_name
                        .cmp(&b.as_ref().borrow().item_name)
                });
                self.selected_item = SelectedItem::Weapon(weapon);
            }
            ItemType::Armor => {
                self.selected_item = SelectedItem::Armor(Self::find_param_entry(
                    &param_header,
                    &mut self.armors.iter(),
                ))
            }
            ItemType::Accessory => {
                self.selected_item = SelectedItem::Accessory(Self::find_param_entry(
                    &param_header,
                    &mut self.talismans.iter(),
                ))
            }
            ItemType::Item => {
                self.selected_item = SelectedItem::Item(Self::find_param_entry(
                    &param_header,
                    &mut self.items.iter(),
                ))
            }
            ItemType::Aow => {
                self.selected_item =
                    SelectedItem::Aow(Self::find_param_entry(&param_header, &mut self.aows.iter()))
            }
        }
    }

    fn find_param_entry<P: er_save_lib::param_trait::Param>(
        param_header: &Rc<RefCell<ParamHeader>>,
        iter: &mut std::slice::Iter<Rc<RefCell<ItemParam<P>>>>,
    ) -> Rc<RefCell<ItemParam<P>>> {
        iter.find(|item| Rc::ptr_eq(&item.as_ref().borrow().header, param_header))
            .unwrap()
            .clone()
    }

    pub(crate) fn infusion_changed(&mut self) {
        let current_infusion = &mut self.available_infusions[self.selected_infusion];
        if let Some(gem_param) = self
            .aows
            .iter()
            .find(|item_param| Rc::ptr_eq(&item_param.as_ref().borrow().header, &current_infusion))
        {
            self.available_affinities = Affinity::available_affinities(gem_param.clone());
            let default_affinity = Affinity::default_affinity(gem_param.clone());
            self.selected_affinity = self
                .available_affinities
                .iter()
                .position(|affinity| affinity == &default_affinity)
                .unwrap();
        } else {
            self.selected_affinity = 0;
            self.available_affinities = Vec::new();
        }
    }
}
