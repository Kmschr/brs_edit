use brickadia::save::{
    Color,
    SaveData,
};
use brickadia::write::SaveWriter;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;
use crate::EditorApp;

impl EditorApp {
    pub fn save(&mut self) {
        if let Some(file_path) = &self.file_path.clone() {
            if self.save_data.is_some() {
                let save_data = self.save_data.clone();
                if let Some(mut save_data) = save_data {
                    save_colors(&self.save_colors, &mut save_data.header2.colors);
                    save_to_path(file_path, save_data);
                }
            }
        }
    }

    pub fn save_as(&mut self, file_path: &PathBuf) {
        if self.save_data.is_some() {
            let save_data = self.save_data.clone();
            if let Some(mut save_data) = save_data {
                save_colors(&self.save_colors, &mut save_data.header2.colors);
                save_to_path(file_path, save_data);
                self.file_path = Some(file_path.to_path_buf());
            }
        }
    }
}

fn save_colors(egui_colors: &Vec<([f32; 4], u32)>, brs_colors: &mut Vec<Color>) {
    brs_colors.clear();
    for (egui_color, _) in egui_colors {
        brs_colors.push(
            Color::from_bytes_bgra(
                [
                    (egui_color[2] * 255.0) as u8,
                    (egui_color[1] * 255.0) as u8,
                    (egui_color[0] * 255.0) as u8,
                    (egui_color[3] * 255.0) as u8,
                ],
            ),
        );
    }
}

fn save_to_path(file_path: &PathBuf, save_data: SaveData) {
    if let Ok(file) = OpenOptions::new().read(true).write(true).create(true).open(file_path) {
        let buffered_writer = BufWriter::new(file);
        let save_writer = SaveWriter::new(buffered_writer, save_data);
        match save_writer.write() {
            Ok(_) => println!("Wrote save successfully to \"{}\"", &file_path.to_string_lossy()),
            Err(e) => eprintln!("Failed to write save file: {}", e),
        }
    }
}
