use brickadia::read::SaveReader;
use egui::Color32;
use itertools::Itertools;
use std::{fs::File, io::BufReader, path::PathBuf};

use egui::{CollapsingHeader, Context, RichText, ScrollArea, Ui};

use crate::gui;
use crate::open;
use crate::EditorApp;

const PREVIEW_HEIGHT: f32 = 100.0;

impl EditorApp {
    pub fn show_explorer(&mut self, ui: &mut Ui, ctx: &Context) {
        if let Some(texture) = &self.preview_handle {
            ui.vertical_centered(|ui| {
                let preview_size = texture.size_vec2();
                let preview_aspect_ratio = preview_size.x / preview_size.y;
                ui.image(
                    texture,
                    [PREVIEW_HEIGHT * preview_aspect_ratio, PREVIEW_HEIGHT],
                );
            });
            ui.separator();
        }

        ui.label("   EXPLORER");
        ui.add_space(5.0);
        ui.visuals_mut().hyperlink_color = Color32::LIGHT_GRAY;
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
            gui::fill_horizontal(ui);
        });
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
                        for path in dir_contents
                            .filter_map(|x| x.ok())
                            .map(|x| x.path())
                            .sorted_by(|a, b| {
                                if a.is_dir() && b.is_file() {
                                    return std::cmp::Ordering::Less;
                                } else if a.is_file() && b.is_dir() {
                                    return std::cmp::Ordering::Greater;
                                }
                                std::cmp::Ordering::Equal
                            })
                        {
                            let potential_path = self.display_path(&path, ui, false);
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
        let mut ret = None;
        if let Some(filename) = path.file_name() {
            let filename = filename.to_string_lossy();
            if let Some(file_extension) = filename.rsplit_once(".") {
                let file_extension = file_extension.1;
                if file_extension == "brs" {
                    let text = filename.to_string();

                    if let Some(cur_path) = &self.file_path {
                        if path == cur_path {
                            ui.visuals_mut().hyperlink_color = Color32::YELLOW;
                        }
                    }

                    if ui.link(text).clicked() {
                        ret = Some(path.to_path_buf());
                    }
                    ui.visuals_mut().hyperlink_color = Color32::LIGHT_GRAY;
                }
            }
        }
        ret
    }
}
