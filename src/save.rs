use brickadia::save::SaveData;
use brickadia::write::SaveWriter;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::mem;
use std::path::PathBuf;

use crate::EditorApp;

impl EditorApp {
    pub fn save(&mut self, ctx: &egui::Context) {
        if let Some(file_path) = &self.file_path.clone() {
            if self.save_data.is_some() {
                let save_data = mem::take(&mut self.save_data);
                if let Some(save_data) = save_data {
                    save_to_path(file_path, save_data);
                    self.open(file_path, ctx);
                }
            }
        }
    }
}

fn save_to_path(file_path: &PathBuf, save_data: SaveData) {
    if let Ok(file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
    {
        let buffered_writer = BufWriter::new(file);
        let save_writer = SaveWriter::new(buffered_writer, save_data);
        save_writer.write().unwrap();
    }
}
