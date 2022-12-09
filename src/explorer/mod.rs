mod filetree;
mod preview;

use crate::EditorApp;

impl EditorApp {
    pub fn explorer_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("\tEXPLORER");
        ui.add_space(5.0);
        preview::ui(ui, &self.preview_handle);
        self.filetree_ui(ui, ctx);
    }
}
