use brickadia::save::SaveData;
use egui::{CollapsingHeader, DragValue};

pub fn ui(ui: &mut egui::Ui, save_data: &mut SaveData) {
    CollapsingHeader::new("Metadata").default_open(true).show(ui, |ui| {
        ui.visuals_mut().override_text_color = None;

        ui.add_space(10.0);
        ui.strong("BRS Version");
        ui.label(
            "The file format version used for this save. Alpha 5 uses version 10. Can not be changed.",
        );
        ui.add_enabled(false, DragValue::new(&mut save_data.version));
        ui.add_space(5.0);
        ui.strong("Game Version");
        ui.label("Also known as \"Commit Level\" and corresponds to each change tracked by developers. Alpha 5 is currently using CL7870 as seen in the top right of the game. This field was introduced in BRS version 8");
        ui.add(DragValue::new(&mut save_data.game_version));
        ui.add_space(5.0);
    });
}
