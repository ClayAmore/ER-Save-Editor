use std::{
    collections::HashMap,
    sync::OnceLock,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Whetblade {
    // Standard,
    BlackWhetbladeBlood,
    BlackWhetbladeOccult,
    BlackWhetbladePoison,
    GlintstoneWhetbladeFrost,
    GlintstoneWhetbladeMagic,
    IronWhetbladeHeavy,
    IronWhetbladeKeen,
    IronWhetbladeQuality,
    RedHotWhetbladeFire,
    RedHotWhetbladeFlameArt,
    SanctifiedWhetbladeLightning,
    SanctifiedWhetbladeSacred,
}

impl Whetblade {
    #[rustfmt::skip]
    pub fn whetblades() -> &'static HashMap<Whetblade, (u32, &'static str)> {
        static WHETBLADES: OnceLock<HashMap<Whetblade, (u32, &'static str)>> = OnceLock::new();

        WHETBLADES.get_or_init(|| 
            HashMap::from([
                // (Whetblade::Standard,(65600, "Upgrade - Standard")),
                (Whetblade::BlackWhetbladeBlood,(65710, "Black Whetblade (Blood)")),
                (Whetblade::BlackWhetbladeOccult,(65720, "Black Whetblade (Occult)")),
                (Whetblade::BlackWhetbladePoison,(65700, "Black Whetblade (Poison)")),
                (Whetblade::GlintstoneWhetbladeFrost,(65690, "Glintstone Whetblade (Frost)")),
                (Whetblade::GlintstoneWhetbladeMagic,(65680, "Glintstone Whetblade (Magic)")),
                (Whetblade::IronWhetbladeHeavy,(65610, "Iron Whetblade (Heavy)")),
                (Whetblade::IronWhetbladeKeen,(65620, "Iron Whetblade (Keen)")),
                (Whetblade::IronWhetbladeQuality,(65630, "Iron Whetblade (Quality)")),
                (Whetblade::RedHotWhetbladeFire,(65640, "Red-Hot Whetblade (Fire)")),
                (Whetblade::RedHotWhetbladeFlameArt,(65650, "Red-Hot Whetblade (Flame Art)")),
                (Whetblade::SanctifiedWhetbladeLightning,(65660, "Sanctified Whetblade (Lightning)")),
                (Whetblade::SanctifiedWhetbladeSacred,(65670, "Sanctified Whetblade (Sacred)")),
            ])
        )
    }
}
