use std::path::PathBuf;

use egui::CollapsingHeader;
use egui::RichText;
use egui::ScrollArea;
use itertools::Itertools;

use crate::gui;
use crate::EditorApp;

impl EditorApp {
    pub fn filetree_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.visuals_mut().hyperlink_color = egui::Color32::LIGHT_GRAY;
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(folder_path) = &self.folder_path {
                let path_to_load = node_ui(ui, &folder_path, &self.file_path, true);
                if let Some(path) = path_to_load {
                    self.open(&path, ctx);
                }
            }
            gui::fill_horizontal(ui);
        });
    }
}

fn node_ui(
    ui: &mut egui::Ui,
    node_path: &PathBuf,
    loaded_path: &Option<PathBuf>,
    is_root: bool,
) -> Option<PathBuf> {
    if node_path.is_dir() {
        return directory_ui(ui, node_path, loaded_path, is_root);
    } else if node_path.is_file() {
        return file_ui(ui, node_path, loaded_path);
    }
    None
}

fn directory_ui(
    ui: &mut egui::Ui,
    dir_path: &PathBuf,
    loaded_path: &Option<PathBuf>,
    is_root: bool,
) -> Option<PathBuf> {
    let mut ret = None;
    if let Some(dir_name) = dir_path.file_name() {
        let dir_name = dir_name.to_string_lossy();
        let text = directory_text(&dir_name, is_root);
        CollapsingHeader::new(text)
            .default_open(is_root)
            .show(ui, |ui| {
                if let Ok(dir_contents) = dir_path.read_dir() {
                    for path in dir_contents
                        .filter_map(|x| x.ok())
                        .map(|x| x.path())
                        .sorted_by(|a, b| {
                            if a.is_dir() && b.is_file() {
                                return std::cmp::Ordering::Less;
                            }
                            if a.is_file() && b.is_dir() {
                                return std::cmp::Ordering::Greater;
                            }
                            std::cmp::Ordering::Equal
                        })
                    {
                        let potential_path = node_ui(ui, &path, loaded_path, false);
                        if potential_path.is_some() {
                            ret = potential_path;
                        }
                    }
                }
            });
    }
    ret
}

fn directory_text(name: &str, is_root: bool) -> RichText {
    if is_root {
        RichText::new(name.to_ascii_uppercase()).strong()
    } else {
        RichText::new(name)
    }
}

fn file_ui(
    ui: &mut egui::Ui,
    file_path: &PathBuf,
    loaded_path: &Option<PathBuf>,
) -> Option<PathBuf> {
    let mut ret = None;
    if let Some(filename) = file_path.file_name() {
        let filename = filename.to_string_lossy();
        if let Some(file_extension) = filename.rsplit_once(".") {
            let file_extension = file_extension.1;
            if file_extension == "brs" {
                let text = filename.to_string();
                set_highlight_color(ui, file_path, loaded_path);
                if ui.link(text).clicked() {
                    ret = Some(file_path.to_path_buf());
                }
            }
        }
    }
    ret
}

fn set_highlight_color(ui: &mut egui::Ui, file_path: &PathBuf, loaded_path: &Option<PathBuf>) {
    if let Some(cur_path) = loaded_path {
        if file_path == cur_path {
            ui.visuals_mut().hyperlink_color = egui::Color32::YELLOW;
        } else {
            ui.visuals_mut().hyperlink_color = egui::Color32::LIGHT_GRAY;
        }
    }
}
