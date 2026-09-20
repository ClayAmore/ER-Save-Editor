pub mod classes {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum ArcheType {
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
        // Added by patch 1.17 (Tarnished Pack), CharaInitParam rows 3010/3011.
        IdusKnight = 10,
        HeavyKnight = 11,
    }

    impl TryFrom<u8> for ArcheType {
        type Error = ();
        fn try_from(v: u8) -> Result<Self, Self::Error> {
            match v {
                x if x == ArcheType::Vagabond as u8 => Ok(ArcheType::Vagabond),
                x if x == ArcheType::Warrior as u8 => Ok(ArcheType::Warrior),
                x if x == ArcheType::Hero as u8 => Ok(ArcheType::Hero),
                x if x == ArcheType::Bandit as u8 => Ok(ArcheType::Bandit),
                x if x == ArcheType::Astrologer as u8 => Ok(ArcheType::Astrologer),
                x if x == ArcheType::Prophet as u8 => Ok(ArcheType::Prophet),
                x if x == ArcheType::Samurai as u8 => Ok(ArcheType::Samurai),
                x if x == ArcheType::Prisoner as u8 => Ok(ArcheType::Prisoner),
                x if x == ArcheType::Confessor as u8 => Ok(ArcheType::Confessor),
                x if x == ArcheType::Wretch as u8 => Ok(ArcheType::Wretch),
                x if x == ArcheType::IdusKnight as u8 => Ok(ArcheType::IdusKnight),
                x if x == ArcheType::HeavyKnight as u8 => Ok(ArcheType::HeavyKnight),
                _ => Err(()),
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
                ArcheType::IdusKnight => "Idus Knight".to_string(),
                ArcheType::HeavyKnight => "Heavy Knight".to_string(),
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


    
    pub static STARTER_CLASSES: Lazy<Mutex<HashMap<ArcheType,Stats>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (ArcheType::Vagabond, Stats{
                vigor: 15,
                mind: 10,
                endurance: 11,
                strength: 14,
                dexterity: 13,
                intelligence: 9,
                faith: 9,
                arcane: 7,
            }),
            
            (ArcheType::Warrior, Stats{
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 10,
                dexterity: 16,
                intelligence: 10,
                faith: 8,
                arcane: 9,
            }),
            
            (ArcheType::Hero, Stats{
                vigor: 14,
                mind: 9,
                endurance: 12,
                strength: 16,
                dexterity: 9,
                intelligence: 7,
                faith: 8,
                arcane: 11,
            }),
            
            (ArcheType::Bandit, Stats{
                vigor: 10,
                mind: 11,
                endurance: 10,
                strength: 9,
                dexterity: 13,
                intelligence: 9,
                faith: 8,
                arcane: 14,
            }),
            
            (ArcheType::Astrologer, Stats{
                vigor: 9,
                mind: 15,
                endurance: 9,
                strength: 8,
                dexterity: 12,
                intelligence: 16,
                faith: 7,
                arcane: 9,
            }),
            
            (ArcheType::Prophet, Stats{
                vigor: 10,
                mind: 14,
                endurance: 8,
                strength: 11,
                dexterity: 10,
                intelligence: 7,
                faith: 16,
                arcane: 10,
            }),
            
            (ArcheType::Samurai, Stats{
                vigor: 12,
                mind: 11,
                endurance: 13,
                strength: 12,
                dexterity: 15,
                intelligence: 9,
                faith: 8,
                arcane: 8,
            }),
            
            (ArcheType::Prisoner, Stats{
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 11,
                dexterity: 14,
                intelligence: 14,
                faith: 6,
                arcane: 9,
            }),
            
            (ArcheType::Confessor, Stats{
                vigor: 10,
                mind: 13,
                endurance: 10,
                strength: 12,
                dexterity: 12,
                intelligence: 9,
                faith: 14,
                arcane: 9,
            }),
            
            (ArcheType::Wretch, Stats{
                vigor: 10,
                mind: 10,
                endurance: 10,
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                faith: 10,
                arcane: 10,
            }),

            (ArcheType::IdusKnight, Stats{
                vigor: 10,
                mind: 12,
                endurance: 11,
                strength: 13,
                dexterity: 15,
                intelligence: 8,
                faith: 11,
                arcane: 6,
            }),

            (ArcheType::HeavyKnight, Stats{
                vigor: 14,
                mind: 8,
                endurance: 17,
                strength: 15,
                dexterity: 11,
                intelligence: 7,
                faith: 8,
                arcane: 9,
            }),

            // Stands in for a class this build does not know yet, so a save
            // from a future patch still opens. Wretch minimums are the floor
            // the game allows for every class.
            (ArcheType::Unknown, Stats{
                vigor: 10,
                mind: 10,
                endurance: 10,
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                faith: 10,
                arcane: 10,
            }),
        ]))
    });
}