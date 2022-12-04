use brickadia::save::SaveData;
use brickadia::write::SaveWriter;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn save(file_path: &PathBuf, save_data: SaveData) {
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
