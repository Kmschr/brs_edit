use std::env;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::{sync::mpsc, thread};

use crate::open;
use crate::EditorApp;
use brickadia::save::Preview;
use egui::Context;
use rfd::FileDialog;

impl EditorApp {
    pub fn receive_file_dialog_paths(&mut self, ctx: &Context) {
        self.receive_file_path(ctx);
        self.receive_folder_path();
        self.receive_preview_path(ctx);
    }

    fn receive_file_path(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.file_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.file_path_receiver = None;
                if let Some(path) = data {
                    self.open(&path, ctx);
                }
            }
        }
    }

    fn receive_folder_path(&mut self) {
        if let Some(rx) = &self.folder_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.folder_path_receiver = None;
                self.folder_path = data;
            }
        }
    }

    fn receive_preview_path(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.preview_path_receiver {
            if let Ok(data) = rx.try_recv() {
                println!("Preview path receiver has data\n {:?}", data);
                self.preview_path_receiver = None;
                if let Some(save_data) = &mut self.save_data {
                    if let Some(path) = data {
                        if let Ok(buffer) = std::fs::read(path) {
                            if let Ok(format) = image::guess_format(&buffer) {
                                match format {
                                    image::ImageFormat::Png => {
                                        println!("Setting PNG data");
                                        save_data.preview = Preview::PNG(buffer.to_vec());
                                    }
                                    image::ImageFormat::Jpeg => {
                                        println!("Setting JPEF data");
                                        save_data.preview = Preview::JPEG(buffer.to_vec());
                                    }
                                    _ => {
                                        println!("Setting other format data");
                                        if let Ok(img) = image::load(Cursor::new(buffer), format) {
                                            let mut buf = Cursor::new(Vec::new());
                                            if img
                                                .write_to(&mut buf, image::ImageFormat::Png)
                                                .is_ok()
                                            {
                                                save_data.preview = Preview::PNG(buf.into_inner());
                                            }
                                        }
                                    }
                                }
                                self.preview_handle = open::load_preview(&save_data, ctx);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn choose_file(&mut self) {
        if self.file_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.file_path_receiver = Some(rx);

            thread::spawn(move || {
                let file = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .add_filter("Brickadia Savefile", &["brs", "BRS"])
                    .pick_file();
                tx.send(file).unwrap();
            });
        }
    }

    pub fn choose_folder(&mut self) {
        if self.folder_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.folder_path_receiver = Some(rx);

            thread::spawn(move || {
                let folder = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .pick_folder();
                tx.send(folder).unwrap();
            });
        }
    }
}

pub fn choose_preview() -> Receiver<Option<PathBuf>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let file = FileDialog::new()
            .add_filter(
                "Image",
                &["png", "jpg", "jpeg", "tiff", "gif", "bmp", "ico", "webp"],
            )
            .pick_file();
        tx.send(file).unwrap();
    });
    rx
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
