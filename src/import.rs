use std::path::PathBuf;
use brickadia::save::SaveData;

use crate::{open::{count_colors, load_colors}, EditorApp};

impl EditorApp {
    pub fn import_json(&mut self, path: PathBuf) {
        if let Ok(json) = std::fs::read_to_string(&path) {
            if let Ok(save_data) = serde_json::from_str::<SaveData>(&json) {
                load_colors(&mut self.save_colors, &save_data.header2.colors);
                count_colors(&mut self.save_colors, &save_data.bricks);

                self.save_data = Some(save_data);
                self.file_path = None;
                self.preview_handle = None;
            } else {
                eprintln!("Couldn't deserialize save data from JSON");
            }
        } else {
            eprintln!("Couldn't read JSON file");
        }
    }
}
