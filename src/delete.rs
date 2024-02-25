use egui::{
    Window,
    DragValue,
};
use itertools::Itertools;
use num_format::{
    Locale,
    ToFormattedString,
};
use crate::{
    EditorApp,
    open,
};

impl EditorApp {
    pub fn delete_ui(&mut self, ctx: &egui::Context) {
        let mut colors: Vec<(usize, [f32; 4], u32)> =
            self.save_colors.iter().enumerate().map(|(i, brick)| (i, brick.0, brick.1)).filter(|color| {
                color.2 > 0
            }).sorted_by_key(|color| -(color.2 as i32)).collect();
        Window::new("Delete Bricks").open(&mut self.show_delete_window).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Delete Bricks By Color");
                egui::Grid::new("color grid").striped(true).min_col_width(150.0).show(ui, |ui| {
                    for row in 0 .. (colors.len() / 4 + 1) {
                        for col in 0 .. 4 {
                            let i = row * 4 + col;
                            if i >= colors.len() {
                                break;
                            }
                            let (i, color, bricks) = &mut colors[i];
                            ui.horizontal(|ui| {
                                ui.color_edit_button_rgba_premultiplied(color);
                                ui.add_enabled(false, DragValue::new(bricks).custom_formatter(|n, _| {
                                    (n as i32).to_formatted_string(&Locale::en)
                                }).suffix(" bricks"));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("🗑").clicked() {
                                        if let Some(save_data) = &mut self.save_data {
                                            save_data.bricks.retain(|brick| match brick.color {
                                                brickadia::save::BrickColor::Index(n) => n != *i as u32,
                                                brickadia::save::BrickColor::Unique(_) => true,
                                            });
                                            open::load_colors(&mut self.save_colors, &save_data.header2.colors);
                                            open::count_colors(&mut self.save_colors, &save_data.bricks);
                                        }
                                    }
                                });
                            });
                        }
                        ui.end_row();
                    }
                });
                ui.heading("Delete Bricks By Material");
                if let Some(save_data) = &mut self.save_data {
                    for (i, material) in save_data.header2.materials.iter().enumerate() {
                        let material = material.clone().replace("BMC_", "");
                        ui.horizontal(|ui| {
                            ui.label(material);
                            if ui.small_button("🗑").clicked() {
                                save_data.bricks.retain(|brick| brick.material_index != i as u32);
                            }
                        });
                    }
                }
                ui.heading("Delete Bricks by Asset");
                if let Some(save_data) = &mut self.save_data {
                    for (i, asset) in save_data.header2.brick_assets.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(asset);
                            if ui.small_button("🗑").clicked() {
                                save_data.bricks.retain(|brick| brick.asset_name_index != i as u32);
                            }
                        });
                    }
                }
                ui.heading("Delete Bricks by Owner");
                if let Some(save_data) = &mut self.save_data {
                    for (i, owner) in save_data.header2.brick_owners.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(owner.name.clone());
                            if ui.small_button("🗑").clicked() {
                                save_data.bricks.retain(|brick| brick.owner_index != i as u32);
                            }
                        });
                    }
                }
            });
        });
    }
}
