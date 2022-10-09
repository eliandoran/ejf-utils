use freetype::{Library, Face, face::LoadFlag, Bitmap};
use image::{DynamicImage, ImageBuffer, ImageFormat};
use viuer::Config;
use core::cmp::max;

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 8 * 64;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;

const SKIP_CONTROL_CHARACTERS: bool = true;
const PRINT_CHARACTERS: bool = false;

fn get_pixels(bitmap: Bitmap, image_height: u32, offset_y: i32) -> DynamicImage {    
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

fn render_single_character(face: &Face, ch: char, image_height: u32, max_ascent: u32) {
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();
    let x = glyph.bitmap_left() as i32;
    let y = glyph.bitmap_top() as i32;
    
    println!("Bitmap position: char='{}', x={}, y={}", ch, x, y);

    // Get the pixels of that single character.
    let offset_y = max_ascent as i32 - (glyph.bitmap_top() as i32);
    let img = get_pixels(glyph.bitmap(), image_height, offset_y);    

    // Save the output to png.
    let filename = format!("output/0x{:x}.png", ch as u32);
    img.save_with_format(filename, ImageFormat::Png).unwrap();

    if PRINT_CHARACTERS {
        let config = Config {
            absolute_offset: false,
            ..Default::default()
        };

        viuer::print(&img, &config)
            .expect("Image printing failed.");
    }
}

fn main() {
    // Try to open the font.
    let library = Library::init().unwrap();
    let face: Face = library
        .new_face(FONT_PATH, 0)
        .expect("Unable to load the font file to be used for rendering.\nPlease check the path to the font, permissions or that the file format is supported.");    

    // Set face properties.
    face.set_char_size(FACE_CHAR_WIDTH, 0, FACE_HORIZONTAL_RESOLUTION, 0)
        .expect("Unable to set the character size.");

    // Determine ascent, descent, image height
    let y_scale = face.size_metrics().unwrap().y_scale as f32;
    let max_ascent = (face.ascender() as f32 * (y_scale / 65536.0)) as u32 >> 6;
    
    // Determine max height.
    let mut max_descent: i32 = 0;
    for code in (0 as u8)..(255 as u8) {
        face.load_char(code as usize, LoadFlag::RENDER)
            .expect("Unable to load character.");
        let glyph = face.glyph();
        let cur_descent = (glyph.metrics().height >> 6) as i32 - glyph.bitmap_top();

        if cur_descent > max_descent {
            max_descent = cur_descent;
        }
    }

    println!("{}", max_descent);
    let image_height = (max_ascent as i32 + max_descent) as u32;

    println!("Font family: {}", face.family_name().unwrap());

    // Render the characters.
    for code in (0 as u8)..(255 as u8) {
        let ch = code as char;

        if SKIP_CONTROL_CHARACTERS && ch.is_control() {
            continue;
        }

        render_single_character(&face, ch as char, image_height, max_ascent);
    }    
}

