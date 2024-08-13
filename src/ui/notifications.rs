use crate::{
    vm::notifications::{NotificationType, NOTIFICATIONS},
    App,
};
use eframe::egui::{self, Layout};
use egui::{epaint::Shadow, Color32, Context, Margin, Rounding, Stroke};

/// Draws a top panel that functions as a notification bar. The notification bar is used to display
/// important messages, alerts, or notifications to the user.
pub fn notifications(ctx: &Context, app: &mut App) {
    let vm = &mut app.vm;

    // If no pending notifications then exit early
    if NOTIFICATIONS.read().unwrap().is_empty() {
        return;
    }

    let notifications = NOTIFICATIONS.read().unwrap().clone();

    // Draw the top panel containing the notification
    for (i, notification) in notifications.iter().enumerate() {
        // Style the notification and give it the proper color based on it's type
        let frame_style = egui::Frame {
            outer_margin: Margin::ZERO,
            inner_margin: Margin {
                left: 10.,
                right: 10.,
                top: 10.,
                bottom: 10.,
            },
            rounding: Rounding::ZERO,
            shadow: Shadow::NONE,
            fill: match notification.notification_type {
                NotificationType::Info => Color32::LIGHT_BLUE,
                NotificationType::Success => Color32::LIGHT_GREEN,
                NotificationType::Warning => Color32::YELLOW,
                NotificationType::Error => Color32::LIGHT_RED,
            },
            stroke: Stroke::NONE,
        };

        // Draw the top panel containing the notification
        egui::TopBottomPanel::top(format!("notifications_{}", i))
            .frame(frame_style)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                        ui.add_space(4.);
                        ui.label(&notification.text);

                        // Check if notification has any buttons
                        if notification.buttons.len() > 0 {
                            // Draw notification buttons
                            ui.horizontal(|ui| {
                                for (button_text, callback) in &notification.buttons {
                                    if ui.button(button_text).clicked() {
                                        callback(ctx, vm);
                                    }
                                }
                            });
                        }
                    });

                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{}", egui_phosphor::regular::X)).size(18.),
                        )
                        .fill(Color32::TRANSPARENT)
                        .stroke(Stroke::NONE);

                        if ui.add(button).clicked() {
                            NOTIFICATIONS.write().unwrap().remove(i);
                        }
                    })
                })
            });
    }
}
