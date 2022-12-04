pub fn handle_key_presses(ctx: &egui::Context, frame: &mut eframe::Frame) {
    if ctx.input().key_pressed(egui::Key::F11) {
        crate::view::toggle_fullscreen(frame);
    }
}
