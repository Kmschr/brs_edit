use eframe::egui;
use egui::{Button, Color32, RichText, Ui};

const BUTTON_COLOR: Color32 = Color32::from_rgb(15, 98, 254);

pub fn button(ui: &mut Ui, text: &str, enabled: bool) -> bool {
    let text = RichText::new(text).color(Color32::WHITE);
    let b = Button::new(text).fill(BUTTON_COLOR);
    ui.add_enabled(enabled, b).clicked()
}

pub fn header(ui: &mut Ui, text: &str) {
    let text = RichText::new(text).color(Color32::WHITE);
    ui.heading(text);
}
