use std::{cell::RefCell, rc::Rc};

use er_save_lib::EquipParamWeapon::EquipParamWeapon;

use super::{add::ParamHeader, weapon_type::WeaponType};

#[derive(Default)]
pub(crate) struct ItemParam<P: er_save_lib::param_trait::Param> {
    pub(crate) header: Rc<RefCell<ParamHeader>>,
    pub(crate) param: P::ParamType,
}

impl ItemParam<EquipParamWeapon> {
    pub(crate) fn is_infusable(&self) -> bool {
        self.param.gemMountType == 2
    }

    pub(crate) fn weapon_type(&self) -> WeaponType {
        WeaponType::from(self.param.wepType)
    }
}
