use egui::{Key, KeyboardShortcut, Modifiers};

use crate::EditorApp;

pub const SAVE_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: true,
        shift: false,
        mac_cmd: false,
        command: false,
    },
    key: Key::S,
};

pub const SAVE_AS_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: true,
        shift: true,
        mac_cmd: false,
        command: false,
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

impl EditorApp {
    pub fn handle_shortcuts(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input_mut().consume_shortcut(&FULLSCREEN_SHORTCUT) {
            crate::view::toggle_fullscreen(frame);
        }
        if ctx.input_mut().consume_shortcut(&SAVE_SHORTCUT) {
            self.save(ctx);
        }
    }
}