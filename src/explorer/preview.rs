use egui::TextureHandle;

const PREVIEW_HEIGHT: f32 = 100.0;

/// Show the save preview image, maintaining original aspect ratio
pub fn ui(ui: &mut egui::Ui, preview_handle: &Option<TextureHandle>) {
    if let Some(texture) = preview_handle {
        image_ui(ui, texture);
    } else {
        empty_ui(ui);
    }
    ui.separator();
}

fn image_ui(ui: &mut egui::Ui, texture: &TextureHandle) {
    ui.vertical_centered(|ui| {
        let preview_size = texture.size_vec2();
        let preview_aspect_ratio = preview_size.x / preview_size.y;
        ui.image(
            texture,
            [PREVIEW_HEIGHT * preview_aspect_ratio, PREVIEW_HEIGHT],
        );
    });
}

fn empty_ui(ui: &mut egui::Ui) {
    let desired_size = egui::vec2(0.0, PREVIEW_HEIGHT);
    ui.allocate_space(desired_size);
}
