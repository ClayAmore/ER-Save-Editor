pub mod none {
    use eframe::egui::{self, Ui};

    pub fn none(ui: &mut Ui) {
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            ui.label("Empty");
        });
    }
}