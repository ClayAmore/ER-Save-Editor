pub mod stats {
    use std::ops::RangeInclusive;

    use eframe::egui::{self, Ui};
    use egui_extras::{Column, TableBody, TableBuilder};
    use crate::{db::classes::classes::STARTER_CLASSES, vm::vm::vm::ViewModel};

    pub fn stats(ui: &mut Ui,  vm: &mut ViewModel) {
        egui::Frame::default()
        .show(ui, |ui| {
            ui.with_layout( egui::Layout::top_down_justified(egui::Align::Min),|ui|{
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui|{

                    ui.heading(vm.slots[vm.index].stats_vm.arche_type.to_string());
                    ui.add_space(8.0);

                    let class = &STARTER_CLASSES.lock().unwrap()[&vm.slots[vm.index].stats_vm.arche_type];

                    // Calculate level from stats
                    let level = 
                        vm.slots[vm.index].stats_vm.vigor + 
                        vm.slots[vm.index].stats_vm.mind + 
                        vm.slots[vm.index].stats_vm.endurance + 
                        vm.slots[vm.index].stats_vm.strength + 
                        vm.slots[vm.index].stats_vm.dexterity + 
                        vm.slots[vm.index].stats_vm.intelligence + 
                        vm.slots[vm.index].stats_vm.faith + 
                        vm.slots[vm.index].stats_vm.arcane-
                        79;


                    let table = TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0));

                    table.body(|mut body| {

                        // Level
                        body.row(24., |mut row| {
                            row.col(|ui| { 
                                ui.label("Level:"); }); 
                            row.col(|ui| { 
                                ui.label(format!("{:6}", level));
                            });
                        });
                        
                        // Stats
                        self::stat_field(&mut body, class.vigor..=99, "Vigor:", &mut vm.slots[vm.index].stats_vm.vigor);
                        self::stat_field(&mut body, class.mind..=99, "Mind:", &mut vm.slots[vm.index].stats_vm.mind);
                        self::stat_field(&mut body, class.endurance..=99, "Endurance:", &mut vm.slots[vm.index].stats_vm.endurance);
                        self::stat_field(&mut body, class.strength..=99, "Strength:", &mut vm.slots[vm.index].stats_vm.strength);
                        self::stat_field(&mut body, class.dexterity..=99, "Dexterity:", &mut vm.slots[vm.index].stats_vm.dexterity);
                        self::stat_field(&mut body, class.intelligence..=99, "Intelligence:", &mut vm.slots[vm.index].stats_vm.intelligence);
                        self::stat_field(&mut body, class.faith..=99, "Faith:", &mut vm.slots[vm.index].stats_vm.faith);
                        self::stat_field(&mut body, class.arcane..=99, "Arcane:", &mut vm.slots[vm.index].stats_vm.arcane);

                        // Space
                        self::space(&mut body, 8.);

                        // Souls
                        let field = egui::widgets::DragValue::new(&mut vm.slots[vm.index].stats_vm.souls)
                            .clamp_range(0..=999999999)
                            .custom_formatter(|n, _|{
                                format!("{:09}", n)
                            });
                        body.row(24., |mut row| {
                            row.col(|ui| {
                                ui.label("Current Souls:");
                            });
                            row.col(|ui| {
                                ui.add(field);
                            });
                        });
                    });
                });
            })
        });
    }

    fn stat_field(body: &mut TableBody, range: RangeInclusive<u32>, name: &str, value: &mut u32) {
        let field = egui::widgets::DragValue::new(value).clamp_range(range);
        body.row(24., |mut row| {
            row.col(|ui| {
                ui.label(name);
            });
            row.col(|ui| {
                ui.add(field);
            });
        });
    }

    fn space(body: &mut TableBody, height: f32) {
        body.row(height, |mut row| {
            row.col(|_| {
            });
            row.col(|_| {
            });
        });
    }
}