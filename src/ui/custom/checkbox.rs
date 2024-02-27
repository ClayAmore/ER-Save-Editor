pub mod checkbox {
    use eframe::egui;

    pub enum State {
        Off = 0,
        On = 1,
        InBetween = 2
    }
    
    pub fn three_states_checkbox(ui: &mut egui::Ui, state: &State) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
        if response.clicked() {
            response.mark_changed();
        }
    
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let rect = rect.expand(visuals.expansion);
            ui.painter().rect(
                rect, 
                2., 
                visuals.bg_fill, 
                visuals.bg_stroke
            );

            match state {
                State::Off => {

                },
                State::On => {
                    ui.painter().line_segment(
                        [
                        egui::pos2(rect.right()-4., rect.top()+4.),
                        egui::pos2(rect.left()+8., rect.bottom()-4.)
                        ],
                        visuals.fg_stroke
                    );
            
                    ui.painter().line_segment(
                        [
                        egui::pos2(rect.left()+8., rect.bottom()-4.),
                        egui::pos2(rect.left()+4., rect.bottom()-8.)
                        ],
                        visuals.fg_stroke
                    );
                },
                State::InBetween => {
                    ui.painter().line_segment(
                        [
                        egui::pos2(rect.left()+4., rect.center().y),
                        egui::pos2(rect.right()-4., rect.center().y)
                        ], 
                        visuals.fg_stroke
                    );
                },
            }
        }
        response
    }
}