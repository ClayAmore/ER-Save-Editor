pub mod stats_view_model {
    use crate::{db::classes::classes::ArcheType, save::common::save_slot::SaveSlot};

    #[derive(Clone, PartialEq, Eq, Copy)]
    pub enum Gender {
        Female,
        Male,
        Uknown,
    }

    impl TryFrom<u8> for Gender {
        type Error = ();
        fn try_from(v: u8) -> Result<Self, Self::Error> {
            match v {
                x if x == Gender::Male as u8 => Ok(Gender::Male),
                x if x == Gender::Female as u8 => Ok(Gender::Female),
                _ => Err(()),
            }
        }
    }

    #[derive(Clone)]
    pub struct StatsViewModel  {
        pub gender: Gender,
        pub arche_type: ArcheType,
        pub vigor: u32,
        pub mind: u32,
        pub endurance: u32,
        pub strength: u32,
        pub dexterity: u32,
        pub intelligence: u32,
        pub faith: u32,
        pub arcane: u32,
        pub level: u32,
        pub souls: u32,
        pub soulsmemory: u32
    }

    impl Default for StatsViewModel {
        fn default() -> Self {
            Self { 
                gender: Gender::Uknown,
                arche_type: ArcheType::Unknown,
                vigor: Default::default(), 
                mind: Default::default(), 
                endurance: Default::default(), 
                strength: Default::default(), 
                dexterity: Default::default(), 
                intelligence: Default::default(), 
                faith: Default::default(), 
                arcane: Default::default(),
                level: Default::default(), 
                souls: Default::default(), 
                soulsmemory: Default::default(), 
            }
        }
    }

    impl StatsViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let gender = Gender::try_from(slot.player_game_data.gender).expect("");
            let arche_type = ArcheType::try_from(slot.player_game_data.arche_type).expect("");
            let vigor = slot.player_game_data.vigor;
            let mind = slot.player_game_data.mind;
            let endurance = slot.player_game_data.endurance;
            let strength = slot.player_game_data.strength;
            let dexterity = slot.player_game_data.dexterity;
            let intelligence = slot.player_game_data.intelligence;
            let faith = slot.player_game_data.faith;
            let arcane = slot.player_game_data.arcane;
            let level = slot.player_game_data.level;
            let souls = slot.player_game_data.souls;
            let soulsmemory = slot.player_game_data.soulsmemory;

            Self {
                gender,
                arche_type,
                vigor,
                mind,
                endurance,
                strength,
                dexterity,
                intelligence,
                faith,
                arcane,
                level,
                souls,
                soulsmemory
            }
        }
    }
}