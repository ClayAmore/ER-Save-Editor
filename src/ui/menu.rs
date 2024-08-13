use crate::App;
use eframe::egui::{self, Context};

pub enum Route {
    None,
    General,
    Stats,
    Equipment,
    Inventory,
    EventFlags,
    Regions,
}

/// Draws a side panel showing a menu for navigating views for a selected character in a opened save.
pub fn menu(ctx: &Context, app: &mut App) {
    // Slot Section Panel
    egui::SidePanel::left("slot_sections_menu").show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Create the buttons
                    let general = ui.add_sized([120., 40.], egui::Button::new("General"));
                    let stats = ui.add_sized([120., 40.], egui::Button::new("Stats"));
                    let equipment = ui.add_sized([120., 40.], egui::Button::new("Equipment"));
                    let inventory = ui.add_sized([120., 40.], egui::Button::new("Inventory"));
                    let event_flags = ui.add_sized([120., 40.], egui::Button::new("Event Flags"));
                    let regions = ui.add_sized([120., 40.], egui::Button::new("Regions"));

                    // Listen for clicks
                    if general.clicked() {
                        app.current_route = Route::General;
                    }
                    if stats.clicked() {
                        app.current_route = Route::Stats;
                    }
                    if equipment.clicked() {
                        app.current_route = Route::Equipment;
                    }
                    if inventory.clicked() {
                        app.current_route = Route::Inventory
                    }
                    if event_flags.clicked() {
                        app.current_route = Route::EventFlags
                    }
                    if regions.clicked() {
                        app.current_route = Route::Regions
                    }

                    // Highlight active
                    match app.current_route {
                        Route::None => {}
                        Route::General => {
                            general.highlight();
                        }
                        Route::Stats => {
                            stats.highlight();
                        }
                        Route::Equipment => {
                            equipment.highlight();
                        }
                        Route::Inventory => {
                            inventory.highlight();
                        }
                        Route::EventFlags => {
                            event_flags.highlight();
                        }
                        Route::Regions => {
                            regions.highlight();
                        }
                    }
                })
            });
    });
}
