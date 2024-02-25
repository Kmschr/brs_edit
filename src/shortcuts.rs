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
    logical_key: Key::S,
};
pub const SAVE_AS_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: true,
        mac_cmd: false,
        command: true,
    },
    logical_key: Key::S,
};
pub const FULLSCREEN_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    },
    logical_key: Key::F11,
};
pub const NEW_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut {
    modifiers: Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: true,
    },
    logical_key: Key::N,
};

impl EditorApp {
    pub fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_shortcut(&FULLSCREEN_SHORTCUT)) {
            crate::view::toggle_fullscreen(ctx);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&SAVE_SHORTCUT)) {
            self.save();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&SAVE_AS_SHORTCUT)) {
            self.choose_save_as();
        }
    }
}
