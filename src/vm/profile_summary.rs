pub mod slot_view_model {
    use crate::save::{user_data_10::ProfileSummary};


    #[derive(Clone)]
    pub struct ProfileSummaryViewModel {
        pub active: bool,
        pub character_name: String,
    }

    impl Default for ProfileSummaryViewModel {
        fn default() -> Self {
            Self { 
                active: Default::default(),
                character_name: Default::default()
            }
        }
    }
    
    impl ProfileSummaryViewModel {
        pub fn from_save(profile_summary: &ProfileSummary) -> Self {
            let active = true;
            
            // Character Name
            let character_name_vec = profile_summary.character_name.to_vec();
            let character_name = String::from_utf8(character_name_vec).expect("");
             
            Self {
                active,
                character_name
            }
        }
    }
}