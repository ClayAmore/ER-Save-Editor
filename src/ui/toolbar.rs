use eframe::egui::{self, Align, Context, Layout};
use er_save_lib::SaveApi;

use crate::{
    vm::{
        importer::ImporterViewModel,
        notifications::{Notification, NotificationButtons, NotificationType, NOTIFICATIONS},
    },
    App,
};

use super::importer::character_importer;

/// Draws a top panel with "Open", "Save", and "Import" buttons.
pub(crate) fn toolbar_top_panel(ctx: &Context, app: &mut App) {
    egui::TopBottomPanel::top("toolbar")
        .default_height(35.)
        .show(ctx, |ui| {
            // Left Side: Open Save Buttons
            ui.columns(2, |uis| {
                uis[0].with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui
                        .button(egui::RichText::new(format!(
                            "{} open",
                            egui_phosphor::regular::FOLDER_OPEN
                        )))
                        .clicked()
                    {
                        let result = App::open_file_dialog();
                        if let Some(path) = result {
                            // Try to open file. Notify if there's error.
                            if let Err(err) = app.open(&path) {
                                NOTIFICATIONS.write().unwrap().push(Notification::new(
                                    NotificationType::Error,
                                    format!("Failed to open file: {}", err),
                                    NotificationButtons::<String>::None,
                                ));
                            }
                        }
                    }
                    if ui
                        .button(egui::RichText::new(format!(
                            "{} save",
                            egui_phosphor::regular::FLOPPY_DISK
                        )))
                        .clicked()
                    {
                        let result = App::save_file_dialog();
                        if let Some(path) = result {
                            // Try to save file. Notify if there's error.
                            match app.save(&path) {
                                Ok(()) => {
                                    NOTIFICATIONS.write().unwrap().push(Notification::new(
                                        NotificationType::Success,
                                        format!("Your file has been saved! A 'backups' folder has been created in the same directory with a copy of the original file."),
                                        NotificationButtons::<String>::None,
                                    ));
                                },
                                Err(err) => {
                                    NOTIFICATIONS.write().unwrap().push(Notification::new(
                                        NotificationType::Error,
                                        format!("Failed to save file: {}", err),
                                        NotificationButtons::<String>::None,
                                    ));
                                },
                            }
                        }
                    }
                });

                // Right Side: Import Button
                uis[1].with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let import_button = egui::widgets::Button::new(egui::RichText::new(format!(
                        "{} Import Character",
                        egui_phosphor::regular::DOWNLOAD_SIMPLE
                    )));
                    if ui
                        .add_enabled(app.save_api.is_some(), import_button)
                        .clicked()
                    {
                        let path = App::open_file_dialog();

                        if let Some(path) = path {
                            let save_api = SaveApi::from_path(path);
                            match save_api {
                                Ok(save_api) => {
                                    app.importer_vm = ImporterViewModel::new(save_api, &app.vm);
                                    app.importer_open = true;
                                }
                                Err(err) => {
                                    NOTIFICATIONS.write().unwrap().push(Notification::new(
                                        NotificationType::Error,
                                        format!("Failed to open save file: {}", err),
                                        NotificationButtons::<String>::None,
                                    ));
                                }
                            }
                        }
                    }
                    if app.importer_vm.save_api.is_some() {
                        character_importer(
                            ui,
                            app.save_api.as_mut().unwrap(),
                            &mut app.vm,
                            &mut app.importer_vm,
                            &mut app.importer_open,
                        );
                    }
                });
            });
        });
}
