pub mod general {
    use eframe::egui::{self, Ui};
    use crate::vm::{general::general_view_model::{Gender, SteedAttire}, vm::vm::ViewModel};


    pub fn general(ui: &mut Ui, vm:&mut ViewModel) {
        ui.with_layout( egui::Layout::top_down_justified(egui::Align::Min),|ui|{
            let general_vm = &mut vm.slots[vm.index].general_vm;
            
            // Character Name
            ui.label("Character Name:");
            ui.add(egui::widgets::TextEdit::singleline(&mut general_vm.character_name).char_limit(0x10));
        
            ui.add_space(8.0);

            ui.horizontal(|ui|{
                if ui.radio(general_vm.gender == Gender::Male, "Male").clicked(){
                    general_vm.gender = Gender::Male;
                };
                if ui.radio(general_vm.gender == Gender::Female, "Female").clicked(){
                    general_vm.gender = Gender::Female;
                };
            });

            ui.add_space(16.0);

            // Torrent appearance (Spectral Steed Attire, added by patch 1.17)
            ui.label("Torrent Appearance:");
            for attire in SteedAttire::ALL {
                if ui.radio(general_vm.steed_attire == attire, attire.label()).clicked() {
                    general_vm.steed_attire = attire;
                }
            }
        });
    }
}