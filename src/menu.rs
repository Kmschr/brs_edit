use crate::EditorApp;

impl EditorApp {
    pub fn show_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {
                    self.choose_file();
                }
                if ui.button("Open Folder...").clicked() {
                    self.choose_folder();
                }
                ui.separator();
                if ui.button("Save").clicked() {}
                if ui.button("Save As...").clicked() {}
                ui.separator();
                if ui.button("Import").clicked() {}
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |ui| if ui.button("Open").clicked() {});
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    }
}
