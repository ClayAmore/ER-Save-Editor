pub mod general_view_model {
    use crate::save::common::save_slot::SaveSlot;

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
    pub struct GeneralViewModel  {
        pub steam_id: String,
        pub character_name: String,
        pub gender: Gender,
    }

    impl Default for GeneralViewModel {
        fn default() -> Self {
            Self { 
                steam_id: Default::default(), 
                character_name: Default::default(),
                gender: Gender::Uknown, 
            }
        }
    }

    impl GeneralViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {

            // Steam Id
            let steam_id = slot.steam_id.to_string();

            // Character Name
            let character_name = slot.player_game_data.character_name;
            let mut character_name_trimmed: [u16; 0x10] = [0;0x10];
            for (i, char) in character_name.iter().enumerate() {
                if *char == 0 { break; }
                character_name_trimmed[i] = *char;
            }
            let character_name = String::from_utf16(&character_name_trimmed).expect("");

            let gender = Gender::try_from(slot.player_game_data.gender).expect("");
             
            Self {
                steam_id,
                character_name,
                gender
            }
        }
    }
}