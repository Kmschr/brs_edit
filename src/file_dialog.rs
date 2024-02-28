use crate::open;
use crate::EditorApp;
use brickadia::save::Preview;
use egui::Context;
use rfd::FileDialog;
use std::env;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::{sync::mpsc, thread};

impl EditorApp {
    pub fn receive_file_dialog_paths(&mut self, ctx: &Context) {
        self.receive_file_path(ctx);
        self.receive_folder_path();
        self.receive_preview_path(ctx);
        self.receive_save_as_path();
        self.receive_export_preview_path();
        self.receive_export_json_path();
        self.receive_export_palette_path();
        self.receive_import_json_path();
    }

    fn receive_file_path(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.receivers.file_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.receivers.file_path_receiver = None;
                self.receivers.num_active -= 1;
                if let Some(path) = data {
                    self.open(&path, ctx);
                }
            }
        }
    }

    fn receive_folder_path(&mut self) {
        if let Some(rx) = &self.receivers.folder_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.receivers.folder_path_receiver = None;
                self.receivers.num_active -= 1;
                self.folder_path = data;
            }
        }
    }

    fn receive_preview_path(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.receivers.preview_path_receiver {
            if let Ok(data) = rx.try_recv() {
                println!("Preview path receiver has data\n {:?}", data);
                self.receivers.preview_path_receiver = None;
                self.receivers.num_active -= 1;
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

    fn receive_save_as_path(&mut self) {
        if let Some(rx) = &self.receivers.save_as_path_receiever {
            if let Ok(data) = rx.try_recv() {
                self.receivers.save_as_path_receiever = None;
                self.receivers.num_active -= 1;
                if let Some(file_path) = data {
                    self.save_as(&file_path);
                }
            }
        }
    }

    fn receive_export_preview_path(&mut self) {
        if let Some(rx) = &self.receivers.export_preview_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.receivers.export_preview_path_receiver = None;
                self.receivers.num_active -= 1;
                if let Some(file_path) = data {
                    self.export_preview(file_path);
                }
            }
        }
    }

    fn receive_export_json_path(&mut self) {
        if let Some(rx) = &self.receivers.export_json_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.receivers.export_json_path_receiver = None;
                self.receivers.num_active -= 1;
                if let Some(file_path) = data {
                    self.export_json(file_path);
                }
            }
        }
    }

    fn receive_export_palette_path(&mut self) {
        if let Some(rx) = &self.receivers.export_palette_path_reciever {
            if let Ok(data) = rx.try_recv() {
                self.receivers.export_palette_path_reciever = None;
                self.receivers.num_active -= 1;
                if let Some(file_path) = data {
                    self.export_palette(file_path);
                }
            }
        }
    }

    fn receive_import_json_path(&mut self) {
        if let Some(rx) = &self.receivers.import_json_path_receiver {
            if let Ok(data) = rx.try_recv() {
                self.receivers.import_json_path_receiver = None;
                self.receivers.num_active -= 1;
                if let Some(file_path) = data {
                    self.import_json(file_path);
                }
            }
        }
    }

    pub fn choose_file(&mut self) {
        if self.receivers.file_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.receivers.file_path_receiver = Some(rx);
            self.receivers.num_active += 1;
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
        if self.receivers.folder_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.receivers.folder_path_receiver = Some(rx);
            self.receivers.num_active += 1;
            thread::spawn(move || {
                let folder = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .pick_folder();
                tx.send(folder).unwrap();
            });
        }
    }

    pub fn choose_save_as(&mut self) {
        if self.receivers.save_as_path_receiever.is_none() {
            let (tx, rx) = mpsc::channel();
            self.receivers.save_as_path_receiever = Some(rx);
            self.receivers.num_active += 1;
            thread::spawn(move || {
                let file = FileDialog::new()
                    .set_directory("%USERPROFILE%/AppData")
                    .add_filter("Brickadia Savefile", &["brs"])
                    .save_file();
                tx.send(file).unwrap();
            });
        }
    }

    pub fn choose_export_preview(&mut self) {
        if self.receivers.export_preview_path_receiver.is_none() {
            if let Some(save_data) = &self.save_data {
                let extensions = match save_data.preview {
                    Preview::JPEG(_) => vec!["jpg", "jpeg"],
                    Preview::PNG(_) => vec!["png"],
                    _ => vec![],
                };
                let (tx, rx) = mpsc::channel();
                self.receivers.export_preview_path_receiver = Some(rx);
                self.receivers.num_active += 1;
                thread::spawn(move || {
                    let file = FileDialog::new()
                        .add_filter("Image", &extensions)
                        .save_file();
                    tx.send(file).unwrap();
                });
            }
        }
    }

    pub fn choose_export_json(&mut self) {
        if self.receivers.export_json_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.receivers.export_json_path_receiver = Some(rx);
            self.receivers.num_active += 1;
            thread::spawn(move || {
                let file = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .save_file();
                tx.send(file).unwrap();
            });
        }
    }

    pub fn choose_import_json(&mut self) {
        if self.receivers.import_json_path_receiver.is_none() {
            let (tx, rx) = mpsc::channel();
            self.receivers.import_json_path_receiver = Some(rx);
            self.receivers.num_active += 1;
            thread::spawn(move || {
                let file = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .pick_file();
                tx.send(file).unwrap();
            });
        }
    }

    pub fn _choose_export_palette(&mut self) {
        if self.receivers.export_palette_path_reciever.is_none() {
            if let Some(save_data) = &self.save_data {
                if save_data.header2.colors.is_empty() {
                    return;
                }
                let (tx, rx) = mpsc::channel();
                self.receivers.export_palette_path_reciever = Some(rx);
                self.receivers.num_active += 1;
                thread::spawn(move || {
                    let file = FileDialog::new()
                        .add_filter("Brickadia Color Palette", &["bp"])
                        .save_file();
                    tx.send(file).unwrap();
                });
            }
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
