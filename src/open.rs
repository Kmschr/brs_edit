use std::{io::Cursor, path::PathBuf};

use brickadia::{
    read::SaveReader,
    save::{Brick, BrickColor, Color, Preview, SaveData},
};
use egui::{ColorImage, Context, TextureHandle, TextureOptions};

use crate::EditorApp;

impl EditorApp {
    pub fn open(&mut self, file_path: &PathBuf, ctx: &egui::Context) {
        if let Ok(file) = std::fs::File::open(&file_path) {
            let reader = std::io::BufReader::new(file);
            if let Ok(mut save_reader) = SaveReader::new(reader) {
                if let Ok(save_data) = save_reader.read_all() {
                    self.preview_handle = load_preview(&save_data, ctx);
                    load_colors(&mut self.save_colors, &save_data.header2.colors);
                    count_colors(&mut self.save_colors, &save_data.bricks);
                    self.save_data = Some(save_data);
                    self.file_path = Some(file_path.to_path_buf());
                }
            }
        }
    }
}

pub fn load_colors(egui_colors: &mut Vec<([f32; 4], u32)>, brs_colors: &Vec<Color>) {
    egui_colors.clear();
    for brs_color in brs_colors {
        egui_colors.push((
            [
                (brs_color.r as f32 / 255.0),
                (brs_color.g as f32 / 255.0),
                (brs_color.b as f32 / 255.0),
                (brs_color.a as f32 / 255.0),
            ],
            0,
        ));
    }
}

pub fn count_colors(egui_colors: &mut Vec<([f32; 4], u32)>, bricks: &Vec<Brick>) {
    for brick in bricks {
        match brick.color {
            BrickColor::Index(i) => {
                egui_colors[i as usize].1 += 1;
            }
            _ => {}
        }
    }
}

pub fn load_preview(save_data: &SaveData, ctx: &Context) -> Option<TextureHandle> {
    match &save_data.preview {
        Preview::JPEG(data) | Preview::PNG(data) => {
            if let Ok(img) = image::io::Reader::new(Cursor::new(data)).with_guessed_format() {
                if let Ok(img) = img.decode() {
                    let img_size = [img.width() as _, img.height() as _];
                    let img_rgba8 = img.to_rgba8();
                    let img_pixels = img_rgba8.as_flat_samples();
                    let img = ColorImage::from_rgba_unmultiplied(img_size, img_pixels.as_slice());
                    return Some(ctx.load_texture("Save-Preview", img, TextureOptions::NEAREST));
                } else {
                    println!("Couldn't decode image")
                }
            } else {
                println!("Couldn't interpret preview data")
            }
        }
        Preview::None => {
            println!("No preview image to load")
        }
        _ => {
            println!("Preview image in unknown format")
        }
    }
    None
}
