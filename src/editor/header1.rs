use brickadia::save::Header1;
use egui::{CollapsingHeader, DragValue};
use num_format::{Locale, ToFormattedString};

use crate::gui;

pub fn ui(ui: &mut egui::Ui, header1: &mut Header1) {
    CollapsingHeader::new("Header1")
        .default_open(true)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = None;

            ui.add_space(5.0);
            ui.strong("Map");
            ui.label("Which game environment the save was generated in.");
            gui::text_edit_singleline(ui, &mut header1.map);

            ui.add_space(5.0);

            ui.strong("Description");
            gui::text_edit_multiline(ui, &mut header1.description);

            ui.add_space(5.0);

            ui.strong("Author: Name");
            ui.label("Who created this save file, not always the builder of the save.");
            gui::text_edit_singleline(ui, &mut header1.author.name);

            ui.add_space(5.0);

            ui.strong("Author: ID");
            ui.label(
                "Player ID of who created this save file, not always the builder of the save.",
            );
            ui.code(header1.author.id.to_string());

            ui.add_space(5.0);

            ui.strong("Brickcount");
            ui.add_enabled(
                false,
                DragValue::new(&mut header1.brick_count)
                    .custom_formatter(|n, _| (n as i32).to_formatted_string(&Locale::en))
                    .suffix(" bricks"),
            );
            ui.add_space(5.0);
        });
}
