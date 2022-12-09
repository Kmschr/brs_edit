use eframe::egui;
use egui::style::Margin;
use egui::*;

pub const TOP_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 5.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    fill: MENU_PANEL_BG,
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

pub const LEFT_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 5.0,
        right: 5.0,
        top: 5.0,
        bottom: 5.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    fill: LEFT_PANEL_BG,
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

pub const RIGHT_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    fill: LEFT_PANEL_BG,
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

pub const CENTER_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 40.0,
        right: 0.0,
        top: 5.0,
        bottom: 5.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    fill: CENTER_PANEL_BG,
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
pub const CENTER_PANEL_BG: Color32 = Color32::from_rgb(30, 30, 30);
pub const LEFT_PANEL_BG: Color32 = Color32::from_rgb(37, 37, 37);
pub const MENU_PANEL_BG: Color32 = Color32::from_rgb(50, 50, 50);
pub const TEXT_EDIT_BG: Color32 = Color32::from_rgb(60, 60, 60);

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

pub fn text_edit_singleline<S>(ui: &mut Ui, text: &mut S)
where
    S: widgets::text_edit::TextBuffer,
{
    ui.visuals_mut().extreme_bg_color = TEXT_EDIT_BG;
    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
    ui.text_edit_singleline(text);
    ui.visuals_mut().override_text_color = None;
}

pub fn text_edit_multiline<S>(ui: &mut Ui, text: &mut S)
where
    S: widgets::text_edit::TextBuffer,
{
    ui.visuals_mut().extreme_bg_color = TEXT_EDIT_BG;
    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
    ui.add(TextEdit::multiline(text).desired_width(600.0));
    ui.visuals_mut().override_text_color = None;
}
