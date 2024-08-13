use std::{cell::RefCell, rc::Rc};

use er_save_lib::{EquipParamGem::EquipParamGem, EquipParamWeapon::EquipParamWeapon};

use super::item_param::ItemParam;

pub(crate) struct ParamFilter;

impl ParamFilter {
    pub(crate) fn weapons_not_infused(weapon: &&Rc<RefCell<ItemParam<EquipParamWeapon>>>) -> bool {
        weapon
            .as_ref()
            .borrow()
            .header
            .as_ref()
            .borrow()
            .item_id
            .as_ref()
            .borrow()
            .to_owned()
            % 10_000
            == 0
    }

    pub(crate) fn aows_nameless(weapon: &&Rc<RefCell<ItemParam<EquipParamGem>>>) -> bool {
        weapon
            .as_ref()
            .borrow()
            .header
            .as_ref()
            .borrow()
            .item_id
            .as_ref()
            .borrow()
            .to_owned()
            > 9999
    }
}
