use std::env;
use std::path::PathBuf;
use std::{fs::File, io::BufReader, sync::mpsc, thread};

use crate::open;
use crate::EditorApp;
use brickadia::read::SaveReader;
use egui::Context;
use rfd::FileDialog;

impl EditorApp {
    pub fn receive_file_dialog_paths(&mut self, ctx: &Context) {
        if let Some(rx) = &self.file_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.file_path_receiver = None;
                if let Some(path) = data {
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
        }

        if let Some(rx) = &self.folder_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.folder_path_receiver = None;
                self.folder_path = data;
            }
        }
    }

    pub fn choose_file(&mut self) {
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

    pub fn choose_folder(&mut self) {
        if self.folder_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.folder_path_receiver = Some(rx);

            thread::spawn(move || {
                let files = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .pick_folder();
                tx.send(files).unwrap();
            });
        }
    }
}

pub fn default_build_directory() -> Option<PathBuf> {
    match env::consts::OS {
        "windows" => dirs::data_local_dir().and_then(|path| {
            Some(PathBuf::from(
                path.to_string_lossy().to_string() + "\\Brickadia\\Saved\\Builds",
            ))
        }),
        "linux" => dirs::config_dir().and_then(|path| {
            Some(PathBuf::from(
                path.to_string_lossy().to_string() + "/Epic/Brickadia/Saved/Builds",
            ))
        }),
        _ => None,
    }
}
