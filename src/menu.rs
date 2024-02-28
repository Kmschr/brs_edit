use crate::shortcuts;
use brickadia::save::SaveData;
use egui::Button;

impl crate::EditorApp {
    pub fn show_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::menu::bar(ui, |ui| {
            self.file_menu_ui(ui, ctx);
            ui.menu_button("Edit", |ui| {
                let enabled = self.save_data.is_some();
                if ui.add_enabled(enabled, Button::new("Delete Bricks By...")).clicked() {
                    self.show_delete_window = true;
                }
                // if ui.add_enabled(enabled, Button::new("Components...")).clicked() {
                //     self.show_components_window = true;
                // }
                // if ui.add_enabled(enabled, Button::new("Ownership...")).clicked() {
                //     self.show_ownership_window = true;
                // }
            });
            view_menu_ui(ui, ctx);
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    }

    fn file_menu_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.menu_button("File", |ui| {
            let new_file_button = Button::new("New File...")
                .shortcut_text(ctx.format_shortcut(&shortcuts::NEW_FILE_SHORTCUT));
            if ui.add(new_file_button).clicked() {
                self.file_path = None;
                self.save_data = Some(SaveData::default());
            }
            ui.separator();
            if ui.button("Open File...").clicked() {
                self.choose_file();
            }
            if ui.button("Open Folder...").clicked() {
                self.choose_folder();
            }
            ui.separator();
            let save_button =
                Button::new("Save").shortcut_text(ctx.format_shortcut(&shortcuts::SAVE_SHORTCUT));
            if ui.add(save_button).clicked() {
                if self.file_path.is_some() {
                    self.save();
                } else {
                    self.choose_save_as();
                }
            }
            let save_as_button = Button::new("Save As...")
                .shortcut_text(ctx.format_shortcut(&shortcuts::SAVE_AS_SHORTCUT));
            if ui.add(save_as_button).clicked() {
                self.choose_save_as();
            }
            ui.separator();
            // ui.menu_button("Import", |ui| {
            //     if ui.button("Blockland Save (.BLS)").clicked() {}
            //     if ui.button("Wavefront (.OBJ)").clicked() {}
            //     if ui.button("Heightmap").clicked() {}
            // });
            ui.menu_button("Export", |ui| {
                if ui.button("Preview Image").clicked() {
                    self.choose_export_preview();
                }
                if ui.button("Color Palette").clicked() {
                    self.choose_export_palette();
                }
            });
            ui.separator();
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn view_menu_ui(ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.menu_button("View", |ui| {
        let fullscreen_button = Button::new("Full Screen")
            .shortcut_text(ctx.format_shortcut(&shortcuts::FULLSCREEN_SHORTCUT));
        if ui.add(fullscreen_button).clicked() {
            crate::view::toggle_fullscreen(ctx);
        }
        ui.separator();
        egui::gui_zoom::zoom_menu_buttons(ui);
    });
}
