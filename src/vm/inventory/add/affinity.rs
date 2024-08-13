use std::{cell::RefCell, rc::Rc};

use er_save_lib::EquipParamGem::EquipParamGem;

use super::item_param::ItemParam;

#[derive(Default, PartialEq, Debug)]
pub(crate) enum Affinity {
    Standard = 0,
    Heavy = 100,
    Keen = 200,
    Quality = 300,
    Fire = 400,
    FlameArt = 500,
    Lightning = 600,
    Sacred = 700,
    Magic = 800,
    Cold = 900,
    Poison = 1000,
    Blood = 1100,
    Occult = 1200,
    Unused13,
    Unused14,
    Unused15,
    Unused16,
    Unused17,
    Unused18,
    Unused19,
    Unused20,
    Unused21,
    Unused22,
    Unused23,
    #[default]
    Uknown,
}
impl ToString for Affinity {
    fn to_string(&self) -> String {
        match self {
            Affinity::Standard => format!("Standard"),
            Affinity::Heavy => format!("Heavy"),
            Affinity::Keen => format!("Keen"),
            Affinity::Quality => format!("Quality"),
            Affinity::Fire => format!("Fire"),
            Affinity::FlameArt => format!("FlameArt"),
            Affinity::Lightning => format!("Lightning"),
            Affinity::Sacred => format!("Sacred"),
            Affinity::Magic => format!("Magic"),
            Affinity::Cold => format!("Cold"),
            Affinity::Poison => format!("Poison"),
            Affinity::Blood => format!("Blood"),
            Affinity::Occult => format!("Occult"),
            Affinity::Unused13 => format!("Unused13"),
            Affinity::Unused14 => format!("Unused14"),
            Affinity::Unused15 => format!("Unused15"),
            Affinity::Unused16 => format!("Unused16"),
            Affinity::Unused17 => format!("Unused17"),
            Affinity::Unused18 => format!("Unused18"),
            Affinity::Unused19 => format!("Unused19"),
            Affinity::Unused20 => format!("Unused20"),
            Affinity::Unused21 => format!("Unused21"),
            Affinity::Unused22 => format!("Unused22"),
            Affinity::Unused23 => format!("Unused23"),
            Affinity::Uknown => format!("Uknown"),
        }
    }
}

impl Affinity {
    pub(crate) fn available_affinities(
        gem_param: Rc<RefCell<ItemParam<EquipParamGem>>>,
    ) -> Vec<Self> {
        let mut available_affinites = Vec::new();

        // Standard
        if gem_param.as_ref().borrow().param.configurableWepAttr00 == 1 {
            available_affinites.push(Affinity::Standard);
        }

        // Heavy
        if gem_param.as_ref().borrow().param.configurableWepAttr01 == 1 {
            available_affinites.push(Affinity::Heavy);
        }

        // Keen
        if gem_param.as_ref().borrow().param.configurableWepAttr02 == 1 {
            available_affinites.push(Affinity::Keen);
        }

        // Quality
        if gem_param.as_ref().borrow().param.configurableWepAttr03 == 1 {
            available_affinites.push(Affinity::Quality);
        }

        // Fire
        if gem_param.as_ref().borrow().param.configurableWepAttr04 == 1 {
            available_affinites.push(Affinity::Fire);
        }

        // FlameArt
        if gem_param.as_ref().borrow().param.configurableWepAttr05 == 1 {
            available_affinites.push(Affinity::FlameArt);
        }

        // Lightning
        if gem_param.as_ref().borrow().param.configurableWepAttr06 == 1 {
            available_affinites.push(Affinity::Lightning);
        }

        // Sacred
        if gem_param.as_ref().borrow().param.configurableWepAttr07 == 1 {
            available_affinites.push(Affinity::Sacred);
        }

        // Magic
        if gem_param.as_ref().borrow().param.configurableWepAttr08 == 1 {
            available_affinites.push(Affinity::Magic);
        }

        // Cold
        if gem_param.as_ref().borrow().param.configurableWepAttr09 == 1 {
            available_affinites.push(Affinity::Cold);
        }

        // Poison
        if gem_param.as_ref().borrow().param.configurableWepAttr10 == 1 {
            available_affinites.push(Affinity::Poison);
        }

        // Blood
        if gem_param.as_ref().borrow().param.configurableWepAttr11 == 1 {
            available_affinites.push(Affinity::Blood);
        }

        // Occult
        if gem_param.as_ref().borrow().param.configurableWepAttr12 == 1 {
            available_affinites.push(Affinity::Occult);
        }

        available_affinites
    }

    pub(crate) fn default_affinity(gem_param: Rc<RefCell<ItemParam<EquipParamGem>>>) -> Self {
        match gem_param.as_ref().borrow().param.defaultWepAttr {
            0 => Affinity::Standard,
            1 => Affinity::Heavy,
            2 => Affinity::Keen,
            3 => Affinity::Quality,
            4 => Affinity::Fire,
            5 => Affinity::FlameArt,
            6 => Affinity::Lightning,
            7 => Affinity::Sacred,
            8 => Affinity::Magic,
            9 => Affinity::Cold,
            10 => Affinity::Poison,
            11 => Affinity::Blood,
            12 => Affinity::Occult,
            _ => Affinity::Uknown,
        }
    }
}
