use std::{cell::RefCell, rc::Rc};

use er_save_lib::{Item, ItemType};
use strsim::sorensen_dice;

#[derive(Default, PartialEq)]
pub(crate) enum BrowseTypeRoute {
    #[default]
    RegularItems,
    KeyItems,
}

#[derive(Default, PartialEq)]
pub(crate) enum BrowseStorageType {
    #[default]
    Held,
    StorageBox,
}

#[derive(Default, PartialEq)]
pub(crate) struct Storage {
    pub(crate) regular_items: Vec<Rc<RefCell<Item>>>,
    pub(crate) key_items: Vec<Rc<RefCell<Item>>>,
}

#[derive(Default)]
pub(crate) struct InventoryBrowseViewModel {
    // Navigation
    pub(crate) current_type_route: BrowseTypeRoute,
    pub(crate) current_sub_type_route: ItemType,
    pub(crate) current_storage_type: BrowseStorageType,

    // Storage
    pub(crate) inventory_held: Storage,
    pub(crate) inventory_storage_box: Storage,
    pub(crate) current_item_list: Vec<Rc<RefCell<Item>>>,
}

impl InventoryBrowseViewModel {
    pub(crate) fn filter(&mut self, filter_text: impl Into<String>) {
        let filter_text = filter_text.into();
        let list = match (&self.current_storage_type, &self.current_type_route) {
            (BrowseStorageType::Held, BrowseTypeRoute::RegularItems) => {
                &self.inventory_held.regular_items
            }
            (BrowseStorageType::Held, BrowseTypeRoute::KeyItems) => &self.inventory_held.key_items,
            (BrowseStorageType::StorageBox, BrowseTypeRoute::RegularItems) => {
                &self.inventory_storage_box.regular_items
            }
            (BrowseStorageType::StorageBox, BrowseTypeRoute::KeyItems) => {
                &self.inventory_storage_box.key_items
            }
        };
        self.current_item_list = list
            .iter()
            .filter(|item| item.as_ref().borrow().item_type == self.current_sub_type_route)
            .filter(|item| {
                if filter_text.is_empty() {
                    return true;
                }
                let distance = sorensen_dice(
                    &item.as_ref().borrow().item_name.to_lowercase(),
                    &filter_text.to_lowercase(),
                );
                distance > 0.3
            })
            .map(|item| item.clone())
            .collect();
    }

    pub(crate) fn to_regular_items_route(
        &mut self,
        item_sub_type: ItemType,
        filter_text: impl Into<String>,
    ) {
        self.current_type_route = BrowseTypeRoute::RegularItems;
        self.current_sub_type_route = item_sub_type;
        self.filter(filter_text);
    }

    pub(crate) fn to_key_items_route(&mut self, filter_text: impl Into<String>) {
        self.current_type_route = BrowseTypeRoute::KeyItems;
        self.current_sub_type_route = ItemType::Item;
        self.filter(filter_text);
    }
}
