use viuer::Config;
use core::{cmp::max};
use image::{DynamicImage, ImageBuffer};
use freetype::{Face, Bitmap, face::LoadFlag};

pub fn get_pixels(bitmap: Bitmap, image_height: u32, offset_y: i32) -> DynamicImage {    
    let width = bitmap.width() as usize;
    let image_width = max(1, width as u32); // 0px width images are not allowed.
    let mut figure = ImageBuffer::new(image_width, image_height);
    
    for cx in 0..width {
        for cy in 0..bitmap.rows() as usize {
            let pos = (cy) * width + (cx);
            let pixel = [ bitmap.buffer()[pos] ];

            let dest_y = cy as i32 + offset_y;
            if dest_y >= 0 && dest_y < image_height as i32 {
                figure[(cx as u32, dest_y as u32)] = image::Luma(pixel);
            }
        }
    }

    let mut image = DynamicImage::ImageLuma8(figure);
    image.invert();
    image 
}

pub fn render_single_character(face: &Face, ch: char, image_height: u32, max_ascent: u16) -> DynamicImage {
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();

    // Get the pixels of that single character.
    let offset_y = max_ascent as i32 - (glyph.bitmap_top() as i32);
    let img = get_pixels(glyph.bitmap(), image_height, offset_y);    

    img
}

pub fn print_character(img: &DynamicImage) {
    let config = Config {
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(&img, &config)
        .expect("Image printing failed.");
}