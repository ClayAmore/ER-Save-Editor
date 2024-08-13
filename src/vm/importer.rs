use er_save_lib::{SaveApi, SaveApiError};

use super::{character::CharacterViewModel, vm::ViewModel};

#[derive(Default, Clone)]
pub struct Profile {
    pub active: bool,
    pub name: String,
}
impl Profile {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Profile {
            active: true,
            name: name.into(),
        }
    }
}

#[derive(Default)]
pub struct ImporterViewModel {
    pub valid: bool,
    pub from_index: usize,
    pub to_index: usize,
    pub from_list: Vec<Profile>,
    pub to_list: Vec<Profile>,
    pub save_api: Option<SaveApi>,
}

impl ImporterViewModel {
    pub fn new(save_api: SaveApi, vm: &ViewModel) -> Self {
        let mut from_profiles = Vec::new();
        for (index, active) in save_api.active_characters().iter().enumerate() {
            if *active {
                // Character Name
                let character_name = save_api.character_name(index);
                from_profiles.push(Profile::new(&character_name));
            }
        }

        let mut to_profiles = Vec::new();
        for character in vm.characters.iter() {
            to_profiles.push(Profile::new(&character.general_vm.character_name));
        }

        ImporterViewModel {
            valid: true,
            from_list: from_profiles,
            to_list: to_profiles,
            save_api: Some(save_api),
            from_index: 0,
            to_index: 0,
        }
    }

    pub fn import_character(
        &mut self,
        app_save_api: &mut SaveApi,
        vm: &mut ViewModel,
    ) -> Result<(), SaveApiError> {
        // Try to import character. Notify if there's error.
        app_save_api.import_character(
            self.to_index,
            self.save_api.as_ref().unwrap(),
            self.from_index,
        )?;

        // Refresh view model
        vm.characters[self.to_index] = CharacterViewModel::from_save(app_save_api, self.to_index)?;

        self.to_list[self.to_index] =
            Profile::new(&vm.characters[self.to_index].general_vm.character_name);

        Ok(())
    }
}
