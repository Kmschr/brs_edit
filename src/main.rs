#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod explorer;
mod file_dialog;
mod gui;
mod icon;
mod input;
mod menu;
mod open;
mod save;
mod view;
mod header2;

use brickadia::save::SaveData;
use eframe::egui;
use egui::*;
use file_dialog::default_build_directory;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use num_format::{Locale, ToFormattedString};
use header2::show_header_two;

const DEFAULT_WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

const MAX_PREVIEW_WIDTH: f32 = 640.0;
const MAX_PREVIEW_HEIGHT: f32 = 360.0;
const MAX_PREVIEW_ASPECT_RATIO: f32 = MAX_PREVIEW_WIDTH / MAX_PREVIEW_HEIGHT;

// TODO:
// - See if a file was modified
// - Shortcuts for saving
// - Save As
// - Render preview (GLOW) ?
// - Import > BLS
// - Import > OBJ
// - Import > io (Bricklink Studio)

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(DEFAULT_WINDOW_SIZE),
        icon_data: Some(eframe::IconData {
            rgba: icon::ICON.to_vec(),
            width: 32,
            height: 32,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "BRS Editor",
        native_options,
        Box::new(|cc| Box::new(EditorApp::new(cc))),
    )
}

struct EditorApp {
    default_build_dir: Option<PathBuf>,
    file_path_receiver: Option<Receiver<Option<PathBuf>>>,
    file_path: Option<PathBuf>,
    folder_path_receiver: Option<Receiver<Option<PathBuf>>>,
    folder_path: Option<PathBuf>,
    save_data: Option<brickadia::save::SaveData>,
    save_colors: Vec<([f32; 4], u32)>,
    preview_handle: Option<TextureHandle>,
    preview_path_receiver: Option<Receiver<Option<PathBuf>>>,
}

impl EditorApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            default_build_dir: default_build_directory(),
            file_path_receiver: None,
            file_path: None,
            folder_path_receiver: None,
            folder_path: None,
            save_data: None,
            save_colors: vec![],
            preview_handle: None,
            preview_path_receiver: None,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx, frame);
        self.receive_file_dialog_paths(ctx);

        TopBottomPanel::top("menu_panel").frame(gui::TOP_FRAME).show(ctx, |ui| {
            self.show_menu(ui, ctx, frame);
        });
        TopBottomPanel::bottom("info_panel")
            .frame(gui::BOTTOM_FRAME)
            .show(ctx, |ui| {
                self.bottom_panel(ui);
            });
        SidePanel::left("file_panel")
            .resizable(true)
            .frame(gui::LEFT_FRAME)
            .max_width(DEFAULT_WINDOW_SIZE.x / 2.0)
            .show(ctx, |ui| {
                self.show_explorer(ui, ctx);
            });
        CentralPanel::default().frame(gui::CENTER_FRAME).show(ctx, |ui| {
            if self.file_path.is_none() {
                self.starting_page(ui);
            } else if self.save_data.is_some() {
                ScrollArea::vertical().stick_to_right(true).show(ui, |ui| {
                    self.save_page(ui);
                    gui::fill_horizontal(ui);
                });
            }
        });
    }
}

impl EditorApp {
    fn bottom_panel(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(file_path) = &self.file_path {
                ui.strong(file_path.to_string_lossy());
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                ui.add_space(5.0);
                let icon = RichText::new("\u{e624}").strong();
                ui.visuals_mut().hyperlink_color = Color32::WHITE;
                ui.hyperlink_to(icon, "https://github.com/Kmschr/brs_edit");
            });
        });
    }

    fn starting_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("BRS Editor");
        gui::header(ui, "Start");
        ui.add_space(5.0);
        if ui.link("🗋 Open File...").clicked() {
            self.choose_file();
        }
        ui.add_space(5.0);
        if ui.link("🗁 Open Folder...").clicked() {
            self.choose_folder();
        }
        ui.add_space(5.0);
        gui::header(ui, "Common Places");
        ui.add_space(5.0);
        if let Some(default_build_dir) = &self.default_build_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Builds").clicked() {
                    self.folder_path = Some(default_build_dir.clone());
                };
                ui.strong(default_build_dir.to_string_lossy());
            });
        }

        ui.label("Documents");
        ui.label("Downloads");
    }

    fn save_page(&mut self, ui: &mut egui::Ui) {
        if let Some(save_data) = &mut self.save_data {
            ui.visuals_mut().override_text_color = Some(Color32::WHITE);
            if let Some(style) = ui.style_mut().text_styles.get_mut(&TextStyle::Button) {
                style.size = 25.0;
            }
            show_metadata(save_data, ui);
            show_header_one(save_data, ui);
            show_header_two(save_data, &mut self.save_colors, ui);
            let new_preview_receiver = show_preview(&self.preview_handle, ui);
            if new_preview_receiver.is_some() {
                self.preview_path_receiver = new_preview_receiver;
            }
        }
    }
}

fn show_metadata(save_data: &mut SaveData, ui: &mut egui::Ui) {
    CollapsingHeader::new("Metadata").default_open(true).show(ui, |ui| {
        ui.visuals_mut().override_text_color = None;

        ui.add_space(10.0);
        ui.strong("BRS Version");
        ui.label(
            "The file format version used for this save. Alpha 5 uses version 10. Can not be changed.",
        );
        ui.add_enabled(false, DragValue::new(&mut save_data.version));
        ui.add_space(5.0);
        ui.strong("Game Version");
        ui.label("Also known as \"Commit Level\" and corresponds to each change tracked by developers. Alpha 5 is currently using CL7870 as seen in the top right of the game. This field was introduced in BRS version 8");
        ui.add(DragValue::new(&mut save_data.game_version));
        ui.add_space(5.0);
    });
}

fn show_header_one(save_data: &mut SaveData, ui: &mut egui::Ui) {
    CollapsingHeader::new("Header1")
        .default_open(true)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = None;

            ui.add_space(5.0);
            ui.strong("Map");
            ui.label("Which game environment the save was generated in.");
            gui::text_edit_singleline(ui, &mut save_data.header1.map);

            ui.add_space(5.0);

            ui.strong("Description");
            gui::text_edit_multiline(ui, &mut save_data.header1.description);

            ui.add_space(5.0);

            ui.strong("Author: Name");
            ui.label("Who created this save file, not always the builder of the save.");
            gui::text_edit_singleline(ui, &mut save_data.header1.author.name);

            ui.add_space(5.0);

            ui.strong("Author: ID");
            ui.label("Player ID of who created this save file, not always the builder of the save.");
            ui.code(save_data.header1.author.id.to_string());

            ui.add_space(5.0);

            ui.strong("Brickcount");
            ui.add_enabled(false, DragValue::new(&mut save_data.header1.brick_count).custom_formatter(|n, _| (n as i32).to_formatted_string(&Locale::en)).suffix(" bricks"));
            ui.add_space(5.0);
        });
}

fn show_preview(
    preview: &Option<TextureHandle>,
    ui: &mut egui::Ui,
) -> Option<Receiver<Option<PathBuf>>> {
    let mut ret = None;

    CollapsingHeader::new("Preview")
    .default_open(true)
    .show(ui, |ui| {
        ui.visuals_mut().override_text_color = None;
        if let Some(style) = ui.style_mut().text_styles.get_mut(&TextStyle::Button) {
            style.size = 14.0;
        }

        ui.add_space(5.0);
        ui.label("Brickadia stores preview images as PNG or JPEG images. If you select an image not in one of these formats it will convert it to PNG");
        ui.add_space(5.0);
        if gui::button(ui, "Choose Image...", true) {
            ret = Some(file_dialog::choose_preview());
        }
        if let Some(texture) = preview {
            let preview_size = texture.size_vec2();
            ui.label(format!("{} x {}", preview_size.x, preview_size.y));
    
            let display_size = contain_preview_size(preview_size);
            Frame::none()
                .shadow(epaint::Shadow::big_dark())
                .show(ui, |ui| {
                    ui.image(texture, display_size);
                });
        }

        ui.add_space(5.0);
    });

    ret
}

fn contain_preview_size(preview_size: Vec2) -> Vec2 {
    let preview_aspect_ratio = preview_size.x / preview_size.y;
    if preview_aspect_ratio > MAX_PREVIEW_ASPECT_RATIO {
        [
            MAX_PREVIEW_HEIGHT * preview_aspect_ratio,
            MAX_PREVIEW_HEIGHT,
        ]
        .into()
    } else {
        [MAX_PREVIEW_WIDTH, MAX_PREVIEW_WIDTH / preview_aspect_ratio].into()
    }
}
