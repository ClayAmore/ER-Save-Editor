pub mod import {
    use eframe::egui::{self, Color32, Ui};
    use crate::{save::save::save::Save, vm::{importer::general_view_model::ImporterViewModel, vm::vm::ViewModel}};

    pub fn character_importer(ui: &mut Ui, open: &mut bool, importer_vm: &mut ImporterViewModel, to_save:&mut Save, vm: &mut ViewModel) {
        egui::Window::new("Importer")
        .open(open)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            if importer_vm.valid {
                ui.columns(2, |uis|{
                    uis[0].vertical_centered_justified(|ui|{
                        ui.heading("From");
                        ui.separator();
                        for (i, from_character) in importer_vm.from_list.iter().filter(|c|c.active).enumerate() {
                            if ui.selectable_label(importer_vm.selected_from_index == i, &from_character.name).clicked() {
                                importer_vm.selected_from_index = i
                            }
                        }
                    });
                    uis[1].vertical_centered_justified(|ui|{
                        ui.heading("To");
                        ui.separator();
                        for (i, to_character) in importer_vm.to_list.iter().filter(|c|c.active).enumerate() {
                            if ui.selectable_label(importer_vm.selected_to_index == i, &to_character.name).clicked() {
                                importer_vm.selected_to_index = i
                            }
                        }
                    });
                });
                ui.add_space(5.);
                ui.vertical_centered_justified(|ui|{
                    if ui.add_sized([ui.available_width(), 40.], egui::widgets::Button::new("Import")).clicked() {
                        importer_vm.import_character(to_save, vm);
                    }
                });
            }
            else {
                ui.label(egui::RichText::new("Save file has irregular data!").color(Color32::DARK_RED));
            }
        });
    }
}