use eframe::egui::{
    NumExt, Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType,
};

pub struct SelectableButton {
    selected: bool,
    text: WidgetText,
}

impl SelectableButton {
    pub fn new(selected: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            selected,
            text: text.into(),
        }
    }
}

impl Widget for SelectableButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { selected, text } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let galley = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_extra + galley.size();
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());
        response.widget_info(|| WidgetInfo::selected(WidgetType::Button, selected, galley.text()));

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, selected);

            if selected || response.hovered() || response.highlighted() || response.has_focus() {
                let rect = rect.expand(visuals.expansion);

                ui.painter().rect(
                    rect,
                    visuals.rounding,
                    ui.visuals().widgets.inactive.bg_fill,
                    ui.visuals().widgets.hovered.bg_stroke,
                );
            } else {
                ui.painter().rect(
                    rect,
                    visuals.rounding,
                    ui.visuals().widgets.inactive.bg_fill,
                    ui.visuals().widgets.inactive.bg_stroke,
                );
            }

            ui.painter()
                .galley(text_pos, galley, ui.visuals().widgets.active.text_color());
        }

        response
    }
}
