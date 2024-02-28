#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod delete;
mod editor;
mod explorer;
mod export;
mod file_dialog;
mod gui;
mod icon;
mod import;
mod menu;
mod open;
mod save;
mod shortcuts;
mod view;

use eframe::egui;
use egui::*;
use file_dialog::default_build_directory;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

const DEFAULT_WINDOW_SIZE: Vec2 = Vec2::new(1600.0, 900.0);

fn main() {
    let icon = egui::IconData {
        rgba: icon::ICON.to_vec(),
        width: 32,
        height: 32,
    };
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(DEFAULT_WINDOW_SIZE)
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "BRS Editor",
        native_options,
        Box::new(|cc| Box::new(EditorApp::new(cc))),
    ).unwrap();
}

struct EditorApp {
    default_build_dir: Option<PathBuf>,
    default_documents_dir: Option<PathBuf>,
    default_downloads_dir: Option<PathBuf>,
    file_path: Option<PathBuf>,
    folder_path: Option<PathBuf>,
    receivers: Receivers,
    save_data: Option<brickadia::save::SaveData>,
    save_colors: Vec<([f32; 4], u32)>,
    preview_handle: Option<TextureHandle>,
    show_delete_window: bool,
    _show_components_window: bool,
    _show_ownership_window: bool,
}

// use channels to get paths back from the native file dialog in another thread
#[derive(Default)]
struct Receivers {
    num_active: usize,
    // Open File
    file_path_receiver: Option<Receiver<Option<PathBuf>>>,
    // Open Folder
    folder_path_receiver: Option<Receiver<Option<PathBuf>>>,
    // Choose Image...
    preview_path_receiver: Option<Receiver<Option<PathBuf>>>,
    // Save As...
    save_as_path_receiever: Option<Receiver<Option<PathBuf>>>,
    // Export > Preview
    export_preview_path_receiver: Option<Receiver<Option<PathBuf>>>,
    // Export > JSON
    export_json_path_receiver: Option<Receiver<Option<PathBuf>>>,
    // Export > Palette
    export_palette_path_reciever: Option<Receiver<Option<PathBuf>>>,
    // Import > JSON
    import_json_path_receiver: Option<Receiver<Option<PathBuf>>>,
}

impl EditorApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            default_build_dir: default_build_directory(),
            default_documents_dir: dirs::document_dir(),
            default_downloads_dir: dirs::download_dir(),
            file_path: None,
            folder_path: None,
            receivers: Receivers::default(),
            save_data: None,
            save_colors: vec![],
            preview_handle: None,
            show_delete_window: false,
            _show_components_window: false,
            _show_ownership_window: false,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);
        if self.receivers.num_active > 0 {
            self.receive_file_dialog_paths(ctx);
        }
        TopBottomPanel::top("menu_panel")
            .frame(gui::TOP_FRAME)
            .show(ctx, |ui| {
                self.show_menu(ui, ctx);
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
                self.explorer_ui(ui, ctx);
            });
        CentralPanel::default()
            .frame(gui::CENTER_FRAME)
            .show(ctx, |ui| {
                if self.file_path.is_none() && self.save_data.is_none() {
                    self.starting_page(ui);
                } else if self.save_data.is_some() {
                    ScrollArea::vertical().stick_to_right(true).show(ui, |ui| {
                        self.editor_ui(ui);
                    });
                }
                if self.show_delete_window {
                    self.delete_ui(ctx);
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
                let github_icon = RichText::new("\u{e624}").strong();
                ui.visuals_mut().hyperlink_color = Color32::WHITE;
                ui.hyperlink_to(github_icon, "https://github.com/Kmschr/brs_edit");
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
        if let Some(default_documents_dir) = &self.default_documents_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Documents").clicked() {
                    self.folder_path = Some(default_documents_dir.clone());
                };
                ui.strong(default_documents_dir.to_string_lossy());
            });
        }
        if let Some(default_downloads_dir) = &self.default_downloads_dir {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                if ui.link("Downloads").clicked() {
                    self.folder_path = Some(default_downloads_dir.clone());
                };
                ui.strong(default_downloads_dir.to_string_lossy());
            });
        }
    }
}
