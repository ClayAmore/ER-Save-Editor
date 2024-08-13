use crate::vm::{general::general_view_model::Gender, vm::ViewModel};
use eframe::egui::{self, Ui};

/// Draws the 'General' route view.
pub fn general(ui: &mut Ui, vm: &mut ViewModel) {
    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        let general_vm = &mut vm.characters[vm.index].general_vm;

        // Character Name
        ui.label("Character Name:");
        ui.add(egui::widgets::TextEdit::singleline(&mut general_vm.character_name).char_limit(16));

        ui.add_space(8.0);

        // Gender
        ui.horizontal(|ui| {
            if ui
                .radio(general_vm.gender == Gender::Male, "Male")
                .clicked()
            {
                general_vm.gender = Gender::Male;
            };
            if ui
                .radio(general_vm.gender == Gender::Female, "Female")
                .clicked()
            {
                general_vm.gender = Gender::Female;
            };
        });
    });
}
