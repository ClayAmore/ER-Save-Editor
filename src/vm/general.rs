pub mod general_view_model {
    use crate::save::save_slot::SaveSlot;


    #[derive(Clone)]
    pub struct GeneralViewModel  {
        pub steam_id: String,
        pub character_name: String,
    }

    impl Default for GeneralViewModel {
        fn default() -> Self {
            Self { steam_id: Default::default(), character_name: Default::default() }
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
             
            Self {
                steam_id,
                character_name
            }
        }
    }
}