use eframe::egui;
use egui::style::Margin;
use egui::*;

pub const BOTTOM_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 5.0,
        right: 0.0,
        top: 5.0,
        bottom: 0.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    fill: BLUE,
    rounding: Rounding {
        ne: 0.0,
        nw: 0.0,
        se: 0.0,
        sw: 0.0,
    },
    shadow: eframe::epaint::Shadow {
        extrusion: 0.0,
        color: Color32::WHITE,
    },
    stroke: Stroke {
        width: 0.0,
        color: Color32::WHITE,
    },
};

pub const BLUE: Color32 = Color32::from_rgb(0, 122, 204);

pub fn button(ui: &mut Ui, text: &str, enabled: bool) -> bool {
    let text = RichText::new(text).color(Color32::WHITE);
    let b = Button::new(text).fill(BLUE);
    ui.add_enabled(enabled, b).clicked()
}

pub fn header(ui: &mut Ui, text: &str) {
    let text = RichText::new(text).color(Color32::WHITE);
    ui.heading(text);
}

pub fn fill_horizontal(ui: &mut Ui) {
    ui.allocate_space([ui.available_width(), 0.0].into());
}
