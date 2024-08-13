use eframe::egui::{self, Color32, Ui};
use er_save_lib::SaveApi;

use crate::vm::{
    importer::ImporterViewModel,
    notifications::{Notification, NotificationButtons, NotificationType, NOTIFICATIONS},
    vm::ViewModel,
};

/// Draws a window showing the character importer.
pub fn character_importer(
    ui: &mut Ui,
    app_save_api: &mut SaveApi,
    vm: &mut ViewModel,
    importer_vm: &mut ImporterViewModel,
    importer_open: &mut bool,
) {
    // Window (Importer)
    egui::Window::new("Importer")
        .open(importer_open)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            // Check if save file is valid
            if importer_vm.valid {
                // Row with two columns (FROM list, and TO list)
                ui.columns(2, |uis| {
                    // FROM character List
                    uis[0].vertical_centered_justified(|ui| {
                        ui.heading("From");
                        ui.separator();
                        for (i, from_character) in importer_vm
                            .from_list
                            .iter()
                            .filter(|c| c.active)
                            .enumerate()
                        {
                            if ui
                                .selectable_label(importer_vm.from_index == i, &from_character.name)
                                .clicked()
                            {
                                importer_vm.from_index = i
                            }
                        }
                    });
                    // TO character list
                    uis[1].vertical_centered_justified(|ui| {
                        ui.heading("To");
                        ui.separator();
                        for (i, to_character) in
                            importer_vm.to_list.iter().filter(|c| c.active).enumerate()
                        {
                            let character_name = if to_character.name.is_empty() {
                                format!("-- empty --")
                            } else {
                                to_character.name.to_string()
                            };
                            if ui
                                .selectable_label(importer_vm.to_index == i, character_name)
                                .clicked()
                            {
                                importer_vm.to_index = i
                            }
                        }
                    });
                });

                // Space
                ui.add_space(5.);

                // Import Button
                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add_sized(
                            [ui.available_width(), 40.],
                            egui::widgets::Button::new("Import"),
                        )
                        .clicked()
                    {
                        if let Err(err) = importer_vm.import_character(app_save_api, vm) {
                            NOTIFICATIONS.write().unwrap().push(Notification::new(
                                NotificationType::Error,
                                format!("Failed to import character: {}", err),
                                NotificationButtons::<String>::None,
                            ));
                        };
                    }
                });
            } else {
                ui.label(
                    egui::RichText::new("Save file has irregular data!").color(Color32::DARK_RED),
                );
            }
        });
}
