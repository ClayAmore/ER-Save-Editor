use crate::App;
use eframe::egui;
use egui::Context;

/// Draws a side panel showing a menu for navigating an opened save characters.
pub fn character_list_side_panel(ctx: &Context, app: &mut App) {
    egui::SidePanel::left("characters").show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (i, state) in app
                        .save_api
                        .as_ref()
                        .unwrap()
                        .active_characters()
                        .iter()
                        .enumerate()
                    {
                        if *state {
                            let button = ui.add_sized(
                                [120., 40.],
                                egui::Button::new(
                                    &app.save_api.as_ref().unwrap().character_name(i),
                                ),
                            );
                            if button.clicked() {
                                app.vm.index = i;
                            }
                            if app.vm.index == i {
                                button.highlight();
                            }
                        }
                    }
                })
            });
    });
}
