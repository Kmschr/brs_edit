#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod gui;

use brickadia::read::SaveReader;
use brickadia::save::{Preview, SaveData};
use eframe::egui;
use egui::*;
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use std::sync::{mpsc, mpsc::Receiver};
use std::thread;

const DEFAULT_WINDOW_SIZE: Vec2 = Vec2::new(900.0, 720.0);

const MAX_PREVIEW_WIDTH: f32 = 640.0;
const MAX_PREVIEW_HEIGHT: f32 = 360.0;
const MAX_PREVIEW_ASPECT_RATIO: f32 = MAX_PREVIEW_WIDTH / MAX_PREVIEW_HEIGHT;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(DEFAULT_WINDOW_SIZE),
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
    _folder_path_receiver: Option<Receiver<Option<PathBuf>>>,
    _folder_path: Option<PathBuf>,
    save_data: Option<brickadia::save::SaveData>,
    preview_handle: Option<TextureHandle>,
}

impl EditorApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_path_receiver: None,
            file_path: None,
            _folder_path_receiver: None,
            _folder_path: None,
            save_data: None,
            preview_handle: None,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.receive_file_dialog_paths(ctx);
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            self.show_menu(ui);
        });
        egui::TopBottomPanel::bottom("info_panel").show(ctx, |ui| {
            self.bottom_panel(ui);
        });
        egui::SidePanel::left("file_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("EXPLORER");
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.file_path.is_none() {
                self.starting_page(ui);
            } else if self.save_data.is_some() {
                ScrollArea::vertical()
                    .always_show_scroll(true)
                    .stick_to_right(true)
                    .show(ui, |ui| {
                        self.save_page(ui);
                    });
            }
        });
    }
}

impl EditorApp {
    fn receive_file_dialog_paths(&mut self, ctx: &Context) {
        if let Some(rx) = &self.file_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.file_path_receiver = None;
                if let Some(path) = data {
                    if let Ok(file) = File::open(&path) {
                        let reader = BufReader::new(file);
                        if let Ok(mut save_reader) = SaveReader::new(reader) {
                            if let Ok(save_data) = save_reader.read_all() {
                                self.save_data = Some(save_data);
                                self.file_path = Some(path);
                                self.load_preview(ctx);
                            }
                        }
                    }
                }
            }
        }
    }

    fn show_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {
                    self.choose_file();
                }
                ui.separator();
                if ui.button("Save").clicked() {}
                if ui.button("Save As...").clicked() {}
                ui.separator();
                if ui.button("Import").clicked() {}
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |ui| if ui.button("Open").clicked() {});
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    }

    fn bottom_panel(&self, ui: &mut egui::Ui) {
        if let Some(file_path) = &self.file_path {
            // if let Some(filename) = file_path.file_name() {
            //     ui.label(filename.to_string_lossy());
            // }
            ui.label(file_path.to_string_lossy());
        }
    }

    fn starting_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("BRS Editor");
        ui.label("Start");
        if ui.link("🗋 Open File...").clicked() {
            self.choose_file();
        }
    }

    fn save_page(&mut self, ui: &mut egui::Ui) {
        if let Some(save_data) = &mut self.save_data {
            show_metadata(save_data, ui);
            show_header_one(save_data, ui);
            show_preview(&self.preview_handle, ui);
        }
    }

    fn choose_file(&mut self) {
        if self.file_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.file_path_receiver = Some(rx);

            thread::spawn(move || {
                let files = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .add_filter("Brickadia Savefile", &["brs", "BRS"])
                    .pick_file();
                tx.send(files).unwrap();
            });
        }
    }

    fn load_preview(&mut self, ctx: &Context) {
        self.preview_handle = None;
        if let Some(save_data) = &self.save_data {
            match &save_data.preview {
                Preview::JPEG(data) | Preview::PNG(data) => {
                    if let Ok(img) = image::io::Reader::new(Cursor::new(data)).with_guessed_format()
                    {
                        if let Ok(img) = img.decode() {
                            let img_size = [img.width() as _, img.height() as _];
                            let img_rgba8 = img.to_rgba8();
                            let img_pixels = img_rgba8.as_flat_samples();
                            let img =
                                ColorImage::from_rgba_unmultiplied(img_size, img_pixels.as_slice());
                            self.preview_handle =
                                Some(ctx.load_texture("Save-Preview", img, TextureFilter::Nearest));
                        } else {
                            println!("Couldn't decode image")
                        }
                    } else {
                        println!("Couldn't interpret preview data")
                    }
                }
                Preview::None => {
                    println!("No preview image to load")
                }
                _ => {
                    println!("Preview image in unknown format")
                }
            }
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
