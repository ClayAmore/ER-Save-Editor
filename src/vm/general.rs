pub mod general_view_model {
    use crate::save::common::save_slot::SaveSlot;

    #[derive(Default, Clone)]
    pub struct MapID {
        area_id: u8,
        block_id: u8,
        region_id: u8,
        index_id: u8,
    }
    impl ToString for MapID {
        fn to_string(&self) -> String {
            format!("{:02}{:02}{:02}{:02}", self.area_id, self.block_id, self.region_id, self.index_id)
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
        pub steam_id: String,
        pub character_name: String,
        pub gender: Gender,
        pub map_id: String,
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
             
            let map_id = MapID{ 
                area_id: slot.map_id[3],  
                block_id: slot.map_id[2],  
                region_id: slot.map_id[1],  
                index_id: slot.map_id[0],
            }.to_string();
            

            Self {
                steam_id,
                character_name,
                gender,
                map_id,
            }
        }
    }
}