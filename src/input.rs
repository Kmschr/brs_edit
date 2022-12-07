use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::EditorApp;

impl EditorApp {
    pub fn handle_shortcuts(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input().key_pressed(egui::Key::F11) {
            crate::view::toggle_fullscreen(frame);
        }
        if ctx.input().key_pressed(egui::Key::S) {
            let device_state = DeviceState::new();
            if device_state.get_keys().contains(&Keycode::LControl) {
                self.save(ctx);
            }
        }
    }
}
