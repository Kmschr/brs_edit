use crate::gui;
use brickadia::save::Header2;
use egui::*;
use num_format::{Locale, ToFormattedString};

const NUM_COLOR_COLUMNS: usize = 4;

pub fn show_header_two(
    header2: &mut Header2,
    save_colors: &mut Vec<([f32; 4], u32)>,
    ui: &mut egui::Ui,
) {
    CollapsingHeader::new("Header2")
        .default_open(true)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = None;
            if let Some(style) = ui.style_mut().text_styles.get_mut(&TextStyle::Button) {
                style.size = 14.0;
            }

            ui.add_space(5.0);
            string_list(
                "Mods",
                "No longer used, but can be found in older saves",
                "Add Mod",
                &mut header2.mods,
                ui,
            );
            ui.add_space(5.0);
            string_list(
                "Brick Assets",
                "Messing with this can break the save, but you can do some useful things like change regular procedural bricks to micro bricks.",
                "Add Brick Asset",
                &mut header2.brick_assets,
                ui,
            );
            ui.add_space(5.0);
            colors(save_colors, ui);
            ui.add_space(5.0);
            string_list(
                "Materials",
                "Messing with this can break the save, but you can do some funny things like replace all glass with hologram.",
                "Add Material",
                &mut header2.materials,
                ui,
            );
            ui.add_space(5.0);
            string_list(
                "Physical Materials",
                "Physical materials in the save",
                "Add Physical Material",
                &mut header2.physical_materials,
                ui,
            );
        });
}

fn string_list(
    header: &str,
    label: &str,
    button_label: &str,
    list: &mut Vec<String>,
    ui: &mut egui::Ui,
) {
    CollapsingHeader::new(header).show(ui, |ui| {
        ui.label(label);

        let mut delete_index = None;
        for i in 0..list.len() {
            ui.horizontal(|ui| {
                gui::text_edit_singleline(ui, &mut list[i]);
                if ui.small_button("🗙").clicked() {
                    delete_index = Some(i);
                }
            });
        }
        if let Some(delete_index) = delete_index {
            list.remove(delete_index);
        }
        if gui::button(ui, button_label, true) {
            list.push("".into());
        }
    });
}

fn colors(save_colors: &mut Vec<([f32; 4], u32)>, ui: &mut egui::Ui) {
    CollapsingHeader::new("Color Palette").show(ui, |ui| {
        ui.label("Used by bricks with color indexes.");

        egui::Grid::new("color grid")
            .striped(true)
            .min_col_width(150.0)
            .show(ui, |ui| {
                for row in 0..(save_colors.len() / NUM_COLOR_COLUMNS + 1) {
                    for col in 0..NUM_COLOR_COLUMNS {
                        let i = row * NUM_COLOR_COLUMNS + col;
                        if i >= save_colors.len() {
                            break;
                        }
                        let (color, bricks) = &mut save_colors[i];
                        ui.horizontal(|ui| {
                            ui.color_edit_button_rgba_premultiplied(color);
                            ui.add_enabled(
                                false,
                                DragValue::new(bricks)
                                    .custom_formatter(|n, _| {
                                        (n as i32).to_formatted_string(&Locale::en)
                                    })
                                    .suffix(" bricks"),
                            );
                        });
                    }
                    ui.end_row();
                }
            });
    });
}
