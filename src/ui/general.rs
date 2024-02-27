pub mod general {
    use eframe::{egui::{self, Ui}, epaint::Color32};
    use crate::vm::vm::vm::ViewModel;


    pub fn general(ui: &mut Ui, vm:&mut ViewModel) {
        ui.with_layout( egui::Layout::top_down_justified(egui::Align::Min),|ui|{
            
            // Steam ID
            ui.label("Steam Id:");
            ui.text_edit_singleline(&mut vm.slots[vm.index].general_vm.steam_id);
            ui.label(egui::RichText::new("Important: This needs to match the id of the steam account that will use this save!").size(8.0).color(Color32::PLACEHOLDER));
            
            ui.add_space(16.0);

            // Character Name
            ui.label("Character Name:");
            ui.add(egui::widgets::TextEdit::singleline(&mut vm.slots[vm.index].general_vm.character_name).char_limit(0x10));
        });
    }
}