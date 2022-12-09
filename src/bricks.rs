use crate::gui;
use brickadia::save::Brick;
use egui::{CollapsingHeader, DragValue, Vec2};
use egui_extras::{Size, StripBuilder, TableBuilder};

pub fn show_bricks(bricks: &mut Vec<Brick>, ui: &mut egui::Ui) {
    CollapsingHeader::new("Bricks")
        .default_open(true)
        .show(ui, |ui| {
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(500.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            table_ui(bricks, ui);
                        });
                    });
                });
        });
}

fn table_ui(bricks: &mut Vec<Brick>, ui: &mut egui::Ui) {
    TableBuilder::new(ui)
        .striped(true)
        .column(Size::exact(50.0))
        .column(Size::exact(70.0))
        .columns(Size::exact(100.0), 3)
        .column(Size::exact(150.0))
        .column(Size::exact(150.0))
        .header(14.0, |mut header| {
            header.col(|ui| {
                ui.label("Brick #");
            });
            header.col(|ui| {
                ui.label("Asset Index");
            });
            header.col(|ui| {
                ui.label("Collision");
            });
            header.col(|ui| {
                ui.label("Owner Index");
            });
            header.col(|ui| {
                ui.label("Material");
            });
            header.col(|ui| {
                ui.label("Position");
            });
            header.col(|ui| {
                ui.label("Size");
            });
        })
        .body(|body| {
            body.rows(15.0, bricks.len(), |row_index, mut row| {
                row.col(|ui| {
                    ui.label(row_index.to_string());
                });
                row.col(|ui| {
                    ui.add(DragValue::new(&mut bricks[row_index].asset_name_index));
                });
                row.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        ui.checkbox(&mut bricks[row_index].collision.player, "")
                            .on_hover_text("Player");
                        ui.checkbox(&mut bricks[row_index].collision.tool, "")
                            .on_hover_text("Tool");
                        ui.checkbox(&mut bricks[row_index].collision.weapon, "")
                            .on_hover_text("Weapon");
                        ui.checkbox(&mut bricks[row_index].collision.interaction, "")
                            .on_hover_text("Interaction");
                    });
                });
                row.col(|ui| {
                    ui.add(DragValue::new(&mut bricks[row_index].owner_index));
                });
                row.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut bricks[row_index].material_index));
                        ui.add(DragValue::new(&mut bricks[row_index].material_intensity));
                    });
                });
                row.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut bricks[row_index].position.0));
                        ui.add(DragValue::new(&mut bricks[row_index].position.1));
                        ui.add(DragValue::new(&mut bricks[row_index].position.2));
                    });
                });
                row.col(|ui| match &mut bricks[row_index].size {
                    brickadia::save::Size::Procedural(x, y, z) => {
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(x));
                            ui.add(DragValue::new(y));
                            ui.add(DragValue::new(z));
                        });
                    }
                    brickadia::save::Size::Empty => {
                        if gui::button(ui, "Add Size", true) {
                            bricks[row_index].size = brickadia::save::Size::Procedural(0, 0, 0);
                        }
                    }
                });
            });
        });
}
