pub mod general_view_model {
    use crate::save::common::save_slot::SaveSlot;
    use crate::util::bit::bit::get_bit;

    // Patch 1.17 records which Spectral Steed Attire is applied to Torrent as
    // one of three event flags: 6701 Tree Sentinel, 6702 Silver of Caria and
    // 6703 Funereal Night. They share one byte of the flag block, a bit each,
    // and no bit set means the plain Torrent.
    pub const STEED_ATTIRE_FLAG_BYTE: usize = 0x345;

    #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
    pub enum SteedAttire {
        #[default] None,
        TreeSentinel,
        SilverOfCaria,
        FunerealNight,
    }

    impl SteedAttire {
        pub const ALL: [SteedAttire; 4] = [
            SteedAttire::None,
            SteedAttire::TreeSentinel,
            SteedAttire::SilverOfCaria,
            SteedAttire::FunerealNight,
        ];

        // Bit within STEED_ATTIRE_FLAG_BYTE, or None for the plain Torrent.
        pub fn bit(&self) -> Option<u8> {
            match self {
                SteedAttire::None => Option::None,
                SteedAttire::FunerealNight => Some(0),
                SteedAttire::SilverOfCaria => Some(1),
                SteedAttire::TreeSentinel => Some(2),
            }
        }

        pub fn label(&self) -> &'static str {
            match self {
                SteedAttire::None => "Default (no attire)",
                SteedAttire::TreeSentinel => "Tree Sentinel",
                SteedAttire::SilverOfCaria => "Silver of Caria",
                SteedAttire::FunerealNight => "Funereal Night",
            }
        }

        pub fn from_flags(flags: &[u8]) -> SteedAttire {
            let byte = match flags.get(STEED_ATTIRE_FLAG_BYTE) {
                Some(byte) => *byte,
                Option::None => return SteedAttire::None,
            };
            for attire in SteedAttire::ALL {
                if attire.bit().is_some_and(|bit| get_bit(byte, bit)) {
                    return attire;
                }
            }
            SteedAttire::None
        }
    }

    #[derive(Default, Clone, PartialEq, Eq, Copy)]
    pub enum Gender {
        Female,
        Male,
        #[default]Uknown,
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

    #[derive(Default, Clone)]
    pub struct GeneralViewModel  {
        pub character_name: String,
        pub gender: Gender,
        pub weapon_level: u8,
        pub steed_attire: SteedAttire,
    }

    impl GeneralViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            // Character Name
            let character_name = slot.player_game_data.character_name;
            let mut character_name_trimmed: [u16; 0x10] = [0;0x10];
            for (i, char) in character_name.iter().enumerate() {
                if *char == 0 { break; }
                character_name_trimmed[i] = *char;
            }
            let character_name = String::from_utf16(&character_name_trimmed).expect("");

            // Gender
            let gender = Gender::try_from(slot.player_game_data.gender).expect("");

            // Weapon Level
            let weapon_level = slot.player_game_data.match_making_wpn_lvl;

            // Torrent appearance
            let steed_attire = SteedAttire::from_flags(&slot.event_flags.flags);

            Self {
                character_name,
                gender,
                weapon_level,
                steed_attire,
            }
        }
    }
}