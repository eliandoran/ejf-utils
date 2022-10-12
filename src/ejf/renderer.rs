use viuer::Config;
use core::{cmp::max};
use image::{DynamicImage, ImageBuffer};
use freetype::{Face, Bitmap, face::LoadFlag};

const DEBUG: bool = false;

pub struct RenderConfig {
    pub total_height: u32,
    pub left_spacing: u8,
    pub right_spacing: u8,
    pub max_ascent: u16
}

pub fn get_pixels(bitmap: Bitmap, config: RenderConfig, offset_y: i32) -> DynamicImage {    
    let char_width = bitmap.width() as usize;
    let image_height = config.total_height;
    let image_width = (config.left_spacing as u32) + (char_width as u32) + (config.right_spacing as u32);
    let image_width = max(1, image_width); // 0px width images are not allowed.
    let offset_x = config.left_spacing as i32;
    let mut figure = ImageBuffer::new(image_width, image_height);
    
    for cx in 0..char_width {
        for cy in 0..bitmap.rows() as usize {
            let pixel = [ bitmap.buffer()[cy * char_width + cx] ];
            let dest_x = cx as i32 + offset_x;
            let dest_y = cy as i32 + offset_y;
            if dest_y >= 0 && dest_y < image_height as i32 {
                figure[(dest_x as u32, dest_y as u32)] = image::Luma(pixel);
            }
        }
    }

    let mut image = DynamicImage::ImageLuma8(figure);
    image.invert();
    image 
}

pub fn render_single_character(face: &Face, ch: char, config: RenderConfig) -> DynamicImage {    
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();
    let metrics = glyph.metrics();
    let left_bearing = (metrics.horiBearingX >> 6) as i32;
    let right_bearing =
        (metrics.horiAdvance - metrics.horiBearingX - metrics.width) >> 6;

    let left_spacing = max(0, left_bearing) as u8;
    let right_spacing = max(0, right_bearing) as u8;

    if DEBUG {
        println!("{} -> leftBearing={}, rightBearing={}, spacing=({}, {})", ch, left_bearing, right_bearing, left_spacing, right_spacing);
    }

    // Get the pixels of that single character.
    let offset_y = config.max_ascent as i32 - (glyph.bitmap_top() as i32);
    let img = get_pixels(glyph.bitmap(), RenderConfig {
        left_spacing,
        right_spacing,
        max_ascent: config.max_ascent,
        total_height: config.total_height
    }, offset_y);    

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