use crate::gui;
use brickadia::save::SaveData;
use egui::*;
use num_format::{Locale, ToFormattedString};

const NUM_COLOR_COLUMNS: usize = 4;

pub fn show_header_two(
    save_data: &mut SaveData,
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
            mods(save_data, ui);
            ui.add_space(5.0);
            brick_assets(save_data, ui);
            ui.add_space(5.0);
            colors(save_colors, ui);
            ui.add_space(5.0);
        });
}

fn mods(save_data: &mut SaveData, ui: &mut egui::Ui) {
    CollapsingHeader::new("Mods").show(ui, |ui| {
        ui.label("No longer used, but can be found in older saves");

        let mut delete_mod_index = None;
        let mods = &mut save_data.header2.mods;
        for i in 0..mods.len() {
            ui.horizontal(|ui| {
                gui::text_edit_singleline(ui, &mut mods[i]);
                if ui.small_button("🗙").clicked() {
                    delete_mod_index = Some(i);
                }
            });
        }
        if let Some(delete_mod_index) = delete_mod_index {
            mods.remove(delete_mod_index);
        }
        if gui::button(ui, "Add Mod", true) {
            mods.push("".into());
        }
    });
}

fn brick_assets(save_data: &mut SaveData, ui: &mut egui::Ui) {
    CollapsingHeader::new("Brick Assets").show(ui, |ui| {
        ui.label("Messing with this can break the save, but you can do some useful things like change regular procedural bricks to micro bricks.");


        let mut delete_brick_asset_index = None;
        let brick_assets = &mut save_data.header2.brick_assets;
        for i in 0..brick_assets.len() {
            ui.horizontal(|ui| {
                gui::text_edit_singleline(ui, &mut brick_assets[i]);
                if ui.small_button("🗙").clicked() {
                    delete_brick_asset_index = Some(i);
                }
            });
        }
        if let Some(delete_brick_asset_index) = delete_brick_asset_index {
            brick_assets.remove(delete_brick_asset_index);
        }
        if gui::button(ui, "Add Brick Asset", true) {
            brick_assets.push("".into());
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

        // for row in 0..(save_colors.len() / 10 + 1) {
        //     ui.horizontal(|ui| {
        //         for col in 0..10 {
        //             let i = row * 10 + col;
        //             if i >= save_colors.len() {
        //                 break;
        //             }
        //             ui.color_edit_button_srgba(&mut save_colors[i]);
        //         }
        //     });
        // }
        // if let Some(delete_color_index) = delete_color_index {
        //     save_colors.remove(delete_color_index);
        // }
        // if gui::button(ui, "Add Color", true) {
        //     save_colors.push(Color32::from_rgba_premultiplied(255, 255, 255, 255));
        // }
    });
}
