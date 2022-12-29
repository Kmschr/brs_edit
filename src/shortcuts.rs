use egui::{
    Key,
    KeyboardShortcut,
    Modifiers,
};
use crate::EditorApp;

pub const SAVE_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: true,
    },
    key: Key::S,
};
pub const SAVE_AS_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: true,
        mac_cmd: false,
        command: true,
    },
    key: Key::S,
};
pub const FULLSCREEN_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    },
    key: Key::F11,
};
pub const NEW_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: true,
    },
    key: Key::N,
};

impl EditorApp {
    pub fn handle_shortcuts(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !frame.is_web() {
            egui::gui_zoom::zoom_with_keyboard_shortcuts(ctx, frame.info().native_pixels_per_point);
        }
        if ctx.input_mut().consume_shortcut(&FULLSCREEN_SHORTCUT) {
            crate::view::toggle_fullscreen(frame);
        }
        if ctx.input_mut().consume_shortcut(&SAVE_SHORTCUT) {
            self.save(ctx);
        }
        if ctx.input_mut().consume_shortcut(&SAVE_AS_SHORTCUT) {
            self.choose_save_as();
        }
    }
}
