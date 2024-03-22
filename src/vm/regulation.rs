pub mod regulation_view_model {
    use std::cmp::Ordering;
    use strsim::sorensen_dice;
    use crate::{util::regulation::Regulation, vm::inventory::{InventoryItemType, InventoryTypeRoute}};
    

    #[derive(Clone, PartialEq)]
    pub enum ProtectorCategory {
        Head,
        Body,
        Arms,
        Legs,
        Hair,
    }
    impl TryFrom<u8> for ProtectorCategory {
        type Error = String;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(ProtectorCategory::Head),
                1 => Ok(ProtectorCategory::Body),
                2 => Ok(ProtectorCategory::Arms),
                3 => Ok(ProtectorCategory::Legs),
                4 => Ok(ProtectorCategory::Hair),
                _ => Err(format!("Cannot recognize protector category {}", value))
            }
        }
    } 

    #[derive(Clone, Default, PartialEq)]
    pub enum WepType {

        #[default] None = 0,
        Dagger = 1,
        StraightSword = 3,
        Greatsword = 5,
        ColossalSword = 7,
        CurvedSword = 9,
        CurvedGreatsword = 11,
        Katana = 13,
        Twinblade = 14,
        ThrustingSword = 15,
        HeavyThrustingSword = 16,
        Axe = 17,
        Greataxe = 19,
        Hammer = 21,
        GreatHammer = 23,
        Flail = 24,
        Spear = 25,
        HeavySpear = 28,
        Halberd = 29,
        Scythe = 31,
        Fist = 35,
        Claw = 37,
        Whip = 39,
        ColossalWeapon = 41,
        LightBow = 50,
        Bow = 51,
        Greatbow = 53,
        Crossbow = 55,
        Ballista = 56,
        Staff = 57,
        Seal = 61,
        SmallShield = 65,
        MediumShield = 67,
        Greatshield = 69,
        Arrow = 81,
        Greatarrow = 83,
        Bolt = 85,
        BallistaBolt = 86,
        Torch = 87,
        Unknown = 99
    }
    impl From<i16> for WepType {
        fn from(value: i16) -> Self {
            match value {
                0 => WepType::None, 
                1 => WepType::Dagger, 
                3 => WepType::StraightSword, 
                5 => WepType::Greatsword, 
                7 => WepType::ColossalSword, 
                9 => WepType::CurvedSword, 
                11 => WepType::CurvedGreatsword, 
                13 => WepType::Katana, 
                14 => WepType::Twinblade, 
                15 => WepType::ThrustingSword, 
                16 => WepType::HeavyThrustingSword, 
                17 => WepType::Axe, 
                19 => WepType::Greataxe, 
                21 => WepType::Hammer, 
                23 => WepType::GreatHammer, 
                24 => WepType::Flail, 
                25 => WepType::Spear, 
                28 => WepType::HeavySpear, 
                29 => WepType::Halberd, 
                31 => WepType::Scythe, 
                35 => WepType::Fist, 
                37 => WepType::Claw, 
                39 => WepType::Whip, 
                41 => WepType::ColossalWeapon, 
                50 => WepType::LightBow, 
                51 => WepType::Bow, 
                53 => WepType::Greatbow, 
                55 => WepType::Crossbow, 
                56 => WepType::Ballista, 
                57 => WepType::Staff, 
                61 => WepType::Seal, 
                65 => WepType::SmallShield, 
                67 => WepType::MediumShield, 
                69 => WepType::Greatshield, 
                81 => WepType::Arrow, 
                83 => WepType::Greatarrow, 
                85 => WepType::Bolt, 
                86 => WepType::BallistaBolt, 
                87 => WepType::Torch, 
                _ => WepType::Unknown
            }
        }
    }

    #[derive(Copy, Clone, Default)]
    pub enum Affinity {
        #[default]Standard = 0,
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
    }
    impl Affinity {
        pub fn as_i16(&self) -> i16 {
            *self as i16
        }
    }
    impl ToString for Affinity {
        fn to_string(&self) -> String {
            match self {
                Affinity::Standard => "Standard".to_string(),
                Affinity::Heavy => "Heavy".to_string(),
                Affinity::Keen => "Keen".to_string(),
                Affinity::Quality => "Quality".to_string(),
                Affinity::Fire => "Fire".to_string(),
                Affinity::FlameArt => "FlameArt".to_string(),
                Affinity::Lightning => "Lightning".to_string(),
                Affinity::Sacred => "Sacred".to_string(),
                Affinity::Magic => "Magic".to_string(),
                Affinity::Cold => "Cold".to_string(),
                Affinity::Poison => "Poison".to_string(),
                Affinity::Blood => "Blood".to_string(),
                Affinity::Occult => "Occult".to_string(),
                Affinity::Unused13 => "Unused13".to_string(),
                Affinity::Unused14 => "Unused14".to_string(),
                Affinity::Unused15 => "Unused15".to_string(),
                Affinity::Unused16 => "Unused16".to_string(),
                Affinity::Unused17 => "Unused17".to_string(),
                Affinity::Unused18 => "Unused18".to_string(),
                Affinity::Unused19 => "Unused19".to_string(),
                Affinity::Unused20 => "Unused20".to_string(),
                Affinity::Unused21 => "Unused21".to_string(),
                Affinity::Unused22 => "Unused22".to_string(),
                Affinity::Unused23 => "Unused23".to_string(),
            }
        }
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum GoodsType {
        NormalItem,
        KeyItem,
        CraftingMaterial,
        Rememberance,
        Sorcery = 5,
        SpiritSummonLesser = 7,
        SpiritSummonGreater,
        WonderousPhysics,
        WonderousPhysicsTears,
        RegenerativeMaterial,
        InfoItem,
        ReinforcementMaterial = 14,
        GreatRune,
        Incantation,
        SelfBuffSorcery,
        SelfBuffIncanation,
        Unknown
    }
    impl From<u8> for GoodsType {
        fn from(value: u8) -> Self {
            match value {
                0 => GoodsType::NormalItem, 
                1 => GoodsType::KeyItem, 
                2 => GoodsType::CraftingMaterial, 
                3 => GoodsType::Rememberance, 
                5 => GoodsType::Sorcery, 
                7 => GoodsType::SpiritSummonLesser, 
                8 => GoodsType::SpiritSummonGreater, 
                9 => GoodsType::WonderousPhysics, 
                10 => GoodsType::WonderousPhysicsTears, 
                11 => GoodsType::RegenerativeMaterial, 
                12 => GoodsType::InfoItem, 
                14 => GoodsType::ReinforcementMaterial, 
                15 => GoodsType::GreatRune, 
                16 => GoodsType::Incantation, 
                17 => GoodsType::SelfBuffSorcery, 
                18 => GoodsType::SelfBuffIncanation, 
                _ => GoodsType::Unknown
            }
        }
    }
    
    #[derive(Default, Clone)]
    pub struct RegulationItemViewModel {
        pub id: u32,
        pub name: String,
        pub max_held: i16,
        pub max_storage: i16,
        pub infusable: bool,
        pub is_key_item: bool,
        pub item_type: InventoryItemType,
        pub wep_type: Option<WepType>, 
        pub quantity: Option<i16>,
        pub upgrade: Option<i16>,
        pub infusion: Option<u32>,
        pub affinity: Option<i16>,
    }

    #[derive(Default, Clone)]
    pub struct RegulationViewModel  {
        pub filtered_goods: Vec<RegulationItemViewModel>,
        pub filtered_weapons: Vec<RegulationItemViewModel>,
        pub filtered_protectors: Vec<RegulationItemViewModel>,
        pub filtered_gems: Vec<RegulationItemViewModel>,
        pub filtered_accessories: Vec<RegulationItemViewModel>,

        pub selected_item: RegulationItemViewModel,

        pub selected_infusion: usize,
        pub available_infusions: Vec<RegulationItemViewModel>,

        pub selected_affinity: usize,
        pub available_affinities: Vec<Affinity>,
    }

    impl RegulationViewModel {
        pub fn update_available_infusions(&mut self){
            self.selected_infusion = 0;
            self.available_infusions.clear();
            self.available_infusions.push(RegulationItemViewModel{
                id: u32::MAX,
                name: "-".to_string(),
                ..Default::default()
            });
            if self.selected_item.wep_type.is_none() || !self.selected_item.infusable {
                return;
            }

            let wep_type = self.selected_item.wep_type.as_ref();
            self.available_infusions.extend(Regulation::equip_gem_param_map()
            .iter()
            .filter(|(_, gem)|{
                match wep_type.unwrap() {
                    WepType::Dagger => {
                        gem.data.canMountWep_Dagger()
                    },
                    WepType::StraightSword => {
                        gem.data.canMountWep_SwordNormal()
                    },
                    WepType::Greatsword => {
                        gem.data.canMountWep_SwordLarge()
                    },
                    WepType::ColossalSword => {
                        gem.data.canMountWep_SwordGigantic()
                    },
                    WepType::CurvedSword => {
                        gem.data.canMountWep_SaberNormal()
                    },
                    WepType::CurvedGreatsword => {
                        gem.data.canMountWep_SaberLarge()
                    },
                    WepType::Katana => {
                        gem.data.canMountWep_katana()
                    },
                    WepType::Twinblade => {
                        gem.data.canMountWep_SwordDoubleEdge()
                    },
                    WepType::ThrustingSword => {
                        gem.data.canMountWep_SwordPierce()
                    },
                    WepType::HeavyThrustingSword => {
                        gem.data.canMountWep_RapierHeavy()
                    },
                    WepType::Axe => {
                        gem.data.canMountWep_AxeNormal()
                    },
                    WepType::Greataxe => {
                        gem.data.canMountWep_AxeLarge()
                    },
                    WepType::Hammer => {
                        gem.data.canMountWep_HammerNormal()
                    },
                    WepType::GreatHammer => {
                        gem.data.canMountWep_HammerLarge()
                    },
                    WepType::Flail => {
                        gem.data.canMountWep_Flail()
                    },
                    WepType::Spear => {
                        gem.data.canMountWep_SpearNormal()
                    },
                    WepType::HeavySpear => {
                        gem.data.canMountWep_SpearHeavy()
                    },
                    WepType::Halberd => {
                        gem.data.canMountWep_SpearAxe()
                    },
                    WepType::Scythe => {
                        gem.data.canMountWep_Sickle()
                    },
                    WepType::Fist => {
                        gem.data.canMountWep_Knuckle()
                    },
                    WepType::Claw => {
                        gem.data.canMountWep_Claw()
                    },
                    WepType::Whip => {
                        gem.data.canMountWep_Whip()
                    },
                    WepType::ColossalWeapon => {
                        gem.data.canMountWep_AxhammerLarge()
                    },
                    WepType::LightBow => {
                        gem.data.canMountWep_BowSmall()
                    },
                    WepType::Bow => {
                        gem.data.canMountWep_BowNormal()
                    },
                    WepType::Greatbow => {
                        gem.data.canMountWep_BowLarge()
                    },
                    WepType::Crossbow => {
                        gem.data.canMountWep_ClossBow()
                    },
                    WepType::Ballista => {
                        gem.data.canMountWep_Ballista()
                    },
                    WepType::Staff => {
                        gem.data.canMountWep_Staff()
                    },
                    WepType::Seal => {
                        gem.data.canMountWep_Talisman()
                    },
                    WepType::SmallShield => {
                        gem.data.canMountWep_ShieldSmall()
                    },
                    WepType::MediumShield => {
                        gem.data.canMountWep_ShieldNormal()
                    },
                    WepType::Greatshield => {
                        gem.data.canMountWep_ShieldLarge()
                    },
                    WepType::Torch => {
                        gem.data.canMountWep_Torch()
                    },
                    WepType::None |
                    WepType::Arrow |
                    WepType::Greatarrow |
                    WepType::Bolt |
                    WepType::BallistaBolt |
                    WepType::Unknown => {
                        false
                    },
                }
            })
            .map(|(_, gem)| RegulationItemViewModel{
                id: gem.id,
                name: gem.name.to_string(),
                max_held: 1,
                max_storage: 1,
                ..Default::default()
            }).filter(|gem|{
                gem.id > 10000
            }).collect::<Vec<RegulationItemViewModel>>());
        }

        pub fn update_available_affinities(&mut self) {
            self.available_affinities.clear();
            self.selected_affinity = 0;
            if self.available_infusions.len() == 0 {
                return;
            }
            let res = Regulation::equip_gem_param_map().get(&self.available_infusions[self.selected_infusion].id);
            if res.is_some() {
                let gem = res.unwrap();
                if gem.data.configurableWepAttr00() {self.available_affinities.push(Affinity::Standard);}
                if gem.data.configurableWepAttr01() {self.available_affinities.push(Affinity::Heavy);}
                if gem.data.configurableWepAttr02() {self.available_affinities.push(Affinity::Keen);}
                if gem.data.configurableWepAttr03() {self.available_affinities.push(Affinity::Quality);}
                if gem.data.configurableWepAttr04() {self.available_affinities.push(Affinity::Fire);}
                if gem.data.configurableWepAttr05() {self.available_affinities.push(Affinity::FlameArt);}
                if gem.data.configurableWepAttr06() {self.available_affinities.push(Affinity::Lightning);}
                if gem.data.configurableWepAttr07() {self.available_affinities.push(Affinity::Sacred);}
                if gem.data.configurableWepAttr08() {self.available_affinities.push(Affinity::Magic);}
                if gem.data.configurableWepAttr09() {self.available_affinities.push(Affinity::Cold);}
                if gem.data.configurableWepAttr10() {self.available_affinities.push(Affinity::Poison);}
                if gem.data.configurableWepAttr11() {self.available_affinities.push(Affinity::Blood);}
                if gem.data.configurableWepAttr12() {self.available_affinities.push(Affinity::Occult);}
                if gem.data.configurableWepAttr13() {self.available_affinities.push(Affinity::Unused13);}
                if gem.data.configurableWepAttr14() {self.available_affinities.push(Affinity::Unused14);}
                if gem.data.configurableWepAttr15() {self.available_affinities.push(Affinity::Unused15);}
                if gem.data.configurableWepAttr16() {self.available_affinities.push(Affinity::Unused16);}
                if gem.data.configurableWepAttr17() {self.available_affinities.push(Affinity::Unused17);}
                if gem.data.configurableWepAttr18() {self.available_affinities.push(Affinity::Unused18);}
                if gem.data.configurableWepAttr19() {self.available_affinities.push(Affinity::Unused19);}
                if gem.data.configurableWepAttr20() {self.available_affinities.push(Affinity::Unused20);}
                if gem.data.configurableWepAttr21() {self.available_affinities.push(Affinity::Unused21);}
                if gem.data.configurableWepAttr22() {self.available_affinities.push(Affinity::Unused22);}
                if gem.data.configurableWepAttr23() {self.available_affinities.push(Affinity::Unused23);}
            }
        }

        pub fn filter(&mut self, inventory_type: &InventoryTypeRoute, filter_text: &str) {
            match inventory_type {
                InventoryTypeRoute::KeyItems |
                InventoryTypeRoute::CommonItems => {
                    // Compiling a list of item replacement ids to avoid showing double items such as, tarnished furled finger + used tarnished furled finger.  
                    let replacement_items = Regulation::equip_goods_param_map()
                    .iter()
                    .filter(|(_, good)| good.data.appearanceReplaceItemId != -1)
                    .map(|(_, good)| good.data.appearanceReplaceItemId as u32)
                    .collect::<Vec<u32>>();
                    
                    self.filtered_goods = Regulation::equip_goods_param_map()
                    .iter()
                    .filter(|(_, good)| good.data.goodsUseAnim != 254)
                    .filter(|(_, good)| good.id < 1001 || good.id > 1025)
                    .filter(|(_, good)| good.id < 1051 || good.id > 1075)
                    .map(|(_, good)| RegulationItemViewModel{
                        id: good.id,
                        name: good.name.to_string(),
                        max_held: good.data.maxNum,
                        max_storage: good.data.maxRepositoryNum,
                        wep_type: None,
                        quantity: Some(good.data.maxRepositoryNum),
                        is_key_item: GoodsType::from(good.data.goodsType) == GoodsType::KeyItem,
                        item_type: InventoryItemType::ITEM,
                        ..Default::default()
                    }).filter(|reg_item_vm|{
                        if filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&reg_item_vm.name.to_lowercase(), &filter_text.to_lowercase());
                        distance > 0.3 
                    }).filter(|reg_item_vm|{
                        !replacement_items.contains(&reg_item_vm.id)
                    })
                    .filter(|reg_item_vm|
                        reg_item_vm.id > 9100 || reg_item_vm.id < 9000
                    )
                    .collect::<Vec<RegulationItemViewModel>>(); 

                    self.filtered_goods.sort_by(|a,b| {
                        if filter_text.is_empty() {
                            return a.name.cmp(&b.name);
                        }
                        let distance_a = sorensen_dice(&a.name.to_lowercase(), &filter_text.to_lowercase());
                        let distance_b = sorensen_dice(&b.name.to_lowercase(), &filter_text.to_lowercase());
                        if distance_a < distance_b { return Ordering::Greater; }
                        else if distance_a > distance_b { return Ordering::Less; }
                        return Ordering::Equal;
                    })
                },
                InventoryTypeRoute::Weapons => {
                    self.filtered_weapons = Regulation::equip_weapon_params_map()
                    .iter()
                    .map(|(_,weapon)| RegulationItemViewModel{
                        id: weapon.id,
                        name: weapon.name.to_string(),
                        max_held: 1,
                        max_storage: 1,
                        infusable: weapon.data.gemMountType == 2,
                        item_type: InventoryItemType::WEAPON,
                        upgrade: Some(0),
                        wep_type: Some(WepType::from(weapon.data.wepType)),
                        quantity: if WepType::from(weapon.data.wepType) == WepType::Arrow 
                        || WepType::from(weapon.data.wepType) == WepType::Greatarrow
                        || WepType::from(weapon.data.wepType) == WepType::Bolt
                        || WepType::from(weapon.data.wepType) == WepType::BallistaBolt {
                            Some(weapon.data.maxArrowQuantity as i16)
                        } else {None},
                        ..Default::default()
                    }).filter(|i|{
                        if filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&i.name.to_lowercase(), &filter_text.to_lowercase());
                        distance > 0.3 
                    }).filter(|i|{
                        i.id % 10_000 == 0
                    }).collect::<Vec<RegulationItemViewModel>>();

                    self.filtered_weapons.sort_by(|a,b| {
                        if filter_text.is_empty() {
                            return a.name.cmp(&b.name);
                        }
                        let distance_a = sorensen_dice(&a.name.to_lowercase(), &filter_text.to_lowercase());
                        let distance_b = sorensen_dice(&b.name.to_lowercase(), &filter_text.to_lowercase());
                        if distance_a < distance_b { return Ordering::Greater; }
                        else if distance_a > distance_b { return Ordering::Less; }
                        return Ordering::Equal;
                    })
                },
                InventoryTypeRoute::Armors => {
                    self.filtered_protectors = Regulation::equip_protectors_param_map()
                    .iter()
                    .map(|(_, protector)| RegulationItemViewModel{
                        id: protector.id,
                        name: protector.name.to_string(),
                        max_held: 1,
                        max_storage: 1,
                        item_type: InventoryItemType::ARMOR,
                        ..Default::default()
                    }).filter(|reg_item_vm|{
                        if filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&reg_item_vm.name.to_lowercase(), &filter_text.to_lowercase());
                        distance > 0.3 
                    }).filter(|reg_item_vm|{
                        reg_item_vm.id > 40000
                    }).collect::<Vec<RegulationItemViewModel>>();

                    self.filtered_protectors.sort_by(|a,b| {
                        if filter_text.is_empty() {
                            return a.name.cmp(&b.name);
                        }
                        let distance_a = sorensen_dice(&a.name.to_lowercase(), &filter_text.to_lowercase());
                        let distance_b = sorensen_dice(&b.name.to_lowercase(), &filter_text.to_lowercase());
                        if distance_a < distance_b { return Ordering::Greater; }
                        else if distance_a > distance_b { return Ordering::Less; }
                        return Ordering::Equal;
                    })
                },
                InventoryTypeRoute::AshOfWar => {
                    self.filtered_gems = Regulation::equip_gem_param_map()
                    .iter()
                    .map(|(_, gem)| RegulationItemViewModel{
                        id: gem.id,
                        name: gem.name.to_string(),
                        max_held: 1,
                        max_storage: 1,
                        item_type: InventoryItemType::AOW,
                        ..Default::default()
                    }).filter(|reg_item_vm|{
                        if filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&reg_item_vm.name.to_lowercase(), &filter_text.to_lowercase());
                        distance > 0.3 
                    }).filter(|reg_item_vm|{
                        reg_item_vm.id > 10000
                    }).collect::<Vec<RegulationItemViewModel>>();

                    self.filtered_gems.sort_by(|a,b| {
                        if filter_text.is_empty() {
                            return a.name.cmp(&b.name);
                        }
                        let distance_a = sorensen_dice(&a.name.to_lowercase(), &filter_text.to_lowercase());
                        let distance_b = sorensen_dice(&b.name.to_lowercase(), &filter_text.to_lowercase());
                        if distance_a < distance_b { return Ordering::Greater; }
                        else if distance_a > distance_b { return Ordering::Less; }
                        return Ordering::Equal;
                    })
                },
                InventoryTypeRoute::Talismans => {
                    self.filtered_accessories = Regulation::equip_accessory_param_map()
                    .iter()
                    .map(|(_, accessory)| RegulationItemViewModel{
                        id: accessory.id,
                        name: accessory.name.to_string(),
                        max_held: 1,
                        max_storage: 1,
                        item_type: InventoryItemType::ACCESSORY,
                        ..Default::default()
                    }).filter(|reg_item_vm|{
                        if filter_text.is_empty() { return true; }
                        let distance = sorensen_dice(&reg_item_vm.name.to_lowercase(), &filter_text.to_lowercase());
                        distance > 0.3 
                    }).collect::<Vec<RegulationItemViewModel>>();

                    self.filtered_accessories.sort_by(|a,b| {
                        if filter_text.is_empty() {
                            return a.name.cmp(&b.name);
                        }
                        let distance_a = sorensen_dice(&a.name.to_lowercase(), &filter_text.to_lowercase());
                        let distance_b = sorensen_dice(&b.name.to_lowercase(), &filter_text.to_lowercase());
                        if distance_a < distance_b { return Ordering::Greater; }
                        else if distance_a > distance_b { return Ordering::Less; }
                        return Ordering::Equal;
                    })
                },
            }
        }
    }
}