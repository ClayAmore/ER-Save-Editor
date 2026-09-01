#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod vm;
mod save;
mod util;
mod read;
mod write;
mod ui;
mod db;
mod media;

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

    eframe::run_native(&format!("ER Save Editor {}", env!("CARGO_PKG_VERSION")), options, Box::new(|creation_context| {
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
    textures: media::textures::ItemTextures,
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
            error: None,
            textures: media::textures::ItemTextures::new(),
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
        self.textures.poll(ctx);
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
                                        self.error = None;
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
                    Route::Equipment => equipment(ui, &mut self.vm, &mut self.textures),
                    Route::Inventory => inventory(ui, &mut self.vm, &mut self.textures),
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

    // A save whose regulation cannot be read must never be loaded using the
    // params of the save before it. Failing loudly is the acceptable outcome;
    // quietly answering with the previous save's data is not.
    #[test]
    fn a_broken_regulation_never_reuses_the_previous_saves_params() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }

        let good = Save::from_path(&saves[0]).expect("parse");
        Regulation::init_params(&good);
        let expected = Regulation::equip_goods_param_map().len();
        assert!(expected > 0);

        let mut broken = Save::from_path(&saves[0]).expect("parse");
        match &mut broken.save_type {
            save::save::save::SaveType::PC(pc) => {
                pc.user_data_11.user_data_11.regulation[0x40..0x80].fill(0xAB);
            }
            _ => {
                eprintln!("skipping: not a PC save");
                return;
            }
        }

        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Regulation::init_params(&broken);
            Regulation::equip_goods_param_map().len()
        }));
        std::panic::set_hook(prev_hook);

        match outcome {
            // Refused the broken regulation outright, which is what we want.
            Err(_) => println!("broken regulation rejected"),
            Ok(len) => assert_ne!(
                len, expected,
                "a broken regulation silently answered with the previous save's params"
            ),
        }

        // Leave the globals holding a save that actually parses.
        Regulation::init_params(&good);
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
        if counts[0] == counts[1] {
            eprintln!("skipping: the saves on disk share a regulation, nothing to tell apart");
            return;
        }

        let first = Save::from_path(&saves[0]).expect("parse");
        Regulation::init_params(&first);
        assert_eq!(
            Regulation::equip_goods_param_map().len(),
            counts[0],
            "param maps did not follow the reloaded save"
        );
    }
}


#[cfg(test)]
mod dlc_names {
    use super::*;
    use crate::util::regulation::Regulation;

    // Shadow of the Erdtree content must not show up as [UNKOWN_<id>].
    #[test]
    fn dlc_items_are_named() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }
        let save = Save::from_path(&saves[0]).expect("parse");
        Regulation::init_params(&save);

        // A few Shadow of the Erdtree rows that used to render as [UNKOWN_].
        let armor = Regulation::equip_protectors_param_map();
        assert_eq!(armor.get(&5200000).map(|r| r.name.as_str()), Some("Death Knight Helm"));
        assert_eq!(armor.get(&5000000).map(|r| r.name.as_str()), Some("Oathseeker Knight Helm"));

        // Ashes of war are fully covered now; the others keep only rows the
        // game itself has no text for.
        let ashes = Regulation::equip_gem_param_map();
        assert_eq!(ashes.values().filter(|r| r.name.starts_with("[UNKOWN_")).count(), 0);

        let unnamed = |n: usize, total: usize| (n * 100) / total;
        assert!(unnamed(armor.values().filter(|r| r.name.starts_with("[UNKOWN_")).count(), armor.len()) < 10);

        let goods = Regulation::equip_goods_param_map();
        assert!(unnamed(goods.values().filter(|r| r.name.starts_with("[UNKOWN_")).count(), goods.len()) < 10);
    }

    // What the player actually sees: nothing they are carrying should be unnamed.
    #[test]
    fn nothing_in_the_inventory_is_unnamed() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }
        let save = Save::from_path(&saves[0]).expect("parse");
        let vm = ViewModel::from_save(&save);

        for (i, active) in save.save_type.active_slots().iter().enumerate() {
            if !*active { continue; }
            let mut unnamed: Vec<u32> = Vec::new();
            let mut total = 0usize;
            for storage in vm.slots[i].inventory_vm.storage.iter() {
                for list in [&storage.common_items, &storage.key_items] {
                    for item in list.iter() {
                        total += 1;
                        if item.item_name.starts_with("[UNKOWN_") {
                            unnamed.push(item.item_id);
                        }
                    }
                }
            }
            unnamed.sort();
            unnamed.dedup();
            println!("slot {i} ({}): {total} items, {} unnamed {:?}",
                vm.slots[i].general_vm.character_name, unnamed.len(),
                &unnamed[..unnamed.len().min(10)]);
        }
    }
}

#[cfg(test)]
mod new_classes {
    use super::*;

    // Patch 1.17 added two starting classes (CharaInitParam 3010 and 3011).
    // A character made with one of them must still load.
    #[test]
    fn characters_using_a_patch_class_still_load() {
        let saves = crate::save_round_trip::saves_on_disk();
        if saves.is_empty() {
            eprintln!("skipping: no saves in saves/");
            return;
        }

        for arche_type in [10u8, 11] {
            let mut save = Save::from_path(&saves[0]).expect("parse");
            match &mut save.save_type {
                save::save::save::SaveType::PC(pc) => {
                    pc.save_slots[0].save_slot.player_game_data.arche_type = arche_type;
                }
                _ => { eprintln!("skipping: not a PC save"); return; }
            }

            let prev = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let loaded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                ViewModel::from_save(&save)
            }));
            std::panic::set_hook(prev);

            assert!(loaded.is_ok(), "arche_type {arche_type} panicked while loading");
            let vm = loaded.unwrap();
            println!("arche_type {arche_type} -> class '{}'",
                vm.slots[0].stats_vm.arche_type.to_string());
        }
    }
}


#[cfg(test)]
mod steed_attire {
    use super::*;
    use crate::vm::general::general_view_model::{SteedAttire, STEED_ATTIRE_FLAG_BYTE};

    // Two saves the game itself produced, same character, differing only in the
    // attire applied to Torrent.
    const SILVER: &str = "saves/attire_silver.sl2";
    const FUNEREAL: &str = "saves/attire_funereal.sl2";

    fn flag_byte(save: &Save, slot: usize) -> u8 {
        save.save_type.get_slot(slot).event_flags.flags[STEED_ATTIRE_FLAG_BYTE]
    }

    #[test]
    fn reads_the_applied_attire() {
        let (silver, funereal) = (PathBuf::from(SILVER), PathBuf::from(FUNEREAL));
        if !silver.exists() || !funereal.exists() {
            eprintln!("skipping: attire reference saves not in saves/");
            return;
        }

        let a = Save::from_path(&silver).expect("parse");
        let b = Save::from_path(&funereal).expect("parse");
        let (va, vb) = (ViewModel::from_save(&a), ViewModel::from_save(&b));

        assert_eq!(va.slots[0].general_vm.steed_attire, SteedAttire::SilverOfCaria);
        assert_eq!(vb.slots[0].general_vm.steed_attire, SteedAttire::FunerealNight);

        // Characters that never touched the system read as the plain Torrent.
        assert_eq!(vb.slots[1].general_vm.steed_attire, SteedAttire::None);
    }

    #[test]
    fn writing_an_attire_matches_what_the_game_writes() {
        let (silver, funereal) = (PathBuf::from(SILVER), PathBuf::from(FUNEREAL));
        if !silver.exists() || !funereal.exists() {
            eprintln!("skipping: attire reference saves not in saves/");
            return;
        }

        let expected = flag_byte(&Save::from_path(&funereal).expect("parse"), 0);

        // Start from the Silver of Caria save and switch it over.
        let mut save = Save::from_path(&silver).expect("parse");
        let mut vm = ViewModel::from_save(&save);
        vm.slots[0].general_vm.steed_attire = SteedAttire::FunerealNight;
        vm.update_save(&mut save.save_type);

        assert_eq!(flag_byte(&save, 0), expected,
            "the editor did not reproduce the byte the game writes");
        println!("editor wrote {:#04x}, game wrote {:#04x}", flag_byte(&save, 0), expected);

        // And the inferred third option lands on its own bit without touching
        // anything else in that byte.
        let before = flag_byte(&save, 0);
        vm.slots[0].general_vm.steed_attire = SteedAttire::TreeSentinel;
        vm.update_save(&mut save.save_type);
        let after = flag_byte(&save, 0);
        println!("tree sentinel: {before:#04x} -> {after:#04x}");
        assert_eq!(after & 0b111, 0b100);
        assert_eq!(after & !0b111, before & !0b111, "unrelated bits changed");

        // Selecting the plain Torrent clears all three.
        vm.slots[0].general_vm.steed_attire = SteedAttire::None;
        vm.update_save(&mut save.save_type);
        assert_eq!(flag_byte(&save, 0) & 0b111, 0);
    }

    #[test]
    fn leaving_the_attire_alone_rewrites_the_save_unchanged() {
        let path = PathBuf::from(FUNEREAL);
        if !path.exists() {
            eprintln!("skipping: attire reference saves not in saves/");
            return;
        }
        let original = std::fs::read(&path).expect("read");
        let mut save = Save::from_path(&path).expect("parse");
        let vm = ViewModel::from_save(&save);
        vm.update_save(&mut save.save_type);
        assert!(save.write().expect("write") == original, "round trip changed the save");
    }
}
