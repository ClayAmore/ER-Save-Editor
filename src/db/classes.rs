use std::{collections::HashMap, sync::LazyLock};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Default)]
pub enum ArcheType {
    #[default]
    Unknown = -1,
    Vagabond = 0,
    Warrior = 1,
    Hero = 2,
    Bandit = 3,
    Astrologer = 4,
    Prophet = 5,
    Samurai = 7,
    Prisoner = 8,
    Confessor = 6,
    Wretch = 9,
}

impl From<u8> for ArcheType {
    fn from(value: u8) -> Self {
        match value {
            x if x == ArcheType::Vagabond as u8 => ArcheType::Vagabond,
            x if x == ArcheType::Warrior as u8 => ArcheType::Warrior,
            x if x == ArcheType::Hero as u8 => ArcheType::Hero,
            x if x == ArcheType::Bandit as u8 => ArcheType::Bandit,
            x if x == ArcheType::Astrologer as u8 => ArcheType::Astrologer,
            x if x == ArcheType::Prophet as u8 => ArcheType::Prophet,
            x if x == ArcheType::Samurai as u8 => ArcheType::Samurai,
            x if x == ArcheType::Prisoner as u8 => ArcheType::Prisoner,
            x if x == ArcheType::Confessor as u8 => ArcheType::Confessor,
            x if x == ArcheType::Wretch as u8 => ArcheType::Wretch,
            _ => ArcheType::Unknown,
        }
    }
}

impl ToString for ArcheType {
    fn to_string(&self) -> String {
        match self {
            ArcheType::Unknown => "Unknown".to_string(),
            ArcheType::Vagabond => "Vagabond".to_string(),
            ArcheType::Warrior => "Warrior".to_string(),
            ArcheType::Hero => "Hero".to_string(),
            ArcheType::Bandit => "Bandit".to_string(),
            ArcheType::Astrologer => "Astrologer".to_string(),
            ArcheType::Prophet => "Prophet".to_string(),
            ArcheType::Samurai => "Samurai".to_string(),
            ArcheType::Prisoner => "Prisoner".to_string(),
            ArcheType::Confessor => "Confessor".to_string(),
            ArcheType::Wretch => "Wretch".to_string(),
        }
    }
}

pub struct Stats {
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
}

pub static STARTER_CLASSES: LazyLock<HashMap<ArcheType, Stats>> = LazyLock::new(|| {
    HashMap::from([
        (
            ArcheType::Vagabond,
            Stats {
                vigor: 15,
                mind: 10,
                endurance: 11,
                strength: 14,
                dexterity: 13,
                intelligence: 9,
                faith: 9,
                arcane: 7,
            },
        ),
        (
            ArcheType::Warrior,
            Stats {
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 10,
                dexterity: 16,
                intelligence: 10,
                faith: 8,
                arcane: 9,
            },
        ),
        (
            ArcheType::Hero,
            Stats {
                vigor: 14,
                mind: 9,
                endurance: 12,
                strength: 16,
                dexterity: 9,
                intelligence: 7,
                faith: 8,
                arcane: 11,
            },
        ),
        (
            ArcheType::Bandit,
            Stats {
                vigor: 10,
                mind: 11,
                endurance: 10,
                strength: 9,
                dexterity: 13,
                intelligence: 9,
                faith: 8,
                arcane: 14,
            },
        ),
        (
            ArcheType::Astrologer,
            Stats {
                vigor: 9,
                mind: 15,
                endurance: 9,
                strength: 8,
                dexterity: 12,
                intelligence: 16,
                faith: 7,
                arcane: 9,
            },
        ),
        (
            ArcheType::Prophet,
            Stats {
                vigor: 10,
                mind: 14,
                endurance: 8,
                strength: 11,
                dexterity: 10,
                intelligence: 7,
                faith: 16,
                arcane: 10,
            },
        ),
        (
            ArcheType::Samurai,
            Stats {
                vigor: 12,
                mind: 11,
                endurance: 13,
                strength: 12,
                dexterity: 15,
                intelligence: 9,
                faith: 8,
                arcane: 8,
            },
        ),
        (
            ArcheType::Prisoner,
            Stats {
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 11,
                dexterity: 14,
                intelligence: 14,
                faith: 6,
                arcane: 9,
            },
        ),
        (
            ArcheType::Confessor,
            Stats {
                vigor: 10,
                mind: 13,
                endurance: 10,
                strength: 12,
                dexterity: 12,
                intelligence: 9,
                faith: 14,
                arcane: 9,
            },
        ),
        (
            ArcheType::Wretch,
            Stats {
                vigor: 10,
                mind: 10,
                endurance: 10,
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                faith: 10,
                arcane: 10,
            },
        ),
    ])
});
