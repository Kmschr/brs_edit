mod bricks;
mod header1;
mod header2;
mod metadata;
mod preview;

use crate::{gui, EditorApp};

impl EditorApp {
    pub fn editor_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(save_data) = &mut self.save_data {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
            metadata::ui(ui, save_data);
            header1::ui(ui, &mut save_data.header1);
            header2::ui(ui, &mut save_data.header2, &mut self.save_colors);
            let new_preview_receiver = preview::ui(ui, &self.preview_handle);
            if new_preview_receiver.is_some() {
                self.receivers.preview_path_receiver = new_preview_receiver;
            }
            bricks::ui(ui, &mut save_data.bricks);
        }
        gui::fill_horizontal(ui);
    }
}
