use crate::shortcuts;
use egui::Button;

impl crate::EditorApp {
    pub fn show_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::menu::bar(ui, |ui| {
            self.file_menu_ui(ui, ctx);
            ui.menu_button("Edit", |ui| if ui.button("Delete Bricks").clicked() {
                self.show_delete_window = true;
            });
            view_menu_ui(ui, ctx, frame);
            ui.menu_button("Help", |ui| if ui.button("About").clicked() { });
        });
    }

    fn file_menu_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.menu_button("File", |ui| {
            if ui.button("Open File...").clicked() {
                self.choose_file();
            }
            if ui.button("Open Folder...").clicked() {
                self.choose_folder();
            }
            ui.separator();
            let save_button = Button::new("Save").shortcut_text(ctx.format_shortcut(&shortcuts::SAVE_SHORTCUT));
            if ui.add(save_button).clicked() {
                self.save(ctx);
            }
            let save_as_button =
                Button::new("Save As...").shortcut_text(ctx.format_shortcut(&shortcuts::SAVE_AS_SHORTCUT));
            if ui.add(save_as_button).clicked() {
                self.choose_save_as();
            }
            ui.separator();
            if ui.button("Import").clicked() { }
            ui.separator();
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn view_menu_ui(ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ui.menu_button("View", |ui| {
        let fullscreen_button =
            Button::new("Full Screen").shortcut_text(ctx.format_shortcut(&shortcuts::FULLSCREEN_SHORTCUT));
        if ui.add(fullscreen_button).clicked() {
            crate::view::toggle_fullscreen(frame);
        }
        ui.separator();
        egui::gui_zoom::zoom_menu_buttons(ui, frame.info().native_pixels_per_point);
    });
}
