use eframe::egui::{self, Color32, Context};
use er_save_lib::SaveType;
use crate::App;

/// Draws a top panel with showing information about the opened save information like platform, 
/// active character name and steam_id.
pub(crate) fn information_top_panel(ctx: &Context, app: &mut App) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        if app.picked_path.exists() {
            let save_type = match app.save_api.as_ref().unwrap().platform() {
                SaveType::PC => "Platform: PC",
                SaveType::Playstation => "Platform: Playstation",
            };

            ui.columns(2,| uis| {
                if app.save_api.is_some() {
                    egui::Frame::none().show(&mut uis[1], |ui| {
                        let steam_id_text_edit = egui::widgets::TextEdit::singleline(&mut app.vm.steam_id)
                        .char_limit(17)
                        .desired_width(125.);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Character: {}", app.save_api.as_ref().unwrap().character_name(app.vm.index)));
                            
                            if app.save_api.as_ref().unwrap().platform() == SaveType::PC {
                                let steam_id_text_edit = ui.add(steam_id_text_edit).labelled_by(ui.label("Steam Id:").id);
                                if steam_id_text_edit.hovered() {
                                    egui::popup::show_tooltip(ui.ctx(), steam_id_text_edit.id, |ui|{
                                        ui.label(egui::RichText::new("Important: This needs to match the id of the steam account that will use this save!").size(8.0).color(Color32::PLACEHOLDER));
                                    });
                                }
                            }
                        });
                    });
                }
                egui::Frame::none().show(&mut uis[0], |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(format!("{}",save_type));
                    });
                });
            });
        }
    });
}
