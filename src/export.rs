use std::{
    path::PathBuf,
    io::Cursor,
};
use brickadia::save::Preview;
use crate::EditorApp;

impl EditorApp {
    pub fn export_preview(&self, path: PathBuf) {
        if let Some(save_data) = &self.save_data {
            match &save_data.preview {
                Preview::JPEG(data) | Preview::PNG(data) => {
                    if let Ok(img) = image::io::Reader::new(Cursor::new(data)).with_guessed_format() {
                        if let Ok(img) = img.decode() {
                            let format = match &save_data.preview {
                                Preview::JPEG(_) => image::ImageFormat::Jpeg,
                                Preview::PNG(_) => image::ImageFormat::Png,
                                _ => image::ImageFormat::Png,
                            };
                            let (width, height) = (img.width(), img.height());
                            match image::save_buffer_with_format(
                                path,
                                &img.into_rgba8(),
                                width,
                                height,
                                image::ColorType::Rgba8,
                                format,
                            ) {
                                Err(e) => {
                                    eprintln!("{}", e);
                                },
                                Ok(_) => { },
                            }
                        } else {
                            eprintln!("Couldn't decode image")
                        }
                    } else {
                        eprintln!("Couldn't interpret preview data")
                    }
                },
                _ => { },
            }
        }
    }

    pub fn export_palette(&self, _path: PathBuf) {
        if let Some(_save_data) = &self.save_data {
            
        }
    }
}
