use std::{
    sync::mpsc::Receiver,
    path::PathBuf,
};
use egui::{
    TextureHandle,
    CollapsingHeader,
    TextStyle,
    Vec2,
};
use eframe::epaint;
use crate::{
    file_dialog,
    gui,
};

const MAX_PREVIEW_WIDTH: f32 = 640.0;
const MAX_PREVIEW_HEIGHT: f32 = 360.0;
const MAX_PREVIEW_ASPECT_RATIO: f32 = MAX_PREVIEW_WIDTH / MAX_PREVIEW_HEIGHT;

pub fn ui(ui: &mut egui::Ui, preview: &Option<TextureHandle>) -> Option<Receiver<Option<PathBuf>>> {
    let mut ret = None;
    CollapsingHeader::new("Preview").default_open(true).show(ui, |ui| {
        ui.visuals_mut().override_text_color = None;
        if let Some(style) = ui.style_mut().text_styles.get_mut(&TextStyle::Button) {
            style.size = 14.0;
        }
        ui.add_space(5.0);
        ui.label(
            "Brickadia stores preview images as PNG or JPEG images. If you select an image not in one of these formats it will convert it to PNG",
        );
        ui.add_space(5.0);
        if gui::button(ui, "Choose Image...", true) {
            ret = Some(file_dialog::choose_preview());
        }
        if let Some(texture) = preview {
            let preview_size = texture.size_vec2();
            ui.label(format!("{} x {}", preview_size.x, preview_size.y));
            let display_size = contain_preview_size(preview_size);
            egui::Frame::none().shadow(epaint::Shadow::big_dark()).show(ui, |ui| {
                let image = egui::Image::new(texture).max_size(display_size);
                ui.add(image);
            });
        }
        ui.add_space(5.0);
    });
    ret
}

fn contain_preview_size(preview_size: Vec2) -> Vec2 {
    let preview_aspect_ratio = preview_size.x / preview_size.y;
    if preview_aspect_ratio > MAX_PREVIEW_ASPECT_RATIO {
        [MAX_PREVIEW_HEIGHT * preview_aspect_ratio, MAX_PREVIEW_HEIGHT].into()
    } else {
        [MAX_PREVIEW_WIDTH, MAX_PREVIEW_WIDTH / preview_aspect_ratio].into()
    }
}
