#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod explorer;
mod file_dialog;
mod gui;
mod icon;
mod menu;
mod open;

use brickadia::save::SaveData;
use eframe::egui;
use egui::*;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

const DEFAULT_WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

const MAX_PREVIEW_WIDTH: f32 = 640.0;
const MAX_PREVIEW_HEIGHT: f32 = 360.0;
const MAX_PREVIEW_ASPECT_RATIO: f32 = MAX_PREVIEW_WIDTH / MAX_PREVIEW_HEIGHT;

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
    file_path_receiver: Option<Receiver<Option<PathBuf>>>,
    file_path: Option<PathBuf>,
    folder_path_receiver: Option<Receiver<Option<PathBuf>>>,
    folder_path: Option<PathBuf>,
    save_data: Option<brickadia::save::SaveData>,
    preview_handle: Option<TextureHandle>,
}

impl EditorApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_path_receiver: None,
            file_path: None,
            folder_path_receiver: None,
            folder_path: None,
            save_data: None,
            preview_handle: None,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if ctx.input().key_pressed(Key::F11) {
            if frame.info().window_info.fullscreen {
                frame.set_fullscreen(false);
            } else {
                frame.set_fullscreen(true);
            }
        }
        self.receive_file_dialog_paths(ctx);
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            self.show_menu(ui);
        });
        TopBottomPanel::bottom("info_panel")
            .frame(gui::BOTTOM_FRAME)
            .show(ctx, |ui| {
                self.bottom_panel(ui);
            });
        SidePanel::left("file_panel")
            .resizable(true)
            .max_width(DEFAULT_WINDOW_SIZE.x / 2.0)
            .show(ctx, |ui| {
                self.show_explorer(ui, ctx);
            });
        CentralPanel::default().show(ctx, |ui| {
            if self.file_path.is_none() {
                self.starting_page(ui);
            } else if self.save_data.is_some() {
                ScrollArea::vertical().stick_to_right(true).show(ui, |ui| {
                    self.save_page(ui);
                    ui.allocate_space([ui.available_width(), 0.0].into());
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
    }

    fn save_page(&mut self, ui: &mut egui::Ui) {
        if let Some(save_data) = &mut self.save_data {
            show_metadata(save_data, ui);
            show_header_one(save_data, ui);
            show_preview(&self.preview_handle, ui);
        }
    }
}

fn show_metadata(save_data: &mut SaveData, ui: &mut egui::Ui) {
    gui::header(ui, "Metadata");
    ui.add_space(10.0);
    ui.strong("BRS Version");
    ui.label(
        "The file format version used for this save. Alpha 5 uses version 10. Can not be changed.",
    );
    ui.add_enabled(false, DragValue::new(&mut save_data.version));
    ui.add_space(5.0);
    ui.strong("Game Version");
    ui.label("Also known as \"Commit Level\" and corresponds to each change tracked by developers. Alpha 5 is currently using CL7870 as seen in the top right of the game.");
    ui.add(DragValue::new(&mut save_data.game_version));
}

fn show_header_one(save_data: &mut SaveData, ui: &mut egui::Ui) {
    ui.add_space(15.0);
    gui::header(ui, "Header1");
    ui.add_space(5.0);
    ui.strong("Author");
    ui.label("Who created this save file, not always the builder of the save.");
    ui.text_edit_singleline(&mut save_data.header1.author.name);
    ui.strong("Description");
    ui.add(TextEdit::multiline(&mut save_data.header1.description).desired_width(600.0));
    ui.strong("Brickcount");
    ui.add(DragValue::new(&mut save_data.header1.brick_count));
}

fn show_preview(preview: &Option<TextureHandle>, ui: &mut egui::Ui) {
    ui.add_space(15.0);
    gui::header(ui, "Preview");
    ui.add_space(5.0);
    if gui::button(ui, "Choose Image...", true) {
        // todo: change preview
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
