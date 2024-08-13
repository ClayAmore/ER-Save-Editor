use crate::{
    ui::custom::selectable::SelectableButton,
    vm::{
        inventory::browse::{BrowseStorageType, BrowseTypeRoute},
        vm::ViewModel,
    },
};
use eframe::{
    egui::{self, Margin, TextFormat, Ui},
    epaint::{text::LayoutJob, Color32},
};
use er_save_lib::ItemType;

pub fn browse_inventory(ui: &mut Ui, vm: &mut ViewModel) {
    let inventory_vm = &mut vm.characters[vm.index].inventory_vm;
    ui.columns(2, |uis| {
        let held_button = uis[0].add_sized(
            [100., 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_storage_type == BrowseStorageType::Held,
                "Held",
            ),
        );
        let storage_box_button = uis[1].add_sized(
            [100., 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_storage_type == BrowseStorageType::StorageBox,
                "Storage Box",
            ),
        );

        if held_button.clicked() {
            inventory_vm.browse_vm.current_storage_type = BrowseStorageType::Held;
            inventory_vm.browse_vm.filter(&inventory_vm.filter_text);
        };
        if storage_box_button.clicked() {
            inventory_vm.browse_vm.current_storage_type = BrowseStorageType::StorageBox;
            inventory_vm.browse_vm.filter(&inventory_vm.filter_text);
        };
    });

    ui.add_space(6.);

    ui.columns(6, |uis| {
        let common_items = uis[0].add_sized(
            [uis[0].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::RegularItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Item,
                "Common Item",
            ),
        );
        let key_items = uis[1].add_sized(
            [uis[1].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::KeyItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Item,
                "Key Item",
            ),
        );
        let weapons = uis[2].add_sized(
            [uis[2].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::RegularItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Weapon,
                "Weapons",
            ),
        );
        let armors = uis[3].add_sized(
            [uis[3].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::RegularItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Armor,
                "Armors",
            ),
        );
        let ashofwar = uis[4].add_sized(
            [uis[4].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::RegularItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Aow,
                "Ash of War",
            ),
        );
        let talismans = uis[5].add_sized(
            [uis[5].available_width(), 40.],
            SelectableButton::new(
                inventory_vm.browse_vm.current_type_route == BrowseTypeRoute::RegularItems
                    && inventory_vm.browse_vm.current_sub_type_route == ItemType::Accessory,
                "Talismans",
            ),
        );

        if common_items.clicked() {
            inventory_vm
                .browse_vm
                .to_regular_items_route(ItemType::Item, &inventory_vm.filter_text);
        }
        if key_items.clicked() {
            inventory_vm
                .browse_vm
                .to_key_items_route(&inventory_vm.filter_text);
        }
        if weapons.clicked() {
            inventory_vm
                .browse_vm
                .to_regular_items_route(ItemType::Weapon, &inventory_vm.filter_text);
        }
        if armors.clicked() {
            inventory_vm
                .browse_vm
                .to_regular_items_route(ItemType::Armor, &inventory_vm.filter_text);
        }
        if ashofwar.clicked() {
            inventory_vm
                .browse_vm
                .to_regular_items_route(ItemType::Aow, &inventory_vm.filter_text);
        }
        if talismans.clicked() {
            inventory_vm
                .browse_vm
                .to_regular_items_route(ItemType::Accessory, &inventory_vm.filter_text);
        }
    });

    ui.add_space(6.);

    ui.horizontal(|ui| {
        let height = 20.;
        let label = ui.label("Filter: ");
        if ui
            .add_sized(
                [ui.available_size().x, height],
                egui::widgets::TextEdit::singleline(&mut inventory_vm.filter_text),
            )
            .labelled_by(label.id)
            .changed()
        {
            inventory_vm.browse_vm.filter(&inventory_vm.filter_text);
        };
    });

    let mut frame = egui::Frame::none();
    frame.inner_margin = Margin {
        top: 8.,
        left: 0.,
        bottom: 8.,
        right: 0.,
    };
    frame.show(ui, |ui| {
        egui::Grid::new("browse_header")
            .spacing([16., 16.])
            .min_col_width(ui.available_width() / 4.)
            .striped(true)
            .show(ui, |ui| {
                // Table Header
                let mut job = LayoutJob::default();
                job.append(
                    "Item ID",
                    0.,
                    TextFormat {
                        color: Color32::BLACK,
                        ..Default::default()
                    },
                );
                ui.label(job);

                let mut job = LayoutJob::default();
                job.append(
                    "Item Name",
                    0.,
                    TextFormat {
                        color: Color32::BLACK,
                        ..Default::default()
                    },
                );
                ui.label(job);

                let mut job = LayoutJob::default();
                job.append(
                    "Quantity",
                    0.,
                    TextFormat {
                        color: Color32::BLACK,
                        ..Default::default()
                    },
                );
                ui.label(job);

                let mut job = LayoutJob::default();
                job.append(
                    "Acquisition Sort ID",
                    0.,
                    TextFormat {
                        color: Color32::BLACK,
                        ..Default::default()
                    },
                );
                ui.label(job);
                ui.end_row();
            });
    });

    egui::ScrollArea::vertical().show_rows(
        ui,
        10.,
        inventory_vm.browse_vm.current_item_list.len(),
        |ui, row_range| {
            egui::Grid::new("browse_body")
                .spacing([8., 8.])
                .min_col_width(ui.available_width() / 4.)
                .striped(true)
                .show(ui, |ui| {
                    for i in row_range {
                        let item = inventory_vm.browse_vm.current_item_list[i].clone();
                        ui.label(format!("{}", item.as_ref().borrow().item_id));
                        ui.add(
                            egui::Label::new(item.as_ref().borrow().item_name.to_string())
                                .wrap(true),
                        );
                        ui.label(format!("{}", item.as_ref().borrow().quantity));
                        ui.label(format!("{}", item.as_ref().borrow().aqcuistion_index));
                        ui.end_row();
                    }
                });
        },
    );
}
