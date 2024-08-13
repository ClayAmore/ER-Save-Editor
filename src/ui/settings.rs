use crate::App;
use eframe::egui::{self, Color32, Context};

/// Draws a bottom panel with general application settings like activating DLC items,
/// Zoom out or Zoom in.
pub(crate) fn settings_bottom_panel(ctx: &Context, app: &mut App) {
    egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        egui::Frame::none().inner_margin(8.).show(ui, |ui| {
            ui.horizontal(|ui| {
                // DLC Checkbox
                let dlc_checkbox = ui.checkbox(&mut app.vm.dlc, "DLC");
                if dlc_checkbox.hovered() {
                    egui::popup::show_tooltip(ui.ctx(), dlc_checkbox.id, |ui|{
                        ui.label(egui::RichText::new("Important: Attempting to access specific DLC items without owning the DLC can result in a ban!").size(8.0).color(Color32::PLACEHOLDER));
                    });
                }

                // Seperator
                ui.separator();

                // Zoom in Button
                ui.label("Zoom");
                let zoom_in_button = ui.button(egui::RichText::new(format!(
                    "{}",
                    egui_phosphor::regular::MAGNIFYING_GLASS_PLUS
                )));
                if zoom_in_button.hovered() {
                    egui::popup::show_tooltip(ui.ctx(), zoom_in_button.id, |ui|{
                        ui.label(egui::RichText::new("Press 'CTRL' + '+' to zoom in").size(8.0).color(Color32::PLACEHOLDER));
                    });
                }
                if zoom_in_button.clicked() {
                    let zoom_factor = ctx.zoom_factor();
                    ctx.set_zoom_factor(zoom_factor + 0.1);
                }

                // Zoom out Button
                let zoom_out_button = ui.button(egui::RichText::new(format!(
                    "{}",
                    egui_phosphor::regular::MAGNIFYING_GLASS_MINUS
                )));
                if zoom_out_button.hovered() {
                    egui::popup::show_tooltip(ui.ctx(), zoom_in_button.id, |ui|{
                        ui.label(egui::RichText::new("Press 'CTRL' + '-' to zoom out").size(8.0).color(Color32::PLACEHOLDER));
                    });
                }
                if zoom_out_button.clicked() {
                    let zoom_factor = ctx.zoom_factor();
                    ctx.set_zoom_factor(zoom_factor - 0.1);
                }
            });
        })
    });
}
