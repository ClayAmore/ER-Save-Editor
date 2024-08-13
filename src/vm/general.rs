pub mod general_view_model {
    use er_save_lib::SaveApi;

    #[derive(Default, Clone)]
    pub struct MapID {
        area_id: u8,
        block_id: u8,
        region_id: u8,
        index_id: u8,
    }
    impl ToString for MapID {
        fn to_string(&self) -> String {
            format!(
                "{:02}{:02}{:02}{:02}",
                self.area_id, self.block_id, self.region_id, self.index_id
            )
        }
    }

    #[derive(Default, Clone, PartialEq, Eq, Copy)]
    pub enum Gender {
        Female,
        Male,
        #[default]
        Uknown,
    }

    impl From<u8> for Gender {
        fn from(value: u8) -> Self {
            match value {
                x if x == Gender::Male as u8 => Gender::Male,
                x if x == Gender::Female as u8 => Gender::Female,
                _ => Gender::Uknown,
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct GeneralViewModel {
        pub character_name: String,
        pub gender: Gender,
    }

    impl GeneralViewModel {
        pub fn from_save(index: usize, save_api: &SaveApi) -> Self {
            // Character Name
            let character_name = save_api.character_name(index);

            // Gender
            let gender = Gender::from(save_api.gender(index));

            Self {
                character_name,
                gender,
            }
        }
    }
}
