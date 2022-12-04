use std::mem;

impl crate::EditorApp {
    pub fn show_menu(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {
                    self.choose_file();
                }
                if ui.button("Open Folder...").clicked() {
                    self.choose_folder();
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    if let Some(file_path) = &self.file_path {
                        if self.save_data.is_some() {
                            let save_data = mem::take(&mut self.save_data);
                            if let Some(save_data) = save_data {
                                crate::save::save(file_path, save_data);
                            }
                        }
                    }
                }
                if ui.button("Save As...").clicked() {}
                ui.separator();
                if ui.button("Import").clicked() {}
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |ui| if ui.button("Open").clicked() {});
            ui.menu_button("View", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Full Screen").clicked() {
                        crate::view::toggle_fullscreen(frame);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                        ui.label("F11");
                    });
                });
            });
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    }
}
