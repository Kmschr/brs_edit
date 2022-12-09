use crate::shortcuts;
use egui::Button;

impl crate::EditorApp {
    pub fn show_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {
                    self.choose_file();
                }
                if ui.button("Open Folder...").clicked() {
                    self.choose_folder();
                }
                ui.separator();

                let save_button = Button::new("Save")
                    .shortcut_text(ctx.format_shortcut(&shortcuts::SAVE_SHORTCUT));
                if ui.add(save_button).clicked() {
                    self.save(ctx);
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
                let fullscreen_button = Button::new("Full Screen")
                    .shortcut_text(ctx.format_shortcut(&shortcuts::FULLSCREEN_SHORTCUT));
                if ui.add(fullscreen_button).clicked() {
                    crate::view::toggle_fullscreen(frame);
                }
            });
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    }
}
