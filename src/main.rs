use freetype::{Library, Face, face::LoadFlag, Bitmap};
use image::{DynamicImage, ImageBuffer, ImageFormat};
use viuer::Config;

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 8 * 64;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;

const SKIP_CONTROL_CHARACTERS: bool = true;
const PRINT_CHARACTERS: bool = false;

fn get_pixels(bitmap: Bitmap, size: (u32, u32)) -> DynamicImage {    
    let mut figure = ImageBuffer::new(size.0, size.1);
    let width = bitmap.width() as usize;
    
    for cx in 0..width {
        for cy in 0..bitmap.rows() as usize {
            let pos = (cy) * width + (cx);
            let pixel = [ bitmap.buffer()[pos] ];
            figure[(cx as u32, cy as u32)] = image::Luma(pixel);
        }
    }

    let mut image = DynamicImage::ImageLuma8(figure);
    image.invert();
    image
}

fn render_single_character(face: &Face, ch: char, size: (u32, u32)) {
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();
    let x = glyph.bitmap_left() as i32;
    let y = glyph.bitmap_top() as i32;
    
    println!("Bitmap position: char='{}', x={}, y={}", ch, x, y);

    // Get the pixels of that single character.
    let img = get_pixels(glyph.bitmap(), size);    

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

    // Determine max height.
    let mut max_height = 0;
    let mut max_width: u32 = 0;
    for code in (0 as u8)..(255 as u8) {
        face.load_char(code as usize, LoadFlag::RENDER)
            .expect("Unable to load character.");
        let bitmap = face.glyph().bitmap();
        let cur_width = bitmap.width() as u32;
        let cur_height = bitmap.rows() as u32;
        if cur_width > max_width {
            max_width = cur_width;
        }
        if cur_height > max_height {
            max_height = cur_height;
        }        
    }

    println!("Font family: {}", face.family_name().unwrap());
    println!("Image dimensions: width={}px, height={}px", max_width, max_height);

    // Render the characters.
    for code in (0 as u8)..(255 as u8) {
        let ch = code as char;

        if SKIP_CONTROL_CHARACTERS && ch.is_control() {
            continue;
        }

        render_single_character(&face, ch as char, (max_width, max_height));
    }    
}

