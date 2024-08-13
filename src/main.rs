#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod api;
mod db;
mod ui;
mod updater;
mod vm;

use eframe::egui::{self, Rounding};
use er_save_lib::{SaveApi, SaveApiError};
use rfd::FileDialog;
use rust_embed::RustEmbed;
use std::{fs::create_dir_all, path::PathBuf};
use ui::{
    character_list::character_list_side_panel,
    events::events,
    file_drop::file_drop_main_panel,
    general::general,
    information::information_top_panel,
    inventory::inventory::inventory,
    menu::{menu, Route},
    none::none,
    notifications::notifications,
    regions::regions,
    settings::settings_bottom_panel,
    stats::stats,
    toolbar::toolbar_top_panel,
};
use updater::updater::Updater;

use vm::{
    importer::ImporterViewModel,
    notifications::{Notification, NotificationButtons, NotificationType, NOTIFICATIONS},
    vm::ViewModel,
};

#[derive(RustEmbed)]
#[folder = "icon/"]
struct Asset;

// Starter values for window
const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 960.;
const DEFAULT_ZOOM_FACTOR: f32 = 1.5;

fn main() -> Result<(), eframe::Error> {
    // Check for updates
    let result = Updater::get_new_version();

    // Push an update notification if there's a new version
    if let Some(new_version) = result {
        NOTIFICATIONS.write().unwrap().push(Notification::new(
            NotificationType::Warning,
            format!(
                "This version is outdated. A new version is available for downlaod!\n\nChangelog\n--------------------\n{}\n",
                new_version.body
            ),
            NotificationButtons::<String>::None,
        ));
    }

    // App Icon
    let mut app_icon = egui::IconData::default();

    // Add Icon to app
    let image = Asset::get("icon.png")
        .expect("Failed to get image data")
        .data;
    let icon = image::load_from_memory(&image)
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    app_icon.rgba = icon.into_raw();
    app_icon.width = icon_width;
    app_icon.height = icon_height;

    // Define window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_icon(app_icon),
        ..Default::default()
    };

    // Run window
    eframe::run_native(
        &format!("ER Save Editor {}", env!("CARGO_PKG_VERSION")),
        options,
        Box::new(|creation_context| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);
            creation_context.egui_ctx.set_fonts(fonts);
            let mut visuals = creation_context.egui_ctx.style().visuals.clone();
            let rounding = 3.;
            visuals.window_rounding = Rounding::default().at_least(rounding);
            visuals.window_highlight_topmost = false;
            creation_context.egui_ctx.set_visuals(visuals);
            creation_context
                .egui_ctx
                .set_zoom_factor(DEFAULT_ZOOM_FACTOR);
            Box::new(App::new(creation_context))
        }),
    )
}

pub struct App {
    save_api: Option<SaveApi>,      // Save Api
    vm: ViewModel,                  // ViewModel from the save data
    picked_path: PathBuf,           // Path to current save file for future use when opening dialogs
    current_route: Route,           // Current in app view
    importer_vm: ImporterViewModel, // ViewModel used for the importer
    importer_open: bool,            // Importer Open Flag
}

impl App {
    /// Constructs a new App instance.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            save_api: None,
            picked_path: Default::default(),
            current_route: Route::None,
            vm: ViewModel::default(),
            importer_vm: ImporterViewModel::default(),
            importer_open: false,
        }
    }

    /// Reads the save file from the user picked path.
    pub(crate) fn open(&mut self, path: &PathBuf) -> Result<(), SaveApiError> {
        let save_api = SaveApi::from_path(path)?;
        self.save_api = Some(save_api);
        self.vm = ViewModel::from_save(self.save_api.as_mut().unwrap())?;
        self.picked_path = path.clone();
        Ok(())
    }

    /// Writes the save file to the user picked path.
    pub(crate) fn save(&mut self, path: &PathBuf) -> Result<(), SaveApiError> {
        // Save backup of original save file
        if let Some(save) = &self.save_api {
            // Get formatted date time
            let now = chrono::Utc::now().format("%d_%m_%Y_%H_%M_%S");

            // Clone path and add timestamp to it
            let mut backup_path = path.clone();

            if let Some(parent) = backup_path.parent() {
                // Append paths parent directory with the backup files new path
                backup_path = parent.to_path_buf();
                backup_path.push("backups");
                backup_path.push(format!("{}", chrono::Utc::now().format("%m_%Y")));

                // Create directories for the backups if they don't exist
                create_dir_all(&backup_path)?;
            } else {
                return Err(SaveApiError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Failed to get directory of the selected save path!",
                )));
            }

            // Add now datetime as name for the backup file
            backup_path.push(format!("{}.sl2", now));

            // Write backup file to path
            save.write_to_path(&backup_path)?;
        }

        // Update raw save with edited save data
        self.vm.update_save(&mut self.save_api.as_mut().unwrap())?;

        // Write save file to disk
        self.save_api.as_mut().unwrap().write_to_path(path)?;

        Ok(())
    }

    /// Opens a file dialog intended for picking the path a save file.
    pub(crate) fn open_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("SL2", &["sl2", "Regular Save File"])
            .add_filter("TXT", &["txt", "Save Wizard Exported TXT File"])
            .add_filter("*", &["*", "All files"])
            .set_directory("/")
            .pick_file()
    }

    /// Opens a file dialog intended for picking a path to where the opened save file will be written.
    pub(crate) fn save_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("SL2", &["sl2", "Regular Save File"])
            .add_filter("TXT", &["txt", "Save Wizard Exported TXT File"])
            .add_filter("*", &["*", "Any format"])
            .set_directory("/")
            .save_file()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TOOLBAR (open, save, import)
        toolbar_top_panel(ctx, self);

        // Notifications (for updates getting notified about updates)
        notifications(ctx, self);

        // Settings (for showing dlc items and editing the zoom factor for the application)
        settings_bottom_panel(ctx, self);

        // Only runs when a valid save is open
        if self.save_api.is_some() {
            // INFO (platform, steam_id, char_name)
            information_top_panel(ctx, self);

            // Save characters
            character_list_side_panel(ctx, self);

            // Menu (stats, inventory, equip, etc...)
            menu(ctx, self);

            // Main Content based on menu selection
            egui::CentralPanel::default().show(ctx, |ui| match self.current_route {
                Route::General => general(ui, &mut self.vm),
                Route::Stats => stats(ui, &mut self.vm),
                // Route::Equipment => equipment(ui, &mut self.vm),
                Route::Inventory => inventory(ui, &mut self.vm),
                Route::EventFlags => events(ui, &mut self.vm),
                Route::Regions => regions(ui, &mut self.vm),
                _ => none(ui),
            });
        }
        // File drop when no save is open
        else {
            file_drop_main_panel(ctx, self);
        }
    }
}
