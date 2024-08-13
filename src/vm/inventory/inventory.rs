use std::{cell::RefCell, rc::Rc};

use er_save_lib::{Item, SaveApi, SaveApiError};

use super::{
    add::add::AddViewModel,
    browse::{InventoryBrowseViewModel, Storage},
};

#[derive(Default, PartialEq, Clone)]
pub(crate) enum InventoryRoute {
    #[default]
    None,
    Add,
    Browse,
}

#[derive(Default)]
pub(crate) struct InventoryViewModel {
    // Navigation
    pub(crate) current_route: InventoryRoute,

    // Filter
    pub(crate) filter_text: String,

    // Acquistion index counter
    pub(crate) acquistion_index_counter: u32,

    // ViewModels
    pub(crate) add_vm: AddViewModel,
    pub(crate) browse_vm: InventoryBrowseViewModel,

    // Log
    pub(crate) log: Vec<String>,
}

impl InventoryViewModel {
    pub(crate) fn from_save(index: usize, save_api: &SaveApi) -> Result<Self, SaveApiError> {
        // Get regular items from the held inventory storage
        let inventory_held_regular_items = Self::prepare_items(Box::new(save_api.get_inventory(
            index,
            er_save_lib::StorageType::Held,
            er_save_lib::StorageItemType::Regular,
        )?));

        // Get key items from the held inventory storage
        let inventory_held_key_items: Vec<Rc<RefCell<Item>>> =
            Self::prepare_items(Box::new(save_api.get_inventory(
                index,
                er_save_lib::StorageType::Held,
                er_save_lib::StorageItemType::Key,
            )?));

        // Get regular items from the storage box inventory storage
        let inventory_storage_box_regular_items: Vec<Rc<RefCell<Item>>> =
            Self::prepare_items(Box::new(save_api.get_inventory(
                index,
                er_save_lib::StorageType::StorageBox,
                er_save_lib::StorageItemType::Regular,
            )?));

        // Get key items from the storage box inventory storage
        let inventory_storage_box_key_items: Vec<Rc<RefCell<Item>>> =
            Self::prepare_items(Box::new(save_api.get_inventory(
                index,
                er_save_lib::StorageType::StorageBox,
                er_save_lib::StorageItemType::Key,
            )?));

        // Create held inventory storage
        let inventory_held = Storage {
            regular_items: inventory_held_regular_items,
            key_items: inventory_held_key_items,
        };

        // Create storage box inventory storage
        let inventory_storage_box = Storage {
            regular_items: inventory_storage_box_regular_items,
            key_items: inventory_storage_box_key_items,
        };

        let add_vm = AddViewModel::from_save(save_api)?;

        // Browse View Model
        let browse_vm = InventoryBrowseViewModel {
            inventory_held,
            inventory_storage_box,
            ..Default::default()
        };

        Ok(Self {
            add_vm,
            browse_vm,
            ..Default::default()
        })
    }

    fn prepare_items(items: Box<Vec<Item>>) -> Vec<Rc<RefCell<Item>>> {
        let items = items
            .into_iter()
            .map(|item| Rc::new(RefCell::new(item)))
            .collect::<Vec<Rc<RefCell<Item>>>>();
        items
    }
}
