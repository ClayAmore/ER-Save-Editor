pub mod events {

    use eframe::egui::{self, Ui};
    use crate::{db::{cookbooks::books::COOKBOKS, graces::maps::GRACES, map_name::maps::{MapName, MAP_NAME}, maps::maps::MAPS, whetblades::whetblades::WHETBLADES}, vm::{events::events_view_model::EventsRoute, vm::vm::ViewModel}};

    pub fn events(ui: &mut Ui, vm:&mut ViewModel) {
        egui::SidePanel::left("inventory_menu").show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let sites_of_grace = ui.add_sized([100., 40.], egui::Button::new("Sites Of Grace"));
                    let whetblades = ui.add_sized([100., 40.], egui::Button::new("Whetblades"));
                    let cookboks = ui.add_sized([100., 40.], egui::Button::new("Cookboks"));
                    let maps = ui.add_sized([100., 40.], egui::Button::new("Maps"));
                    let bosses = ui.add_sized([100., 40.], egui::Button::new("Bosses"));
                    let npcs = ui.add_sized([100., 40.], egui::Button::new("NPCS"));
                    let doors_floors_walls = ui.add_sized([100., 60.], egui::Button::new("Doors\nFloors\nWalls"));
                    let misc = ui.add_sized([100., 40.], egui::Button::new("Misc"));
        
                    if sites_of_grace.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::SitesOfGrace}
                    if whetblades.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Whetblades}
                    if cookboks.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Cookboks}
                    if maps.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Maps}
                    if bosses.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Bosses}
                    if npcs.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::NPCS}
                    if doors_floors_walls.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::DoorsFloorsWalls}
                    if misc.clicked() {vm.slots[vm.index].events_vm.current_route = EventsRoute::Misc}
        
                    // Highlight active 
                    match vm.slots[vm.index].events_vm.current_route {
                        EventsRoute::None => {},
                        EventsRoute::SitesOfGrace => {sites_of_grace.highlight();},
                        EventsRoute::Whetblades => {whetblades.highlight();},
                        EventsRoute::Cookboks => {cookboks.highlight();},
                        EventsRoute::Maps => {maps.highlight();},
                        EventsRoute::Bosses => {bosses.highlight();},
                        EventsRoute::NPCS => {npcs.highlight();},
                        EventsRoute::DoorsFloorsWalls => {doors_floors_walls.highlight();},
                        EventsRoute::Misc => {misc.highlight();},
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ui.ctx(), |ui|{
            egui::ScrollArea::vertical()
            .id_source("left")
            .auto_shrink(false)
            .show(ui, |ui| {
                match vm.slots[vm.index].events_vm.current_route {
                    EventsRoute::None => {},
                    EventsRoute::SitesOfGrace => {graces(ui, vm);},
                    EventsRoute::Whetblades => {whetblades(ui, vm);},
                    EventsRoute::Cookboks => {cookbooks(ui, vm);},
                    EventsRoute::Maps => {maps(ui, vm);},
                    EventsRoute::Bosses => {},
                    EventsRoute::NPCS => {},
                    EventsRoute::DoorsFloorsWalls => {},
                    EventsRoute::Misc => {},
                }
            });
        });
        
    }

    fn graces(ui: &mut Ui, vm:&mut ViewModel) {
        let maps = &vm.slots[vm.index].events_vm.grace_groups;
        let graces = &mut vm.slots[vm.index].events_vm.graces;
        for map in maps {
            ui.push_id(map.0, |ui| {
                let collapsing = egui::containers::collapsing_header::CollapsingHeader::new(MAP_NAME.lock().unwrap()[&map.0]);
                ui.horizontal(|ui|{
                    ui.checkbox(&mut false, "");
                    collapsing.show(ui, |ui| {
                        for grace in map.1 {
                            let grace_info: (MapName, u32, &str) = GRACES.lock().unwrap()[&grace];
                            let on = graces.get_mut(grace).expect("");
                            ui.checkbox(on, grace_info.2.to_string());
                        }
                    });
                })
            });
        }
    }

    fn whetblades(ui: &mut Ui, vm:&mut ViewModel) {
        let whetblades = &mut vm.slots[vm.index].events_vm.whetblades;
        ui.checkbox(&mut false, "");
        for (whetblade, on) in whetblades {
            let whetblade_info: (u32,&str) = WHETBLADES.lock().unwrap()[&whetblade];
            ui.checkbox(on, whetblade_info.1.to_string());
        }
    }

    fn cookbooks(ui: &mut Ui, vm:&mut ViewModel) {
        let cookbooks = &mut vm.slots[vm.index].events_vm.cookbooks;
        ui.checkbox(&mut false, "");

        for (cookbook, on) in cookbooks {
            let cookbook_info: (u32,&str) = COOKBOKS.lock().unwrap()[&cookbook];
            ui.checkbox(on, cookbook_info.1.to_string());
        }
    }

    fn maps(ui: &mut Ui, vm:&mut ViewModel) {
        let maps = &mut vm.slots[vm.index].events_vm.maps;
        ui.checkbox(&mut false, "");

        for (map, on) in maps {
            let map_info: (u32,&str) = MAPS.lock().unwrap()[&map];
            ui.checkbox(on, map_info.1.to_string());
        }
    }
}