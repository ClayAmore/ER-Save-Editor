#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod vm;
mod save;
mod util;
mod read;
mod write;
mod ui;
mod db;

use std::{fs::File, io::Write, path::PathBuf};

use eframe::{egui::{self, text::LayoutJob, Align, FontSelection, Id, LayerId, Layout, Order, RichText, Rounding, Style}, epaint::Color32};
use rfd::FileDialog;
use save::save::save::{Save, SaveType};
use ui::{equipment::equipment::equipment, events::events::events, general::general::general, importer::import::character_importer, inventory::inventory::inventory::inventory, menu::menu::{menu, Route}, none::none::none, regions::regions::regions, stats::stats::stats};
use vm::{importer::general_view_model::ImporterViewModel, vm::vm::ViewModel};
use crate::write::write::Write as w; 
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "icon/"]
struct Asset;

const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 960.;

fn main() -> Result<(), eframe::Error> {
    // App Icon
    let mut app_icon = egui::IconData::default();
    
    let image = Asset::get("icon.png").expect("Failed to get image data").data;
    let icon = image::load_from_memory(&image).expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    app_icon.rgba = icon.into_raw();
    app_icon.width = icon_width;
    app_icon.height = icon_height;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_icon(app_icon),
        ..Default::default()
    };

    eframe::run_native("ER Save Editor 0.0.21", options, Box::new(|creation_context| {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);
        creation_context.egui_ctx.set_fonts(fonts);
        let mut visuals = creation_context.egui_ctx.style().visuals.clone();
        let rounding = 3.;
        visuals.window_rounding = Rounding::default().at_least(rounding);
        visuals.window_highlight_topmost = false;
        creation_context.egui_ctx.set_visuals(visuals);
        Box::new(App::new(creation_context))
    }))
}

pub struct App {
    save: Save,
    vm: ViewModel,
    picked_path: PathBuf,
    current_route: Route,
    importer_vm: ImporterViewModel,
    importer_open: bool,
    error: Option<String>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            save: Save::default(), 
            picked_path: Default::default(), 
            current_route: Route::None,
            vm: ViewModel::default(),
            importer_vm: Default::default(),
            importer_open: Default::default(),
            error: None
        }
    }

    // The parsers assert their way through the save layout, so a file this build
    // does not understand surfaces as a panic rather than an Err. Every load
    // goes through here so a bad file reports itself instead of killing the
    // window with no message.
    fn guarded<T>(load: impl FnOnce() -> Result<T, std::io::Error>) -> Result<T, String> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(load)) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(err)) => Err(format!("Could not read this file: {err}")),
            Err(_) => Err(
                "Could not read this file. It is either not an Elden Ring save, or it comes from a game patch this build cannot parse yet."
                    .to_string(),
            ),
        }
    }

    fn open(&mut self, path: PathBuf) {
        let loaded = Self::guarded(|| {
            let save = Save::from_path(&path)?;
            let vm = ViewModel::from_save(&save);
            Ok((save, vm))
        });

        match loaded {
            Ok((save, vm)) => {
                self.error = None;
                self.save = save;
                self.vm = vm;
                self.picked_path = path;
            }
            Err(err) => self.error = Some(err),
        }
    }

    fn save(&mut self, path: PathBuf) {
        self.vm.update_save(&mut self.save.save_type);
        let mut f = File::create(path).expect("");
        let bytes = self.save.write().expect("");
        let res = f.write_all(&bytes);

        match res {
            Ok(_) => {},
            Err(_) => todo!(),
        }
    }

    fn open_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
        .add_filter("SL2", &["sl2", "Regular Save File"])
        .add_filter("CO2", &["co2", "Seamless Co-op Save File"])
        .add_filter("TXT", &["txt", "Save Wizard Exported TXT File"])
        .add_filter("*", &["*", "All files"])
        .set_directory("/")
        .pick_file()
    } 

    fn save_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
        .add_filter("SL2", &["sl2", "Regular Save File"])
        .add_filter("CO2", &["co2", "Seamless Co-op Save File"])
        .add_filter("TXT", &["txt", "Save Wizard Exported TXT File"])
        .add_filter("*", &["*", "Any format"])
        .set_directory("/")
        .save_file()
    } 
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.75);
        // TOP PANEL
        egui::TopBottomPanel::top("toolbar").default_height(35.).show(ctx, |ui| {
            ui.columns(2, |uis|{
                uis[0].with_layout(Layout::left_to_right(Align::Center),| ui| {
                    if ui.button(egui::RichText::new(format!("{} open", egui_phosphor::regular::FOLDER_OPEN))).clicked() {
                        let files = Self::open_file_dialog();
                        match files {
                            Some(path) => self.open(path),
                            None => {},
                        }
                    }
                    if ui.button(egui::RichText::new(format!("{} save", egui_phosphor::regular::FLOPPY_DISK))).clicked() {
                        let files = Self::save_file_dialog();
                        match files {
                            Some(path) => self.save(path),
                            None => {},
                        }
                    }
                });
                
                uis[1].with_layout(Layout::right_to_left(egui::Align::Center),|ui| {
                    let import_button = egui::widgets::Button::new(egui::RichText::new(format!("{} Import Character", egui_phosphor::regular::DOWNLOAD_SIMPLE)));
                    if ui.add_enabled(!self.vm.steam_id.is_empty(), import_button).clicked() {
                        let files = Self::open_file_dialog();
                        match files {
                            Some(path) => {
                                let current = &self.vm;
                                let imported = Self::guarded(|| {
                                    let save = Save::from_path(&path)?;
                                    Ok(ImporterViewModel::new(save, current))
                                });
                                match imported {
                                    Ok(importer_vm) => {
                                        self.importer_vm = importer_vm;
                                        self.importer_open = true;
                                    }
                                    Err(err) => self.error = Some(err),
                                }
                            },
                            None => {},
                        }
                    }
                    character_importer(ui, &mut self.importer_open, &mut self.importer_vm, &mut self.save, &mut self.vm);
                });
            });


            if let Some(error) = &self.error {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(error).color(Color32::DARK_RED));
                });
            }
        });

        // TOP PANEL
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if self.picked_path.exists() {
                let save_type = match self.save.save_type {
                    SaveType::Unknown => {
                        "Platform: Unknown"
                    }
                    SaveType::PC(_) => {
                        "Platform: PC"
                    }
                    SaveType::PlayStation(_) => {
                        "Platform: Playstation"
                    },
                };

                ui.columns(2,| uis| {
                    if self.vm.active.is_some_and(|valid| valid) {
                        egui::Frame::none().show(&mut uis[1], |ui| {
                            let steam_id_text_edit = egui::widgets::TextEdit::singleline(&mut self.vm.steam_id)
                            .char_limit(17)
                            .desired_width(125.);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("Character: {}", self.vm.slots[self.vm.index].general_vm.character_name));
                                
                                match self.save.save_type {
                                    SaveType::Unknown => {},
                                    SaveType::PC(_) => {
                                        let steam_id_text_edit = ui.add(steam_id_text_edit).labelled_by(ui.label("Steam Id:").id);
                                        if steam_id_text_edit.hovered() {
                                            egui::popup::show_tooltip(ui.ctx(), steam_id_text_edit.id, |ui|{
                                                ui.label(egui::RichText::new("Important: This needs to match the id of the steam account that will use this save!").size(8.0).color(Color32::PLACEHOLDER));
                                            });
                                        }
                                    },
                                    SaveType::PlayStation(_) => {},
                                };
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
        if self.vm.active.is_some_and(|valid| valid) {
            egui::SidePanel::left("characters").show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("left")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for i in 0..0xA {
                                if self.vm.profile_summary[i].active {
                                    let button = ui.add_sized([120., 40.], egui::Button::new(&self.vm.slots[i].general_vm.character_name));
                                    if button.clicked() {self.vm.index = i;}
                                    if self.vm.index == i {button.highlight();}
                                }
                            }
                        })
                    });
            });

            // Slot Section Panel
            egui::SidePanel::left("slot_sections_menu").show(ctx, |ui| {
                egui::ScrollArea::vertical() .id_source("left") .show(ui, |ui| {
                    ui.vertical(|ui| {
                        menu(ui, self);
                    })
                });
            });

            // Main Content
            egui::CentralPanel::default().show(ctx, |ui| {
                match self.current_route {
                    Route::None => none(ui),
                    Route::General => general(ui, &mut self.vm),
                    Route::Stats => stats(ui, &mut self.vm),
                    Route::Equipment => equipment(ui, &mut self.vm),
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
                        let style = Style::default();
                        let mut layout_job = LayoutJob::default();
                        if self.vm.active.is_some_and(|valid| !valid) {
                            RichText::new("Save file has irregular data!\n\n")
                            .color(Color32::DARK_RED)
                            .append_to(
                                &mut layout_job,
                                &style,
                                FontSelection::Default,
                                Align::Center,
                            );
                        }
                        RichText::new("Drop a save file here or click 'Open' to browse")
                        .append_to(
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
                        let file = i.raw.dropped_files[0].clone();
                        let path: std::path::PathBuf = file.path.expect("Error!");
                        self.open(path);
                    }
                });
            });
        }
    }
}

#[cfg(test)]
mod save_round_trip {
    use super::*;
    use crate::util::regulation::Regulation;

    // Uses real saves, which are not part of the repo. Drop your own
    // ER0000.sl2 / ER0000.co2 into saves/ to run this.
    pub fn saves_on_disk() -> Vec<PathBuf> {
        let dir = match std::fs::read_dir("saves") {
            Ok(dir) => dir,
            Err(_) => return Vec::new(),
        };
        let mut found: Vec<PathBuf> = dir
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                matches!(
                    p.extension().and_then(|e| e.to_str()),
                    Some("sl2") | Some("co2")
                )
            })
            .collect();
        found.sort();
        found
    }

    #[test]
    fn reads_current_patch_save_and_writes_it_back_unchanged() {
        let saves = saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }
        for path in saves {
            println!("--- {} ---", path.display());
            round_trip(&path);
        }
    }

    fn round_trip(path: &PathBuf) {
        let original = std::fs::read(path).expect("read save file");
        let save = Save::from_path(path).expect("save failed to parse");

        // init_params swallows errors, so check the regulation separately.
        let params = Regulation::params_from_regulation(save.save_type.get_regulation())
            .expect("regulation failed to parse");
        assert!(!params.is_empty(), "no params came out of the regulation");

        let vm = ViewModel::from_save(&save);
        assert_eq!(vm.active, Some(true), "validator rejected the save");

        // Writing back an unedited save must reproduce the file exactly.
        let rewritten = save.write().expect("save failed to serialize");
        assert_eq!(rewritten.len(), original.len(), "size changed on round trip");
        assert!(rewritten == original, "round trip lost or altered data");
    }
}

#[cfg(test)]
mod stat_edit {
    use super::*;
    use crate::save::save::save::Save;

    #[test]
    fn edited_stat_survives_a_save_and_reload() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }
        for path in saves {
            println!("--- {} ---", path.display());
            edit_and_reload(&path);
        }
    }

    fn edit_and_reload(path: &PathBuf) {
        let original_len = std::fs::metadata(path).expect("stat save").len();
        let mut save = Save::from_path(path).expect("save failed to parse");
        let mut vm = ViewModel::from_save(&save);

        let before = vm.slots[0].stats_vm.clone();
        println!("before: vigor {} level {}", before.vigor, before.level);

        vm.slots[0].stats_vm.vigor = before.vigor + 1;
        vm.update_save(&mut save.save_type);

        let out = PathBuf::from("target/edited.sl2");

        std::fs::write(&out, save.write().expect("serialize")).expect("write edited save");
        assert_eq!(
            std::fs::metadata(&out).expect("stat edited").len(),
            original_len,
            "edited save changed size"
        );

        let reloaded = Save::from_path(&out).expect("edited save no longer parses");
        let vm2 = ViewModel::from_save(&reloaded);
        assert_eq!(vm2.active, Some(true), "validator rejected the edited save");

        let after = &vm2.slots[0].stats_vm;
        println!("after:  vigor {} level {}", after.vigor, after.level);
        assert_eq!(after.vigor, before.vigor + 1, "vigor did not persist");
        assert_eq!(after.level, before.level + 1, "level was not recalculated");

        // Only the edited character should have moved.
        assert_eq!(vm2.slots[1].stats_vm.vigor, vm.slots[1].stats_vm.vigor);
        assert_eq!(
            vm2.slots[1].general_vm.character_name,
            vm.slots[1].general_vm.character_name
        );
    }
}

#[cfg(test)]
mod bad_file {
    use super::*;

    // Mirrors what App::open does, so a file it cannot parse reports an error
    // instead of taking the process down with no message.
    fn try_load(path: &PathBuf) -> Result<(), ()> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let save = Save::from_path(path).map_err(|_| ())?;
            ViewModel::from_save(&save);
            Ok(())
        }))
        .unwrap_or(Err(()))
    }

    #[test]
    fn unparseable_files_are_caught_not_fatal() {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));

        let junk = PathBuf::from("target/junk.sl2");
        std::fs::write(&junk, vec![0u8; 1024]).expect("write junk");
        assert!(try_load(&junk).is_err(), "junk file should not load");

        // A real BND4 magic but nothing valid behind it.
        let mut truncated = b"BND4".to_vec();
        truncated.extend(vec![0u8; 4096]);
        let trunc = PathBuf::from("target/truncated.sl2");
        std::fs::write(&trunc, truncated).expect("write truncated");
        assert!(try_load(&trunc).is_err(), "truncated file should not load");

        std::panic::set_hook(prev);
    }
}

#[cfg(test)]
mod modded_saves {
    use super::*;
    use crate::util::regulation::Regulation;

    // A Seamless Co-op save (.co2) has the same layout but can carry items the
    // vanilla regulation does not list.
    #[test]
    fn co_op_saves_load_and_are_editable() {
        let path = PathBuf::from("saves/ER0000.co2");
        if !path.exists() {
            eprintln!("skipping: no saves/ER0000.co2");
            return;
        }

        let save = Save::from_path(&path).expect("co2 failed to parse");
        let vm = ViewModel::from_save(&save);
        assert_eq!(vm.active, Some(true), "co2 was rejected as irregular");

        let active: Vec<usize> = save.save_type.active_slots().iter().enumerate()
            .filter(|(_, a)| **a).map(|(i, _)| i).collect();
        assert!(!active.is_empty(), "no characters found in the co2");
        for i in &active {
            println!("co2 slot {i}: '{}'", vm.slots[*i].general_vm.character_name);
        }
    }

    // Every save brings its own regulation, so the derived maps must follow it.
    #[test]
    fn param_maps_follow_the_loaded_save() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.len() < 2 {
            eprintln!("skipping: need two saves");
            return;
        }
        let mut counts = Vec::new();
        for path in &saves {
            let save = Save::from_path(path).expect("parse");
            Regulation::init_params(&save);
            counts.push(Regulation::equip_goods_param_map().len());
        }
        let first = Save::from_path(&saves[0]).expect("parse");
        Regulation::init_params(&first);
        assert_eq!(
            Regulation::equip_goods_param_map().len(),
            counts[0],
            "param maps did not follow the reloaded save"
        );
        assert_ne!(counts[0], counts[1], "expected the saves to differ");
    }
}
