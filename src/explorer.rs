use brickadia::read::SaveReader;
use std::{fs::File, io::BufReader, path::PathBuf};

use egui::{CollapsingHeader, Context, RichText, ScrollArea, Ui};

use crate::open;
use crate::EditorApp;

impl EditorApp {
    pub fn show_explorer(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.label("EXPLORER");
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(folder_path) = &self.folder_path {
                let path_to_load = self.display_path(&folder_path, ui, true);
                if let Some(path) = path_to_load {
                    if let Ok(file) = File::open(&path) {
                        let reader = BufReader::new(file);
                        if let Ok(mut save_reader) = SaveReader::new(reader) {
                            if let Ok(save_data) = save_reader.read_all() {
                                self.preview_handle = open::load_preview(&save_data, ctx);
                                self.save_data = Some(save_data);
                                self.file_path = Some(path);
                            }
                        }
                    }
                }
            }
        });
        ui.allocate_space([ui.available_width(), 0.0].into());
    }

    fn display_path(&self, path: &PathBuf, ui: &mut Ui, root: bool) -> Option<PathBuf> {
        if path.is_dir() {
            return self.display_dir(path, ui, root);
        } else if path.is_file() {
            return self.display_file(path, ui);
        }
        None
    }

    fn display_dir(&self, path: &PathBuf, ui: &mut Ui, root: bool) -> Option<PathBuf> {
        let mut ret = None;
        if let Some(dir_name) = path.file_name() {
            let dir_name = dir_name.to_string_lossy();

            let text = if root {
                RichText::new(dir_name.to_ascii_uppercase()).strong()
            } else {
                RichText::new(dir_name)
            };

            CollapsingHeader::new(text)
                .default_open(root)
                .show(ui, |ui| {
                    if let Ok(dir_contents) = path.read_dir() {
                        for entry in dir_contents.filter_map(|x| x.ok()) {
                            let potential_path = self.display_path(&entry.path(), ui, false);
                            if potential_path.is_some() {
                                ret = potential_path;
                            }
                        }
                    }
                });
        }
        ret
    }

    fn display_file(&self, path: &PathBuf, ui: &mut Ui) -> Option<PathBuf> {
        if let Some(filename) = path.file_name() {
            let filename = filename.to_string_lossy();
            if let Some(file_extension) = filename.rsplit_once(".") {
                let file_extension = file_extension.1;
                if file_extension == "brs" {
                    let mut text = filename.to_string();

                    if let Some(cur_path) = &self.file_path {
                        if path == cur_path {
                            text = format!("{} 🛠", text);
                        }
                    }

                    if ui.link(text).clicked() {
                        return Some(path.to_path_buf());
                    }
                }
            }
        }
        None
    }
}
