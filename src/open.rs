use std::io::Cursor;

use brickadia::save::{Preview, SaveData};
use egui::{ColorImage, Context, TextureFilter, TextureHandle};

pub fn load_preview(save_data: &SaveData, ctx: &Context) -> Option<TextureHandle> {
    match &save_data.preview {
        Preview::JPEG(data) | Preview::PNG(data) => {
            if let Ok(img) = image::io::Reader::new(Cursor::new(data)).with_guessed_format() {
                if let Ok(img) = img.decode() {
                    let img_size = [img.width() as _, img.height() as _];
                    let img_rgba8 = img.to_rgba8();
                    let img_pixels = img_rgba8.as_flat_samples();
                    let img = ColorImage::from_rgba_unmultiplied(img_size, img_pixels.as_slice());
                    return Some(ctx.load_texture("Save-Preview", img, TextureFilter::Nearest));
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