#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//use std::env;
mod vm;
mod save;
mod read;
mod write;
mod ui;
mod db;

use std::path::PathBuf;

use eframe::{egui::{self, Id, LayerId, Order}, epaint::Color32};
use save::save::save::{Save, SaveType};
use ui::{events::events::events, general::general::general, inventory::inventory::inventory, menu::menu::{menu, Route}, none::none::none, regions::regions::regions, stats::stats::stats};
use vm::vm::vm::ViewModel;
 


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Save Editor", options, Box::new(|creation_context| {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        creation_context.egui_ctx.set_fonts(fonts);
        Box::new(App::new(creation_context))
    }))
}

pub struct App {
    save: Save,
    vm: ViewModel,
    picked_path: PathBuf,
    current_route: Route
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            save: Save::default(), 
            picked_path: Default::default(), 
            current_route: Route::None,
            vm: ViewModel::default()
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.75);

        // TOP PANEL
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if self.picked_path.exists() {
                let save_type = match self.save.save_type {
                    SaveType::Unknown => {
                        "Plattform: Unknown"
                    }
                    SaveType::PC => {
                        "Plattform: PC"
                    }
                    SaveType::SaveWizard => {
                        "Plattform: Save Wizard"
                    },
                };

                ui.columns(3,| uis| {
                    if self.vm.active {
                        egui::Frame::none().show(&mut uis[1], |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("Character: {}", self.vm.profile_summary[self.vm.index].character_name));
                                ui.label(format!("Steam Id: {}", self.vm.steam_id));
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

        // Character List Panel
        if !self.vm.steam_id.is_empty() {
            egui::SidePanel::left("characters").show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("left")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for i in 0..0xA {
                                if self.vm.profile_summary[i].active {
                                    let button = ui.add_sized([120., 40.], egui::Button::new(&self.vm.profile_summary[i].character_name));
                                    if button.clicked() {self.vm.index = i;}
                                    if self.vm.index == i {button.highlight();}
                                }
                            }
                        })
                    });
            });

            // Slot Section Panel
            if self.vm.active {
                egui::SidePanel::left("slot_sections_menu").show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .id_source("left")
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                menu(ui, self);
                            })
                        });
                });
            }

            // Main Content
            egui::CentralPanel::default().show(ctx, |ui| {
                match self.current_route {
                    Route::None => none(ui),
                    Route::General => general(ui, &mut self.vm),
                    Route::Stats => stats(ui, &mut self.vm),
                    Route::Inventory => inventory(ui, &mut self.vm),
                    Route::EventFlags => events(ui, &mut self.vm),
                    Route::Regions => regions(ui, &mut self.vm),
                }
            });
        }

        // No file loaded View
        else {

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
                    }
                    else {
                        ui.label("Drop a save file here or click 'Open' to browse");
                    }
                });

                // Check a file that has been dropped in the window
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        let file = i.raw.dropped_files[0].clone();
                        let path: std::path::PathBuf = file.path.expect("Error!");
                        
                        self.save = Save::from_path(&path).expect("Failed to read save");
                        self.vm = ViewModel::from_save(&self.save);
                        self.picked_path = path.clone();
                    }
                });
            });
        }
    }
}
