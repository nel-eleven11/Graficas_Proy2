// texture.rs

use image::{ImageReader, /*DynamicImage,*/ GenericImageView};
use std::fmt;
use crate::color::Color;
use image::Pixel;

#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    color_array: Vec<Color>,
}

impl Texture {
    pub fn new(file_path: &str) -> Self {
        // Attempt to load the image file
        let img = ImageReader::open(file_path)
            .expect("Failed to open texture file") // Panic if the file is not accessible
            .decode()
            .expect("Failed to decode texture file"); // Panic if the image format is invalid

        let width = img.width() as usize;
        let height = img.height() as usize;

        let mut color_array = vec![Color::black(); width * height];
        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x as u32, y as u32).to_rgb();
                let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                color_array[y * width + x] = Color::from_hex(color);
            }
        }

        Texture {
            width,
            height,
            color_array,
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            Color::from_hex(0xFF00FF) // Debug color for out-of-bounds access
        } else {
            self.color_array[y * self.width + x]
        }
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
