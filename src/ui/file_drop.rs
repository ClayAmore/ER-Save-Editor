use crate::{
    vm::notifications::{Notification, NotificationButtons, NotificationType, NOTIFICATIONS},
    App,
};
use eframe::egui::{self, text::LayoutJob, FontSelection, Id, LayerId, Order, RichText, Style};
use egui::{Align, Color32, Context};

/// Draws a Central Panel used for listening for dragged files and attempting
/// to open them when dropped.
pub fn file_drop_main_panel(ctx: &Context, app: &mut App) {
    // Listen for dragged files and update path
    egui::CentralPanel::default().show(ctx, |ui| {
        // Check if hovering a file
        let path = ctx.input(|i| {
            if !i.raw.hovered_files.is_empty() {
                let file = i.raw.hovered_files[0].clone();
                let path: std::path::PathBuf = file.path.expect("Error!");
                return path.into_os_string().into_string().expect("");
            }
            "".to_string()
        });

        // Display indicator of hovering file
        ui.centered_and_justified(|ui| {
            if !path.is_empty() {
                let painter =
                    ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

                let screen_rect = ctx.screen_rect();
                painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(96));
                ui.label(egui::RichText::new(path));
            } else {
                let style = Style::default();
                let mut layout_job = LayoutJob::default();
                RichText::new("Drop a save file here or click 'Open' to browse").append_to(
                    &mut layout_job,
                    &style,
                    FontSelection::Default,
                    Align::Center,
                );
                ui.label(layout_job);
            }
        });

        // Check a file that has been dropped in the window
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let file = &i.raw.dropped_files[0];
                if let Some(path) = &file.path {
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
        });
    });
}
